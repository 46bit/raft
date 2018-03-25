use super::*;
use rand::RngCore;

pub fn poll(node: &mut Node, idler: Idler, rng: &mut RngCore) -> (Role, Vec<Message>) {
    if let Some(result) = poll_election_timeout(node, rng) {
        return result;
    }

    (idler.into(), vec![])
}

pub fn process_msg(
    in_msg: Message,
    node: &mut Node,
    mut idler: Idler,
    _: &mut RngCore,
) -> (Role, Vec<Message>) {
    use Message::*;
    match in_msg {
        Heartbeat(heartbeat) => {
            if heartbeat.term < node.term {
                return (idler.into(), vec![]);
            }

            follow_leader(node, heartbeat)
        }
        Candidacy(candidacy) => {
            if candidacy.term < node.term {
                return (idler.into(), vec![]);
            }

            if idler.vote.is_none() {
                node.term = candidacy.term;
                node.last_activity = node.time;
                idler.vote = Some(candidacy.candidate_id.clone());
                let out_msg = message::Vote {
                    voter_id: node.id.clone(),
                    term: node.term,
                    candidate: candidacy.candidate_id,
                }.into();
                return (idler.into(), vec![out_msg]);
            }

            (idler.into(), vec![])
        }
        Vote(..) => (idler.into(), vec![]),
    }
}
