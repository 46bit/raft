use super::*;
use rand::RngCore;

pub fn poll(node: &mut Node, follower: Follower, rng: &mut RngCore) -> (Role, Vec<Message>) {
    if let Some(result) = poll_election_timeout(node, rng) {
        return result;
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
            if node.term > heartbeat.term {
                return (follower.into(), vec![]);
            }

            if node.term < heartbeat.term {
                node.term = heartbeat.term;
                node.last_activity = node.time;
                follower.leader_id = heartbeat.leader_id;
                return (follower.into(), vec![]);
            }

            if follower.leader_id != heartbeat.leader_id {
                return go_into_idle();
            }

            node.last_activity = node.time;
            (follower.into(), vec![])
        }
        Candidacy(candidacy) => {
            if candidacy.term <= node.term {
                return (follower.into(), vec![]);
            }

            vote_for_later_candidate(node, candidacy)
        }
        Vote(..) => (follower.into(), vec![]),
    }
}
