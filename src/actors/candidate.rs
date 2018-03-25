use super::*;
use rand::RngCore;

pub fn poll(node: &mut Node, candidate: Candidate, _: &mut RngCore) -> (Role, Vec<Message>) {
    let necessary_votes = (node.peers.len() / 2) as u64;
    if candidate.votes > necessary_votes {
        let leader = Leader {};
        // FIXME: Send heartbeat (node.last_activity hasn't been reset)
        return (leader.into(), vec![]);
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
        Heartbeat(leader_id, heartbeat) => {
            if heartbeat.term >= node.term {
                let follower = Follower { leader_id };
                return (follower.into(), vec![]);
            }

            (candidate.into(), vec![])
        }
        Candidacy(other_candidate_id, candidacy) => {
            let other_candidate_is_later_term = candidacy.term > node.term;
            if other_candidate_is_later_term {
                node.term = candidacy.term;
                let idler = Idler {
                    vote: Some(other_candidate_id.clone()),
                };
                let msg = message::Vote {
                    term: candidacy.term,
                    candidate: other_candidate_id,
                }.into_message(node.id.clone());
                return (idler.into(), vec![msg]);
            }

            (candidate.into(), vec![])
        }
        Vote(_, vote) => {
            let was_voted_for = (vote.term, vote.candidate) == (node.term, node.id.clone());
            if was_voted_for {
                candidate.votes += 1;
            }
            (candidate.into(), vec![])
        }
    }
}
