use super::*;
use rand::{Rng, RngCore};

pub fn poll(node: &mut Node, idler: Idler, mut rng: &mut RngCore) -> (Role, Vec<Message>) {
    let election_timeout = rng.gen_range(ELECTION_TIMEOUT, ELECTION_TIMEOUT * 2);
    if node.time < node.last_activity + election_timeout {
        let role = Role::Idler(idler);
        (role, vec![])
    } else {
        node.term += 1;
        let candidate = Candidate { votes: 1 };
        let out_msg = message::Candidacy { term: node.term }.into_message(node.id.clone());
        (Role::Candidate(candidate), vec![out_msg])
    }
}

pub fn process_msg(
    in_msg: Message,
    node: &mut Node,
    mut idler: Idler,
    _: &mut RngCore,
) -> (Role, Vec<Message>) {
    use Message::*;
    match in_msg {
        Heartbeat(leader_id, heartbeat) => {
            if heartbeat.term < node.term {
                return (Role::Idler(idler), vec![]);
            }

            node.term = heartbeat.term;
            node.last_activity = node.time;
            let follower = Follower { leader_id };
            (Role::Follower(follower), vec![])
        }
        Candidacy(candidate_id, candidacy) => {
            if candidacy.term < node.term {
                return (Role::Idler(idler), vec![]);
            }

            if idler.vote.is_none() {
                node.term = candidacy.term;
                idler.vote = Some(candidate_id.clone());
                let role = Role::Idler(idler);
                let out_msg = message::Vote {
                    term: node.term,
                    candidate: candidate_id,
                }.into_message(node.id.clone());
                (role, vec![out_msg])
            } else {
                (Role::Idler(idler), vec![])
            }
        }
        Vote(..) => (Role::Idler(idler), vec![]),
    }
}
