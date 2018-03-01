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
        mut outbox: &mut MessageQueue,
        rng: &mut RngCore,
    ) -> Result<Actor, Error> {
        self.leader.time += 1;
        while let Some(in_msg) = inbox.pop_front() {
            if let Some(next_actor) = self.process_msg(in_msg, outbox)? {
                return next_actor.act(inbox, outbox, rng);
            }
        }
        let time_for_heartbeat = self.leader.time >= self.leader.last_sent_heartbeat + HEARTBEAT_PERIOD;
        if time_for_heartbeat {
            self.heartbeat(&mut outbox);
        }
        Ok(Actor::Leader(self))
    }

    fn process_msg(
        &mut self,
        msg: Message,
        mut outbox: &mut MessageQueue,
    ) -> Result<Option<Actor>, Error> {
        if let Message::Heartbeat(heartbeat_msg) = msg {
            let other_leader_is_later_term = heartbeat_msg.term > self.leader.term;
            if other_leader_is_later_term {
                return Ok(Some(self.clone().adopt_later_term_leader(heartbeat_msg)));
            }
            let leader_is_duplicate = heartbeat_msg.term == self.leader.term;
            if leader_is_duplicate {
                return Ok(Some(
                    self.clone()
                        .replace_duplicate_term_leader(heartbeat_msg, &mut outbox),
                ));
            }
        }
        Ok(None)
    }

    fn heartbeat(&mut self, outbox: &mut MessageQueue) {
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

    fn adopt_later_term_leader(self, heartbeat_msg: HeartbeatMessage) -> Actor {
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
        Actor::Follower(FollowerActor { follower })
    }

    fn replace_duplicate_term_leader(
        self,
        heartbeat_msg: HeartbeatMessage,
        outbox: &mut MessageQueue,
    ) -> Actor {
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
        Actor::Candidate(CandidateActor { candidate })
    }
}
