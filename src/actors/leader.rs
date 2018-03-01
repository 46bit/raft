use super::*;
use rand::RngCore;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaderActor {
    pub leader: Leader,
}

impl LeaderActor {
    pub fn act(
        mut self,
        inbox: &mut MessageQueue,
        outbox: &mut MessageQueue,
        rng: &mut RngCore,
    ) -> Result<Actor, Error> {
        self.leader.time += 1;

        while let Some(in_msg) = inbox.pop_front() {
            if let Message::Heartbeat(heartbeat_msg) = in_msg {
                // Adopt later-term leaders
                if heartbeat_msg.term > self.leader.term {
                    let follower = Follower {
                        id: self.leader.id,
                        time: self.leader.time,
                        term: heartbeat_msg.term,
                        nodes: heartbeat_msg.nodes,
                        last_recv_heartbeat: self.leader.time,
                        voted: None,
                    };
                    println!(
                        "[time {}] [id {}] [term {}] leader followed later-term leader {}",
                        follower.time, follower.id, follower.term, heartbeat_msg.from,
                    );
                    return Actor::Follower(FollowerActor { follower }).act(inbox, outbox, rng);
                }
                if heartbeat_msg.term == self.leader.term {
                    let candidate = Candidate {
                        id: self.leader.id,
                        time: self.leader.time,
                        term: self.leader.term + 1,
                        nodes: self.leader.nodes,
                        votes: 1,
                    };
                    outbox.push_back(Message::Candidacy(CandidacyMessage {
                        term: candidate.term,
                        candidate: candidate.id,
                    }));
                    println!(
                        "[time {}] [id {}] [term {}] leader became next-term candidate because of same-term leader {}",
                        candidate.time, candidate.id, candidate.term, heartbeat_msg.from,
                    );
                    return Actor::Candidate(CandidateActor { candidate }).act(inbox, outbox, rng);
                }
            }
        }

        if self.time_for_heartbeat() {
            self.leader.last_sent_heartbeat = self.leader.time;
            outbox.push_back(Message::Heartbeat(HeartbeatMessage {
                term: self.leader.term,
                from: self.leader.id,
                nodes: self.leader.nodes.clone(),
            }));
            println!(
                "[time {}] [id {}] [term {}] leader sent heartbeat",
                self.leader.time, self.leader.id, self.leader.term,
            );
        }

        Ok(Actor::Leader(self))
    }

    fn time_for_heartbeat(&self) -> bool {
        self.leader.time >= self.leader.last_sent_heartbeat + ELECTION_TIMEOUT / 2
    }
}
