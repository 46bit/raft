#[cfg(test)]
extern crate quickcheck;
extern crate rand;

mod node;
mod message;
mod actor;

pub use node::*;
pub use message::*;
pub use actor::*;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

pub type Id = u64;

pub type Time = u64;

pub type Term = u64;
