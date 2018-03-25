use super::*;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Heartbeat(Id, Heartbeat),
    Candidacy(Id, Candidacy),
    Vote(Id, Vote),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heartbeat {
    pub term: Term,
    pub nodes: HashSet<Id>,
}

impl Heartbeat {
    pub fn into_message(self, node_id: Id) -> Message {
        Message::Heartbeat(node_id, self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidacy {
    pub term: Term,
}

impl Candidacy {
    pub fn into_message(self, node_id: Id) -> Message {
        Message::Candidacy(node_id, self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vote {
    pub term: Term,
    pub candidate: Id,
}

impl Vote {
    pub fn into_message(self, node_id: Id) -> Message {
        Message::Vote(node_id, self)
    }
}
