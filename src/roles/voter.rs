use super::*;
use rand::RngCore;

pub fn poll(node: &mut Node, voter: Voter, rng: &mut RngCore) -> (Role, Vec<Message>) {
    if let Some(result) = poll_election_timeout(node, rng) {
        println!("{} VOTER became CANDIDATE", node.log_prefix());
        return result;
    }

    (voter.into(), vec![])
}

pub fn process_msg(
    in_msg: Message,
    node: &mut Node,
    voter: Voter,
    _: &mut RngCore,
) -> (Role, Vec<Message>) {
    use Message::*;
    match in_msg {
        Heartbeat(heartbeat) => {
            if heartbeat.term < node.term {
                return (voter.into(), vec![]);
            }

            let result = follower_of(node, heartbeat.clone());
            node.log(&format!(
                "VOTER became FOLLOWER of {:?} (received heartbeat)",
                heartbeat.leader_id
            ));
            result
        }
        Candidacy(..) | Vote(..) => (voter.into(), vec![]),
    }
}
