#![feature(box_syntax)]

extern crate rand;

mod node;
pub use node::*;
pub mod message;
pub use message::Message;
mod actors;
pub use actors::*;

pub type Id = String;

pub type Time = u64;

pub type Term = u64;
