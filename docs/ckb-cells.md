# CKB Cell Model

The project models a CKB Cell with capacity, lock script, optional type script, and data.

The first invariant implemented here is capacity conservation between transaction inputs and outputs.

This is a domain model, not a replacement for official CKB transaction types. Live transaction construction will use the official SDK rather than duplicating serialization rules.
