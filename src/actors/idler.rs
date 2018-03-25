use super::*;
use rand::{Rng, RngCore};

pub fn poll(node: &mut Node, idler: Idler, mut rng: &mut RngCore) -> (Role, Vec<Message>) {
    let election_timeout = rng.gen_range(ELECTION_TIMEOUT, ELECTION_TIMEOUT * 2);
    if node.time < node.last_activity + election_timeout {
        (idler.into(), vec![])
    } else {
        node.term += 1;
        let candidate = Candidate { votes: 1 };
        let out_msg = message::Candidacy {
            candidate_id: node.id.clone(),
            term: node.term,
        }.into();
        (candidate.into(), vec![out_msg])
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
        Heartbeat(heartbeat) => {
            if heartbeat.term < node.term {
                return (idler.into(), vec![]);
            }

            node.term = heartbeat.term;
            node.last_activity = node.time;
            let follower = Follower {
                leader_id: heartbeat.leader_id,
            };
            (follower.into(), vec![])
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
