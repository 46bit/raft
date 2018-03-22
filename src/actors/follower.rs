use super::*;
use rand::{Rng, RngCore};

pub fn poll(
    node: &mut Node,
    mut follower: Follower,
    mut rng: &mut RngCore,
) -> Result<(Role, Vec<Message>), Error> {
    let heartbeat_expired = follower.last_recv_heartbeat
        + rng.gen_range(ELECTION_TIMEOUT, ELECTION_TIMEOUT * 2)
        < node.time;
    if heartbeat_expired {
        node.term += 1;
        let candidate = Candidate { votes: 1 };
        println!(
            "[time {}] [id {}] [term {}] follower became a candidate",
            node.time, node.id, node.term,
        );
        let msg = message::Candidacy { term: node.term }.into_message(node.id);
        Ok((candidate.into_role(), vec![msg]))
    } else {
        Ok((follower.into_role(), vec![]))
    }
}

pub fn process_msg(
    msg: Message,
    node: &mut Node,
    mut follower: Follower,
    _: &mut RngCore,
) -> Result<(Role, Vec<Message>), Error> {
    use Message::*;
    match msg {
        Heartbeat(leader_id, heartbeat) => {
            let is_valid_heartbeat = heartbeat.term >= node.term;
            if is_valid_heartbeat {
                follower.last_recv_heartbeat = node.time;
                node.term = heartbeat.term;
                node.peers = heartbeat.nodes;
                println!(
                    "{} follower heartbeated by leader {}",
                    node.log_prefix(),
                    leader_id,
                );
            }
            Ok((follower.into_role(), vec![]))
        }
        Candidacy(candidate_id, candidacy) => {
            let candidate_is_older_term = candidacy.term < node.term;
            if candidate_is_older_term {
                return Ok((follower.into_role(), vec![]));
            }
            let candidate_is_later_term = candidacy.term > node.term;
            if candidate_is_later_term {
                node.term = candidacy.term;
                follower.voted = None;
            }
            let has_already_voted = follower.voted.is_some();
            if has_already_voted {
                return Ok((follower.into_role(), vec![]));
            }
            follower.voted = Some(candidate_id);
            follower.last_recv_heartbeat = node.time;
            println!(
                "{} follower voted for candidate {}",
                node.log_prefix(),
                candidate_id,
            );
            let msg = message::Vote {
                term: node.term,
                candidate: candidate_id,
            }.into_message(node.id);
            Ok((follower.into_role(), vec![msg]))
        }
        Vote(_, _) => Ok((follower.into_role(), vec![])),
    }
}
