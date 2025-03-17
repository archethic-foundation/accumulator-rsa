# accumulator-rsa
Cryptographic Accumulators in Rust

A cryptographic accumulator allows for testing set membership without revealing which set member was tested. This avoids the need to check every member to see if a value exists and compress into a small value. Provers use a witness that a specific value is or is not in the set and generate a zero-knowledge proof.

There are three constructions for accumulators as referenced [Zero-Knowledge Proofs for Set Membership](https://eprint.iacr.org/2019/1255.pdf)

RSA-Based: Requires groups of unknown order and can be slow to create but offers reasonable performance for witness updates and proof generation and verification. Each element must be prime number and the modulus must be large enough to be secure (≥ 2048-bits). Elements do not have to be know in advance and can be added on-the-fly. Setup parameters include generating prime numbers for the modulus.

# License

 MIT license ([LICENSE.md])
a
# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
