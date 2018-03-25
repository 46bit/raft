use super::*;
use rand::RngCore;

pub fn poll(node: &mut Node, leader: Leader, _: &mut RngCore) -> (Role, Vec<Message>) {
    let time_for_heartbeat = node.time >= node.last_activity + HEARTBEAT_PERIOD;
    if time_for_heartbeat {
        node.last_activity = node.time;
        let msg = message::Heartbeat {
            term: node.term,
            nodes: node.peers.clone(),
        }.into_message(node.id.clone());
        return (leader.into(), vec![msg]);
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
        Heartbeat(leader_id, heartbeat) => {
            if heartbeat.term > node.term {
                let follower = Follower { leader_id };
                return (follower.into(), vec![]);
            }

            if heartbeat.term == node.term && node.id != leader_id {
                let candidate = Candidate { votes: 1 };
                let msg = message::Candidacy { term: node.term }.into_message(node.id.clone());
                return (candidate.into(), vec![msg]);
            }

            (leader.into(), vec![])
        }
        Candidacy(_, _) | Vote(_, _) => (leader.into(), vec![]),
    }
}
