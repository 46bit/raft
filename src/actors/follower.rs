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
    follower: Follower,
    _: &mut RngCore,
) -> (Role, Vec<Message>) {
    use Message::*;
    match in_msg {
        Heartbeat(heartbeat) => {
            if node.term > heartbeat.term {
                return (follower.into(), vec![]);
            }

            if node.term < heartbeat.term {
                let result = follow_leader(node, heartbeat);
                println!("{} FOLLOWER became FOLLOWER", node.log_prefix());
                return result;
            }

            if follower.leader_id != heartbeat.leader_id {
                println!(
                    "{} FOLLOWER of {} heartbeated by same-term leader {}",
                    node.log_prefix(),
                    follower.leader_id,
                    heartbeat.leader_id
                );
                println!("{} FOLLOWER became IDLER", node.log_prefix());
                return go_into_idle();
            }

            // FIXME: Log?
            node.last_activity = node.time;
            (follower.into(), vec![])
        }
        Candidacy(candidacy) => {
            if candidacy.term <= node.term {
                return (follower.into(), vec![]);
            }

            // FIXME: Log
            vote_for_later_candidate(node, candidacy)
        }
        Vote(..) => (follower.into(), vec![]),
    }
}
