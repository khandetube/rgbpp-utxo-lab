# Security Notes

This repository is an educational and portfolio implementation, not production-ready wallet software.

- Never commit private keys, seed phrases, mnemonics, or API secrets.
- Examples use synthetic identifiers and values.
- Network integration must validate chain and network configuration before signing.
- Transaction builders must fail closed on arithmetic overflow and insufficient funds.
- Protocol-specific behavior must be derived from upstream specifications or source code rather than inferred.
