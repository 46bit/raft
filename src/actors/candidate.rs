use super::*;
use rand::RngCore;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateActor {
    pub candidate: Candidate,
}

impl CandidateActor {
    pub fn act(
        mut self,
        inbox: &mut MessageQueue,
        outbox: &mut MessageQueue,
        rng: &mut RngCore,
    ) -> Result<Actor, Error> {
        self.candidate.time += 1;
        while let Some(in_msg) = inbox.pop_front() {
            if let Some(next_actor) = self.process_msg(in_msg, outbox)? {
                return next_actor.act(inbox, outbox, rng);
            }
        }
        if self.elected() {
            return self.take_leadership().act(inbox, outbox, rng);
        }
        Ok(Actor::Candidate(self))
    }

    fn process_msg(
        &mut self,
        msg: Message,
        mut outbox: &mut MessageQueue,
    ) -> Result<Option<Actor>, Error> {
        match msg {
            Message::Heartbeat(heartbeat_msg) => {
                let heartbeat_is_not_earlier_term = heartbeat_msg.term >= self.candidate.term;
                if heartbeat_is_not_earlier_term {
                    return Ok(Some(self.clone().follow_leader(heartbeat_msg)));
                }
            }
            Message::Candidacy(candidacy_msg) => {
                let candidate_is_later_term = candidacy_msg.term > self.candidate.term;
                if candidate_is_later_term {
                    return Ok(Some(
                        self.clone()
                            .follow_later_term_candidate(candidacy_msg, &mut outbox),
                    ));
                }
            }
            Message::Vote(vote_msg) => {
                if self.voted(&vote_msg) {
                    self.candidate.votes += 1;
                    println!(
                        "[time {}] [id {}] [term {}] candidate received vote from {}",
                        self.candidate.time,
                        self.candidate.id,
                        self.candidate.term,
                        vote_msg.elector,
                    );
                }
            }
        }
        Ok(None)
    }

    fn voted(&self, vote_msg: &VoteMessage) -> bool {
        vote_msg.term == self.candidate.term && vote_msg.candidate == self.candidate.id
    }

    fn elected(&self) -> bool {
        let necessary_votes = (self.candidate.nodes.len() / 2) as u64;
        self.candidate.votes > necessary_votes
    }

    fn take_leadership(self) -> Actor {
        let number_of_votes = self.candidate.votes;
        let leader = Leader {
            id: self.candidate.id,
            time: self.candidate.time,
            term: self.candidate.term,
            nodes: self.candidate.nodes,
            last_sent_heartbeat: 0,
        };
        println!(
            "[time {}] [id {}] [term={}] candidate elected with {}/{} votes",
            leader.time,
            leader.id,
            leader.term,
            number_of_votes,
            leader.nodes.len()
        );
        Actor::Leader(LeaderActor { leader })
    }

    fn follow_leader(self, heartbeat_msg: HeartbeatMessage) -> Actor {
        let follower = Follower {
            id: self.candidate.id,
            time: self.candidate.time,
            term: heartbeat_msg.term,
            nodes: heartbeat_msg.nodes,
            last_recv_heartbeat: self.candidate.time,
            voted: None,
        };
        println!(
            "[time {}] [id {}] [term {}] candidate followed leader {}",
            follower.time, follower.id, follower.term, heartbeat_msg.from,
        );
        Actor::Follower(FollowerActor { follower })
    }

    fn follow_later_term_candidate(
        self,
        candidacy_msg: CandidacyMessage,
        outbox: &mut MessageQueue,
    ) -> Actor {
        let follower = Follower {
            id: self.candidate.id,
            time: self.candidate.time,
            term: candidacy_msg.term,
            nodes: self.candidate.nodes.clone(),
            last_recv_heartbeat: 0,
            voted: Some(self.candidate.id),
        };
        outbox.push_back(Message::Vote(VoteMessage {
            term: follower.term,
            candidate: candidacy_msg.candidate,
            elector: follower.id,
        }));
        println!(
            "[time {}] [id {}] [term {}] candidate resigned for later-term candidate {}",
            follower.time, follower.id, follower.term, candidacy_msg.candidate,
        );
        Actor::Follower(FollowerActor { follower })
    }
}
