use super::*;
use rand::RngCore;

pub fn poll(node: &mut Node, candidate: Candidate, rng: &mut RngCore) -> (Role, Vec<Message>) {
    let necessary_votes = (node.peers.len() / 2) as u64;
    if candidate.votes > necessary_votes {
        let leader = Leader {};
        node.last_activity = node.time;
        node.log(&format!(
            "CANDIDATE became LEADER (received >{} votes: {}/{})",
            necessary_votes,
            candidate.votes,
            node.peers.len()
        ));
        return heartbeat(node, leader);
    }

    if let Some(result) = poll_election_timeout(node, rng) {
        node.log("CANDIDATE became CANDIDATE (election timeout)");
        return result;
    }

    (candidate.into(), vec![])
}

pub fn process_msg(
    msg: Message,
    node: &mut Node,
    mut candidate: Candidate,
    _: &mut RngCore,
) -> (Role, Vec<Message>) {
    use Message::*;
    match msg {
        Heartbeat(heartbeat) => {
            if heartbeat.term < node.term {
                return (candidate.into(), vec![]);
            }

            let result = follower_of(node, heartbeat.clone());
            node.log(&format!(
                "CANDIDATE became FOLLOWER (heartbeat from {:?})",
                heartbeat.leader_id
            ));
            result
        }
        Candidacy(candidacy) => {
            if candidacy.term <= node.term {
                return (candidate.into(), vec![]);
            }

            let result = voter_for(node, candidacy.clone());
            node.log(&format!(
                "CANDIDATE became VOTER (later-term candidacy of {:?})",
                candidacy.candidate_id
            ));
            result
        }
        Vote(vote) => {
            let was_voted_for = (vote.term, vote.candidate) == (node.term, node.id.clone());
            if was_voted_for {
                // FIXME: Should the candidate consider a vote for itself as activity?
                candidate.votes += 1;
                node.log(&format!("CANDIDATE received vote from {:?}", vote.voter_id));
            }
            // FIXME: Should a later-term vote really be ignored?
            (candidate.into(), vec![])
        }
    }
}
