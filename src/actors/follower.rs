use super::*;
use rand::{Rng, RngCore};

pub fn poll(node: &mut Node, follower: Follower, mut rng: &mut RngCore) -> (Role, Vec<Message>) {
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
        Heartbeat(heartbeat) => {
            if heartbeat.term < node.term {
                return (follower.into(), vec![]);
            }

            if node.term > heartbeat.term {
                node.term = heartbeat.term;
                node.last_activity = node.time;
                follower.leader_id = heartbeat.leader_id;
                return (follower.into(), vec![]);
            }

            if follower.leader_id != heartbeat.leader_id {
                let idler = Idler { vote: None };
                return (idler.into(), vec![]);
            }

            node.last_activity = node.time;
            (follower.into(), vec![])
        }
        Candidacy(candidacy) => {
            if candidacy.term <= node.term {
                return (follower.into(), vec![]);
            }

            node.term = candidacy.term;
            node.last_activity = node.time;
            let idler = Idler {
                vote: Some(candidacy.candidate_id.clone()),
            };
            let out_msg = message::Vote {
                voter_id: node.id.clone(),
                term: node.term,
                candidate: candidacy.candidate_id,
            }.into();
            (idler.into(), vec![out_msg])
        }
        Vote(..) => (follower.into(), vec![]),
    }
}
