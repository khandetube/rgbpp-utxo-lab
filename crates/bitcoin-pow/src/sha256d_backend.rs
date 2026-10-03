use crate::double_sha256;

/// A fixed-size Bitcoin block-header mining request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeaderWork {
    pub header: [u8; 80],
    pub start_nonce: u32,
    pub nonce_stride: u32,
}

impl HeaderWork {
    pub fn with_nonce_range(header: [u8; 80], start_nonce: u32, nonce_stride: u32) -> Self {
        assert!(nonce_stride > 0, "nonce stride must be non-zero");
        Self { header, start_nonce, nonce_stride }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashResult {
    pub nonce: u32,
    pub hash: [u8; 32],
}

/// Backend-independent contract shared by CPU, OpenCL, and future CUDA workers.
pub trait Sha256dBackend {
    fn search(&self, work: HeaderWork, target: &[u8; 32]) -> Option<HashResult>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CpuSha256dBackend;

impl Sha256dBackend for CpuSha256dBackend {
    fn search(&self, work: HeaderWork, target: &[u8; 32]) -> Option<HashResult> {
        let mut header = work.header;
        let mut nonce = work.start_nonce;
        loop {
            header[76..80].copy_from_slice(&nonce.to_le_bytes());
            let hash = double_sha256(&header);
            if displayed_hash_leq_target(&hash, target) {
                return Some(HashResult { nonce, hash });
            }
            match nonce.checked_add(work.nonce_stride) {
                Some(next) => nonce = next,
                None => return None,
            }
        }
    }
}

/// Bitcoin displays a SHA256d digest in reverse byte order. Compare that
/// displayed value numerically against the compact-target-expanded value.
pub fn displayed_hash_leq_target(hash: &[u8; 32], target: &[u8; 32]) -> bool {
    hash.iter().rev().cmp(target.iter()) != std::cmp::Ordering::Greater
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_comparison_uses_displayed_hash_order() {
        let mut hash = [0u8; 32];
        hash[31] = 1;
        let target = [0u8; 31].into_iter().chain([1u8]).collect::<Vec<_>>();
        let target: [u8; 32] = target.try_into().unwrap();
        assert!(displayed_hash_leq_target(&hash, &target));
        hash[31] = 2;
        assert!(!displayed_hash_leq_target(&hash, &target));
    }

    #[test]
    fn cpu_backend_respects_nonce_start_and_stride() {
        let backend = CpuSha256dBackend;
        let mut header = [0u8; 80];
        header[0] = 1;
        let target = [0xffu8; 32];
        let result = backend.search(HeaderWork::with_nonce_range(header, 7, 3), &target).unwrap();
        assert_eq!(result.nonce, 7);
    }
}
