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
    pub fn log(&self, msg: &str) {
        println!("{} {}", self.log_prefix(), msg);
    }

    pub fn log_prefix(&self) -> String {
        format!(
            "[time {}] [id {:?}] [term {}]",
            self.time, self.id, self.term
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Idler,
    Voter(Voter),
    Follower(Follower),
    Candidate(Candidate),
    Leader(Leader),
}

impl From<Voter> for Role {
    fn from(voter: Voter) -> Role {
        Role::Voter(voter)
    }
}

impl From<Follower> for Role {
    fn from(follower: Follower) -> Role {
        Role::Follower(follower)
    }
}

impl From<Candidate> for Role {
    fn from(candidate: Candidate) -> Role {
        Role::Candidate(candidate)
    }
}

impl From<Leader> for Role {
    fn from(leader: Leader) -> Role {
        Role::Leader(leader)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Voter {
    pub candidate_id: Id,
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
