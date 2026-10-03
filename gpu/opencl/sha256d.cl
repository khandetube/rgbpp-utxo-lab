// Standalone OpenCL SHA256d reference kernel.
// Input: one 80-byte Bitcoin block-header template per work item.
// The kernel writes the 32-byte SHA256d digest and the nonce used.
// It deliberately has no wallet, Stratum, or network dependencies.

typedef unsigned int u32;
typedef unsigned char u8;
#define ROTR(x,n) ((x >> n) | (x << (32-n)))
#define CH(x,y,z) ((x & y) ^ (~x & z))
#define MAJ(x,y,z) ((x & y) ^ (x & z) ^ (y & z))
#define BS0(x) (ROTR(x,2) ^ ROTR(x,13) ^ ROTR(x,22))
#define BS1(x) (ROTR(x,6) ^ ROTR(x,11) ^ ROTR(x,25))
#define SS0(x) (ROTR(x,7) ^ ROTR(x,18) ^ (x >> 3))
#define SS1(x) (ROTR(x,17) ^ ROTR(x,19) ^ (x >> 10))

__constant u32 K[64] = {
0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e0,0x92722c85,
0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2
};

inline u32 load_be(const u8 *p) { return ((u32)p[0]<<24)|((u32)p[1]<<16)|((u32)p[2]<<8)|p[3]; }
inline void store_be(u8 *p,u32 x) { p[0]=x>>24;p[1]=x>>16;p[2]=x>>8;p[3]=x; }

inline void sha256_compress(const u8 *block,u32 *s) {
  u32 w[64];
  for(int i=0;i<16;i++) w[i]=load_be(block+4*i);
  for(int i=16;i<64;i++) w[i]=SS1(w[i-2])+w[i-7]+SS0(w[i-15])+w[i-16];
  u32 a=s[0],b=s[1],c=s[2],d=s[3],e=s[4],f=s[5],g=s[6],h=s[7];
  for(int i=0;i<64;i++){u32 t1=h+BS1(e)+CH(e,f,g)+K[i]+w[i],t2=BS0(a)+MAJ(a,b,c);h=g;g=f;f=e;e=d+t1;d=c;c=b;b=a;a=t1+t2;}
  s[0]+=a;s[1]+=b;s[2]+=c;s[3]+=d;s[4]+=e;s[5]+=f;s[6]+=g;s[7]+=h;
}

__kernel void sha256d_header(__global const u8 *header_template,__global u8 *digests,__global u32 *nonces,u32 nonce_base) {
  const size_t gid=get_global_id(0); u8 first[80];
  for(int i=0;i<80;i++) first[i]=header_template[i];
  const u32 nonce=nonce_base+(u32)gid;
  first[76]=(u8)nonce;first[77]=(u8)(nonce>>8);first[78]=(u8)(nonce>>16);first[79]=(u8)(nonce>>24);
  u32 s1[8]={0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19};
  u8 block[64]; for(int i=0;i<64;i++) block[i]=first[i]; sha256_compress(block,s1);
  for(int i=0;i<16;i++) block[i]=first[64+i]; block[16]=0x80; for(int i=17;i<56;i++) block[i]=0;
  for(int i=0;i<8;i++) block[56+i]=(u8)(640>>(56-8*i)); sha256_compress(block,s1);
  u8 d1[32]; for(int i=0;i<8;i++) store_be(d1+4*i,s1[i]);
  u32 s2[8]={0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19};
  for(int i=0;i<32;i++) block[i]=d1[i]; block[32]=0x80; for(int i=33;i<56;i++) block[i]=0;
  for(int i=0;i<8;i++) block[56+i]=(u8)(256>>(56-8*i)); sha256_compress(block,s2);
  for(int i=0;i<8;i++) store_be(digests[gid*32+4*i],s2[i]); nonces[gid]=nonce;
}
