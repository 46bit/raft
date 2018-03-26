use super::*;
use rand::RngCore;

pub fn poll(node: &mut Node, rng: &mut RngCore) -> (Role, Vec<Message>) {
    if let Some(result) = poll_election_timeout(node, rng) {
        node.log("IDLER became CANDIDATE (election timeout)");
        return result;
    }

    (Role::Idler, vec![])
}

pub fn process_msg(in_msg: Message, node: &mut Node, _: &mut RngCore) -> (Role, Vec<Message>) {
    use Message::*;
    match in_msg {
        Heartbeat(heartbeat) => {
            if heartbeat.term < node.term {
                return (Role::Idler, vec![]);
            }

            let result = follower_of(node, heartbeat.clone());
            node.log(&format!(
                "IDLER became FOLLOWER of {:?} (received heartbeat)",
                heartbeat.leader_id
            ));
            result
        }
        Candidacy(candidacy) => {
            if candidacy.term < node.term {
                return (Role::Idler, vec![]);
            }

            let result = voter_for(node, candidacy.clone());
            node.log(&format!(
                "IDLER became VOTER for {:?} (received candidacy)",
                candidacy.candidate_id
            ));
            result
        }
        Vote(..) => (Role::Idler, vec![]),
    }
}
