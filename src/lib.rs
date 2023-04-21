pub mod keygen;
pub mod core;
pub mod traits;
pub mod serialize;

pub use keygen::*;
pub use crate::core::*;
pub use traits::*;
pub use serialize::*;

pub use curv::arithmetic::traits::*;
pub use curv::arithmetic::BigInt;

use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SHE;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyGenParam {
    pub k0: usize,
    pub k1: usize,
    pub k2: usize,
    pub p: BigInt,
    pub q: BigInt,
    pub L: BigInt,
    pub N: BigInt,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriKey {
    pub p: BigInt,
    pub q: BigInt,
    pub L: BigInt,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PubParam {
    pub k0: usize,
    pub k1: usize,
    pub k2: usize,
    pub N: BigInt,
}