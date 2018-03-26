use super::*;
use rand::{Rng, RngCore};

pub fn poll_election_timeout(
    node: &mut Node,
    mut rng: &mut RngCore,
) -> Option<(Role, Vec<Message>)> {
    let election_timeout = rng.gen_range(ELECTION_TIMEOUT, ELECTION_TIMEOUT * 2);
    if node.time > node.last_activity + election_timeout {
        node.term += 1;
        node.last_activity = node.time;
        let candidate = Candidate { votes: 1 };
        let out_msg = message::Candidacy {
            candidate_id: node.id.clone(),
            term: node.term,
        }.into();
        return Some((candidate.into(), vec![out_msg]));
    }

    None
}

pub fn voter_for(node: &mut Node, candidacy: message::Candidacy) -> (Role, Vec<Message>) {
    node.term = candidacy.term;
    node.last_activity = node.time;
    let voter = Voter {
        candidate_id: candidacy.candidate_id.clone(),
    };
    let out_msg = message::Vote {
        voter_id: node.id.clone(),
        term: candidacy.term,
        candidate: candidacy.candidate_id,
    }.into();
    (voter.into(), vec![out_msg])
}

pub fn follower_of(node: &mut Node, heartbeat: message::Heartbeat) -> (Role, Vec<Message>) {
    node.term = heartbeat.term;
    node.last_activity = node.time;
    let follower = Follower {
        leader_id: heartbeat.leader_id,
    };
    (follower.into(), vec![])
}

pub fn heartbeat(node: &mut Node, leader: Leader) -> (Role, Vec<Message>) {
    node.last_activity = node.time;
    let out_msg = message::Heartbeat {
        leader_id: node.id.clone(),
        term: node.term,
        nodes: node.peers.clone(),
    }.into();
    (leader.into(), vec![out_msg])
}
