use super::*;
use rand::{Rng, RngCore};

pub fn poll(node: &mut Node, follower: Follower, mut rng: &mut RngCore) -> (Role, Vec<Message>) {
    let election_timeout = rng.gen_range(ELECTION_TIMEOUT, ELECTION_TIMEOUT * 2);
    if node.time > node.last_activity + election_timeout {
        node.term += 1;
        let candidate = Candidate { votes: 1 };
        let out_msg = message::Candidacy { term: node.term }.into_message(node.id.clone());
        return (candidate.into(), vec![out_msg]);
    }

    (follower.into(), vec![])
}

pub fn process_msg(
    in_msg: Message,
    node: &mut Node,
    mut follower: Follower,
    _: &mut RngCore,
) -> (Role, Vec<Message>) {
    use Message::*;
    match in_msg {
        Heartbeat(heartbeat_leader_id, heartbeat) => {
            if heartbeat.term < node.term {
                return (follower.into(), vec![]);
            }

            if node.term > heartbeat.term {
                node.term = heartbeat.term;
                node.last_activity = node.time;
                follower.leader_id = heartbeat_leader_id;
                return (follower.into(), vec![]);
            }

            if follower.leader_id == heartbeat_leader_id {
                node.last_activity = node.time;
                return (follower.into(), vec![]);
            }

            let idler = Idler { vote: None };
            (idler.into(), vec![])
        }
        Candidacy(candidate_id, candidacy) => {
            if candidacy.term <= node.term {
                return (follower.into(), vec![]);
            }

            node.term = candidacy.term;
            let idler = Idler {
                vote: Some(candidate_id.clone()),
            };
            let msg = message::Vote {
                term: node.term,
                candidate: candidate_id,
            }.into_message(node.id.clone());
            (idler.into(), vec![msg])
        }
        Vote(..) => (follower.into(), vec![]),
    }
}
