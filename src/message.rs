use super::*;
use std::collections::VecDeque;

pub type MessageIterator = Iterator<Item = Message>;

pub type MessageQueue = VecDeque<Message>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Heartbeat(HeartbeatMessage),
    Candidacy(CandidacyMessage),
    Vote(VoteMessage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeartbeatMessage {
    pub term: Term,
    pub from: Id,
    pub nodes: Vec<Id>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidacyMessage {
    pub term: Term,
    pub candidate: Id,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoteMessage {
    pub term: Term,
    pub candidate: Id,
    pub elector: Id,
}
