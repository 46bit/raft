use super::*;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: Id,
    pub time: Time,
    pub last_activity: Time,
    pub term: Term,
    pub peers: HashSet<Id>,
}

impl Node {
    pub fn log_prefix(&self) -> String {
        format!("[time {}] [id {}] [term {}]", self.time, self.id, self.term)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Idler(Idler),
    Follower(Follower),
    Candidate(Candidate),
    Leader(Leader),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Idler {
    pub vote: Option<Id>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Follower {
    pub leader_id: Id,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub votes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Leader {}
