use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Follower(FollowerNode),
    Candidate(CandidateNode),
    Leader(LeaderNode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FollowerNode {
    pub id: Id,
    pub time: Time,
    pub term: Term,
    pub nodes: Vec<Id>,
    pub last_recv_heartbeat: Time,
    pub voted: Option<Id>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateNode {
    pub id: Id,
    pub time: Time,
    pub term: Term,
    pub nodes: Vec<Id>,
    pub votes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaderNode {
    pub id: Id,
    pub time: Time,
    pub term: Term,
    pub nodes: Vec<Id>,
    pub last_sent_heartbeat: Time,
}
