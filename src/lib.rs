#![feature(box_syntax)]

extern crate rand;

mod node;
pub use node::*;
mod message;
pub use message::*;
mod actors;
pub use actors::*;

pub type Id = u64;

pub type Time = u64;

pub type Term = u64;
