use super::*;
use rand::{Rng, RngCore};

pub fn poll(node: &mut Node, follower: Follower, mut rng: &mut RngCore) -> (Role, Vec<Message>) {
    let election_timeout = rng.gen_range(ELECTION_TIMEOUT, ELECTION_TIMEOUT * 2);
    if node.time > node.last_activity + election_timeout {
        node.term += 1;
        let candidate = Candidate { votes: 1 };
        let out_msg = message::Candidacy { term: node.term }.into_message(node.id.clone());
        return (Role::Candidate(candidate), vec![out_msg]);
    }

    (Role::Follower(follower), vec![])
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
                return (Role::Follower(follower), vec![]);
            }

            if node.term > heartbeat.term {
                node.term = heartbeat.term;
                node.last_activity = node.time;
                follower.leader_id = heartbeat_leader_id;
                return (Role::Follower(follower), vec![]);
            }

            if follower.leader_id == heartbeat_leader_id {
                node.last_activity = node.time;
                return (Role::Follower(follower), vec![]);
            }

            let idler = Idler { vote: None };
            (Role::Idler(idler), vec![])
        }
        Candidacy(candidate_id, candidacy) => {
            if candidacy.term <= node.term {
                return (Role::Follower(follower), vec![]);
            }

            node.term = candidacy.term;
            let idler = Idler {
                vote: Some(candidate_id.clone()),
            };
            let msg = message::Vote {
                term: node.term,
                candidate: candidate_id,
            }.into_message(node.id.clone());
            (Role::Idler(idler), vec![msg])
        }
        Vote(..) => (Role::Follower(follower), vec![]),
    }
}
