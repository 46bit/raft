use super::*;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Heartbeat(Heartbeat),
    Candidacy(Candidacy),
    Vote(Vote),
}

impl From<Heartbeat> for Message {
    fn from(heartbeat: Heartbeat) -> Message {
        Message::Heartbeat(heartbeat)
    }
}

impl From<Candidacy> for Message {
    fn from(candidacy: Candidacy) -> Message {
        Message::Candidacy(candidacy)
    }
}

impl From<Vote> for Message {
    fn from(vote: Vote) -> Message {
        Message::Vote(vote)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heartbeat {
    pub leader_id: Id,
    pub term: Term,
    pub nodes: HashSet<Id>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidacy {
    pub candidate_id: Id,
    pub term: Term,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vote {
    pub voter_id: Id,
    pub term: Term,
    pub candidate: Id,
}
