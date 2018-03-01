use super::*;
use rand::{Rng, RngCore};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FollowerActor {
    pub follower: Follower,
}

impl FollowerActor {
    pub fn act(
        mut self,
        inbox: &mut MessageQueue,
        outbox: &mut MessageQueue,
        rng: &mut RngCore,
    ) -> Result<Actor, Error> {
        self.follower.time += 1;

        // Process messages
        while let Some(in_msg) = inbox.pop_front() {
            self.process_msg(in_msg, outbox)?;
        }

        // Check if it is time to become a candidate
        if !self.heartbeat_expired(rng) {
            return Ok(Actor::Follower(self));
        }

        let candidate = Candidate {
            id: self.follower.id,
            time: self.follower.time,
            term: self.follower.term + 1,
            nodes: self.follower.nodes,
            votes: 1,
        };
        outbox.push_back(Message::Candidacy(CandidacyMessage {
            term: candidate.term,
            candidate: candidate.id,
        }));
        println!(
            "[time {}] [id {}] [term {}] follower became a candidate",
            candidate.time, candidate.id, candidate.term,
        );
        Ok(Actor::Candidate(CandidateActor { candidate }))
    }

    fn heartbeat_expired(&self, mut rng: &mut RngCore) -> bool {
        let timeout = rng.gen_range(ELECTION_TIMEOUT, ELECTION_TIMEOUT * 2);
        self.follower.last_recv_heartbeat + timeout < self.follower.time
    }

    fn process_msg(&mut self, msg: Message, outbox: &mut MessageQueue) -> Result<(), Error> {
        match msg {
            Message::Heartbeat(heartbeat_msg) => {
                self.follower.last_recv_heartbeat = self.follower.time;
                self.follower.nodes = heartbeat_msg.nodes;
                println!(
                    "[time {}] [id {}] [term {}] follower got heartbeat from {}",
                    self.follower.time, self.follower.id, self.follower.term, heartbeat_msg.from,
                );
                Ok(())
            }
            Message::Candidacy(candidacy_msg) => {
                if candidacy_msg.term < self.follower.term {
                    return Ok(());
                }
                if candidacy_msg.term > self.follower.term {
                    self.follower.term = candidacy_msg.term;
                    self.follower.voted = None;
                }
                if self.follower.voted.is_some() {
                    return Ok(());
                }
                self.follower.voted = Some(candidacy_msg.candidate);
                self.follower.last_recv_heartbeat = self.follower.time;
                outbox.push_back(Message::Vote(VoteMessage {
                    term: self.follower.term,
                    candidate: candidacy_msg.candidate,
                    elector: self.follower.id,
                }));
                println!(
                    "[time {}] [id {}] [term {}] follower voted for candidate {}",
                    self.follower.time,
                    self.follower.id,
                    self.follower.term,
                    candidacy_msg.candidate,
                );
                Ok(())
            }
            Message::Vote(vote_msg) => {
                if vote_msg.term < self.follower.term {
                    return Ok(());
                }
                self.follower.term = vote_msg.term;
                //println!(
                //    "[time {}] [id {}] [term {}] follower notified {} voted for {}",
                //    self.follower.time,
                //    self.follower.id,
                //    self.follower.term,
                //    vote_msg.elector,
                //    vote_msg.candidate,
                //);
                Ok(())
            }
        }
    }
}
