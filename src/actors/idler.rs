use super::*;
use rand::RngCore;

pub fn poll(node: &mut Node, idler: Idler, rng: &mut RngCore) -> (Role, Vec<Message>) {
    if let Some(result) = poll_election_timeout(node, rng) {
        println!("{} IDLER became CANDIDATE", node.log_prefix());
        return result;
    }

    (idler.into(), vec![])
}

pub fn process_msg(
    in_msg: Message,
    node: &mut Node,
    idler: Idler,
    _: &mut RngCore,
) -> (Role, Vec<Message>) {
    use Message::*;
    match in_msg {
        Heartbeat(heartbeat) => {
            if heartbeat.term < node.term {
                return (idler.into(), vec![]);
            }

            let result = follow_leader(node, heartbeat);
            println!("{} IDLER became FOLLOWER", node.log_prefix());
            result
        }
        Candidacy(candidacy) => {
            if candidacy.term < node.term {
                return (idler.into(), vec![]);
            }

            // FIXME: Now actions are deduped, I think it'd be better to have a
            // dedicated Voter role to indicate when a vote has been cast.
            if idler.vote.is_none() {
                let result = vote_for_later_candidate(node, candidacy.clone());
                println!("{} IDLER became IDLER", node.log_prefix());
                return result;
            }

            (idler.into(), vec![])
        }
        Vote(..) => (idler.into(), vec![]),
    }
}
