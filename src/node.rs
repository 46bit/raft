use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: Id,
    pub time: Time,
    pub term: Term,
    pub peers: Vec<Id>,
}

impl Node {
    pub fn log_prefix(&self) -> String {
        format!("[time {}] [id {}] [term {}]", self.time, self.id, self.term)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Follower(Follower),
    Candidate(Candidate),
    Leader(Leader),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Follower {
    pub last_recv_heartbeat: Time,
    pub voted: Option<Id>,
}

impl Follower {
    pub fn into_role(self) -> Role {
        Role::Follower(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub votes: u64,
}

impl Candidate {
    pub fn into_role(self) -> Role {
        Role::Candidate(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Leader {
    pub last_sent_heartbeat: Time,
}

impl Leader {
    pub fn into_role(self) -> Role {
        Role::Leader(self)
    }
}
