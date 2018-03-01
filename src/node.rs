use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Follower {
    pub id: Id,
    pub time: Time,
    pub term: Term,
    pub nodes: Vec<Id>,
    pub last_recv_heartbeat: Time,
    pub voted: Option<Id>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub id: Id,
    pub time: Time,
    pub term: Term,
    pub nodes: Vec<Id>,
    pub votes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Leader {
    pub id: Id,
    pub time: Time,
    pub term: Term,
    pub nodes: Vec<Id>,
    pub last_sent_heartbeat: Time,
}
