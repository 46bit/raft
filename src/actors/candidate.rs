use super::*;
use rand::RngCore;

pub fn poll(node: &mut Node, candidate: Candidate, rng: &mut RngCore) -> (Role, Vec<Message>) {
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

    if let Some(result) = poll_election_timeout(node, rng) {
        return result;
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
            if heartbeat.term < node.term {
                return (candidate.into(), vec![]);
            }

            follow_leader(node, heartbeat)
        }
        Candidacy(candidacy) => {
            if candidacy.term <= node.term {
                return (candidate.into(), vec![]);
            }

            vote_for_later_candidate(node, candidacy)
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
