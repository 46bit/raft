use super::*;
use rand::RngCore;

pub fn poll(
    node: &mut Node,
    mut candidate: Candidate,
    _: &mut RngCore,
) -> Result<(Role, Vec<Message>), Error> {
    let necessary_votes = (node.peers.len() / 2) as u64;
    if candidate.votes > necessary_votes {
        println!(
            "{} candidate elected with {}/{} votes",
            node.log_prefix(),
            candidate.votes,
            node.peers.len()
        );
        let leader = Leader {
            last_sent_heartbeat: 0,
        };
        Ok((leader.into_role(), vec![]))
    } else {
        Ok((candidate.into_role(), vec![]))
    }
}

pub fn process_msg(
    msg: Message,
    node: &mut Node,
    mut candidate: Candidate,
    _: &mut RngCore,
) -> Result<(Role, Vec<Message>), Error> {
    use Message::*;
    match msg {
        Heartbeat(leader_id, heartbeat) => {
            let leader_is_not_earlier_term = heartbeat.term >= node.term;
            if leader_is_not_earlier_term {
                println!(
                    "{} candidate followed leader {}",
                    node.log_prefix(),
                    leader_id,
                );
                let follower = Follower {
                    last_recv_heartbeat: node.time,
                    voted: None,
                };
                Ok((follower.into_role(), vec![]))
            } else {
                Ok((candidate.into_role(), vec![]))
            }
        }
        Candidacy(other_candidate_id, candidacy) => {
            let other_candidate_is_later_term = candidacy.term > node.term;
            if other_candidate_is_later_term {
                node.term = candidacy.term;
                println!(
                    "{} candidate resigned for later-term candidate {}",
                    node.log_prefix(),
                    other_candidate_id,
                );
                let follower = Follower {
                    last_recv_heartbeat: 0,
                    voted: Some(other_candidate_id),
                };
                let msg = message::Vote {
                    term: candidacy.term,
                    candidate: other_candidate_id,
                }.into_message(node.id);
                Ok((follower.into_role(), vec![msg]))
            } else {
                Ok((candidate.into_role(), vec![]))
            }
        }
        Vote(follower_id, vote) => {
            let was_voted_for = (vote.term, vote.candidate) == (node.term, node.id);
            if was_voted_for {
                candidate.votes += 1;
                println!(
                    "{} candidate received vote from {}",
                    node.log_prefix(),
                    follower_id,
                );
            }
            Ok((candidate.into_role(), vec![]))
        }
    }
}
