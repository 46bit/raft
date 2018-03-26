#![feature(box_syntax)]

extern crate rand;

mod node;
pub use node::*;
pub mod message;
pub use message::Message;
mod roles;
use roles::*;
mod actor;
pub use actor::*;

pub type Id = String;

pub type Time = u64;

pub type Term = u64;

// How long to wait for a master's heartbeat before starting an election.
pub const ELECTION_TIMEOUT: Time = 150;
// How long between sending heartbeats
pub const HEARTBEAT_PERIOD: Time = 15;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {}
