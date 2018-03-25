use super::*;
use rand::RngCore;

pub fn poll(node: &mut Node, leader: Leader, _: &mut RngCore) -> (Role, Vec<Message>) {
    let time_for_heartbeat = node.time >= node.last_activity + HEARTBEAT_PERIOD;
    if time_for_heartbeat {
        node.last_activity = node.time;
        let out_msg = message::Heartbeat {
            leader_id: node.id.clone(),
            term: node.term,
            nodes: node.peers.clone(),
        }.into();
        return (leader.into(), vec![out_msg]);
    }

    (leader.into(), vec![])
}

pub fn process_msg(
    msg: Message,
    node: &mut Node,
    leader: Leader,
    _: &mut RngCore,
) -> (Role, Vec<Message>) {
    use Message::*;
    match msg {
        Heartbeat(heartbeat) => {
            if heartbeat.term > node.term {
                let follower = Follower {
                    leader_id: heartbeat.leader_id,
                };
                return (follower.into(), vec![]);
            }

            if heartbeat.term == node.term && node.id != heartbeat.leader_id {
                let candidate = Candidate { votes: 1 };
                let out_msg = message::Candidacy {
                    candidate_id: node.id.clone(),
                    term: node.term,
                }.into();
                return (candidate.into(), vec![out_msg]);
            }

            (leader.into(), vec![])
        }
        Candidacy(_) | Vote(_) => (leader.into(), vec![]),
    }
}
