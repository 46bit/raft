use super::*;
use rand::{Rng, RngCore};

pub fn poll_election_timeout(
    node: &mut Node,
    mut rng: &mut RngCore,
) -> Option<(Role, Vec<Message>)> {
    let election_timeout = rng.gen_range(ELECTION_TIMEOUT, ELECTION_TIMEOUT * 2);
    if node.time > node.last_activity + election_timeout {
        println!("{} hit election timeout", node.log_prefix());
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

pub fn vote_for_later_candidate(
    node: &mut Node,
    candidacy: message::Candidacy,
) -> (Role, Vec<Message>) {
    println!(
        "{} voted for candidate {}",
        node.log_prefix(),
        candidacy.candidate_id
    );
    node.term = candidacy.term;
    node.last_activity = node.time;
    let idler = Idler {
        vote: Some(candidacy.candidate_id.clone()),
    };
    let out_msg = message::Vote {
        voter_id: node.id.clone(),
        term: candidacy.term,
        candidate: candidacy.candidate_id,
    }.into();
    (idler.into(), vec![out_msg])
}

pub fn follow_leader(node: &mut Node, heartbeat: message::Heartbeat) -> (Role, Vec<Message>) {
    println!(
        "{} heartbeated by newer-term leader {}",
        node.log_prefix(),
        heartbeat.leader_id
    );
    node.term = heartbeat.term;
    node.last_activity = node.time;
    let follower = Follower {
        leader_id: heartbeat.leader_id,
    };
    (follower.into(), vec![])
}

pub fn go_into_idle() -> (Role, Vec<Message>) {
    let idler = Idler { vote: None };
    (idler.into(), vec![])
}
