use super::*;
use rand::RngCore;

pub fn poll(node: &mut Node, follower: Follower, rng: &mut RngCore) -> (Role, Vec<Message>) {
    if let Some(result) = poll_election_timeout(node, rng) {
        node.log("FOLLOWER became CANDIDATE (election timeout)");
        return result;
    }

    (follower.into(), vec![])
}

pub fn process_msg(
    in_msg: Message,
    node: &mut Node,
    follower: Follower,
    _: &mut RngCore,
) -> (Role, Vec<Message>) {
    use Message::*;
    match in_msg {
        Heartbeat(heartbeat) => {
            if heartbeat.term < node.term {
                return (follower.into(), vec![]);
            }

            if heartbeat.term > node.term {
                let result = follower_of(node, heartbeat.clone());
                node.log(&format!(
                    "FOLLOWER became FOLLOWER of {:?} (received later-term heartbeat)",
                    heartbeat.leader_id
                ));
                return result;
            }

            if heartbeat.leader_id != follower.leader_id {
                node.log(&format!(
                    "FOLLOWER became IDLER (heartbeat from {:?} conflicts with leader {:?})",
                    heartbeat.leader_id, follower.leader_id,
                ));
                return (Role::Idler, vec![]);
            }

            node.last_activity = node.time;
            node.log(&format!(
                "FOLLOWER heartbeated by its leader {:?}",
                follower.leader_id,
            ));
            (follower.into(), vec![])
        }
        Candidacy(candidacy) => {
            if candidacy.term <= node.term {
                return (follower.into(), vec![]);
            }

            let result = voter_for(node, candidacy.clone());
            node.log(&format!(
                "FOLLOWER became VOTER for {:?} (received candidacy)",
                candidacy.candidate_id,
            ));
            result
        }
        Vote(..) => (follower.into(), vec![]),
    }
}
