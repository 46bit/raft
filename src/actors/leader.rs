use super::*;
use rand::RngCore;

pub fn poll(node: &mut Node, leader: Leader, _: &mut RngCore) -> (Role, Vec<Message>) {
    let time_for_heartbeat = node.time >= node.last_activity + HEARTBEAT_PERIOD;
    if time_for_heartbeat {
        let result = heartbeat(node, leader);
        node.log("LEADER heartbeated (reached heartbeat period)");
        return result;
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
                let result = follower_of(node, heartbeat.clone());
                node.log(&format!(
                    "LEADER became FOLLOWER of {:?} (received later-term heartbeat)",
                    heartbeat.leader_id
                ));
                return result;
            }

            if heartbeat.term == node.term && node.id != heartbeat.leader_id {
                node.log("LEADER became IDLER (received duplicate-term heartbeat)");
                return (Role::Idler, vec![]);
            }

            (leader.into(), vec![])
        }
        Candidacy(_) | Vote(_) => (leader.into(), vec![]),
    }
}
