use super::*;
use rand::RngCore;

pub fn poll(
    node: &mut Node,
    mut leader: Leader,
    _: &mut RngCore,
) -> Result<(Role, Vec<Message>), Error> {
    let time_for_heartbeat = node.time >= leader.last_sent_heartbeat + HEARTBEAT_PERIOD;
    if time_for_heartbeat {
        leader.last_sent_heartbeat = node.time;
        println!("{} leader sent heartbeat", node.log_prefix(),);
        let msg = message::Heartbeat {
            term: node.term,
            nodes: node.peers.clone(),
        }.into_message(node.id);
        Ok((leader.into_role(), vec![msg]))
    } else {
        Ok((leader.into_role(), vec![]))
    }
}

pub fn process_msg(
    msg: Message,
    node: &mut Node,
    mut leader: Leader,
    _: &mut RngCore,
) -> Result<(Role, Vec<Message>), Error> {
    use Message::*;
    match msg {
        Heartbeat(other_leader_id, heartbeat) => {
            let other_leader_is_later_term = heartbeat.term > node.term;
            if other_leader_is_later_term {
                println!(
                    "{} leader followed later-term leader {}",
                    node.log_prefix(),
                    other_leader_id,
                );
                let follower = Follower {
                    last_recv_heartbeat: node.time,
                    voted: None,
                };
                return Ok((follower.into_role(), vec![]));
            }
            let leader_is_duplicate = heartbeat.term == node.term;
            if leader_is_duplicate {
                println!(
                    "{} leader became next-term candidate because of same-term leader {}",
                    node.log_prefix(),
                    other_leader_id,
                );
                let candidate = Candidate { votes: 1 };
                let msg = message::Candidacy { term: node.term }.into_message(node.id);
                return Ok((candidate.into_role(), vec![msg]));
            }
            Ok((leader.into_role(), vec![]))
        }
        Candidacy(_, _) | Vote(_, _) => Ok((leader.into_role(), vec![])),
    }
}
