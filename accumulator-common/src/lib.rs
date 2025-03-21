#![deny(
// warnings,
missing_docs,
unsafe_code,
unused_import_braces,
unused_lifetimes,
unused_qualifications,
)]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
//! This crate is only meant to be used internally across
//! accumulator types.

/// Common macros
pub mod macros;
/// Multiprecision Big Integer Implementation
pub mod bigint;
/// Accumulator errors that can be thrown
pub mod error;

#[cfg(not(any(feature = "bi-rust", feature = "bi-gmp")))]
compile_error!("A big number library must be chosen: either bi-rust or bi-gmp");
#[cfg(all(feature = "bi-rust", feature = "bi-gmp"))]
compile_error!("Only one big number library must be chosen: either bi-rust or bi-gmp");

use bigint::BigInteger;

/// Helper class that always reduces operations by a modulus
#[derive(Debug)]
pub struct Field {
    modulus: BigInteger
}

impl Field {
    /// Construct a new field
    pub fn new(modulus: &BigInteger) -> Self {
        Self { modulus: modulus.clone() }
    }

    /// b^e mod r
    pub fn exp(&self, base: &BigInteger, exp: &BigInteger) -> BigInteger {
        base.mod_exp(exp, &self.modulus)
    }

    /// (a * b) mod r
    pub fn mul(&self, a: &BigInteger, b: &BigInteger) -> BigInteger {
        a.mod_mul(&b, &self.modulus)
    }

    /// a^-1 mod r
    pub fn inv(&self, a: &BigInteger) -> BigInteger { a.mod_inverse(&self.modulus) }
}