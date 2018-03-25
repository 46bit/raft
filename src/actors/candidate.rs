use super::*;
use rand::{Rng, RngCore};

pub fn poll(node: &mut Node, candidate: Candidate, mut rng: &mut RngCore) -> (Role, Vec<Message>) {
    let necessary_votes = (node.peers.len() / 2) as u64;
    if candidate.votes > necessary_votes {
        node.last_activity = node.time;
        let leader = Leader {};
        let out_msg = message::Heartbeat {
            leader_id: node.id.clone(),
            term: node.term,
            nodes: node.peers.clone(),
        }.into();
        return (leader.into(), vec![out_msg]);
    }

    let election_timeout = rng.gen_range(ELECTION_TIMEOUT, ELECTION_TIMEOUT * 2);
    if node.time > node.last_activity + election_timeout {
        node.term += 1;
        node.last_activity = node.time;
        let candidate = Candidate { votes: 1 };
        let out_msg = message::Candidacy {
            candidate_id: node.id.clone(),
            term: node.term,
        }.into();
        return (candidate.into(), vec![out_msg]);
    }

    (candidate.into(), vec![])
}

pub fn process_msg(
    msg: Message,
    node: &mut Node,
    mut candidate: Candidate,
    _: &mut RngCore,
) -> (Role, Vec<Message>) {
    use Message::*;
    match msg {
        Heartbeat(heartbeat) => {
            if heartbeat.term >= node.term {
                node.term = heartbeat.term;
                node.last_activity = node.time;
                let follower = Follower {
                    leader_id: heartbeat.leader_id,
                };
                return (follower.into(), vec![]);
            }

            (candidate.into(), vec![])
        }
        Candidacy(candidacy) => {
            if candidacy.term > node.term {
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
                return (idler.into(), vec![out_msg]);
            }

            (candidate.into(), vec![])
        }
        Vote(vote) => {
            let was_voted_for = (vote.term, vote.candidate) == (node.term, node.id.clone());
            if was_voted_for {
                // FIXME: Should the candidate consider a vote for itself as activity?
                candidate.votes += 1;
            }
            (candidate.into(), vec![])
        }
    }
}
