# Serde, Borsh, Lifetimes in Rust

### Serde
- General-purpose framework for (de)serialization.
- Supports tons of formats: JSON, YAML, TOML, MessagePack, CBOR, even custom ones.

### Borsh (Binary Object Representation Serializer for Hashing)
- Built specifically for blockchains (by NEAR, used in Solana too).
- Goal: deterministic, compact, binary format.

### Lifetimes
- Lifetimes track how long references live – they don’t change runtime behavior, just let the compiler prove safety.
- Written as <'a> – 'a is just a label the compiler uses to relate references.
- Every reference has a lifetime, even if you don’t write it. Most of the time, Rust can infer it.
- Functions with references often need explicit lifetimes when multiple borrows are involved
- Structs with references must declare lifetimes
- Static lifetime ('static) = the reference lives for the whole program (like string literals).