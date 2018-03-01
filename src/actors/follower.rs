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
        mut outbox: &mut MessageQueue,
        rng: &mut RngCore,
    ) -> Result<Actor, Error> {
        self.follower.time += 1;
        while let Some(in_msg) = inbox.pop_front() {
            self.process_msg(in_msg, outbox)?;
        }
        if self.heartbeat_expired(rng) {
            return Ok(self.announce_candidacy(&mut outbox));
        }
        return Ok(Actor::Follower(self));
    }

    fn process_msg(&mut self, msg: Message, mut outbox: &mut MessageQueue) -> Result<(), Error> {
        match msg {
            Message::Heartbeat(heartbeat_msg) => {
                let is_valid_heartbeat = heartbeat_msg.term >= self.follower.term;
                if is_valid_heartbeat {
                    self.heartbeated(heartbeat_msg);
                }
                Ok(())
            }
            Message::Candidacy(candidacy_msg) => {
                let candidate_is_older_term = candidacy_msg.term < self.follower.term;
                if candidate_is_older_term {
                    return Ok(());
                }
                let candidate_is_later_term = candidacy_msg.term > self.follower.term;
                if candidate_is_later_term {
                    self.follower.term = candidacy_msg.term;
                    self.follower.voted = None;
                }
                let has_already_voted = self.follower.voted.is_some();
                if !has_already_voted {
                    self.vote_for_candidate(candidacy_msg, &mut outbox);
                }
                Ok(())
            }
            Message::Vote(_) => Ok(()),
        }
    }

    fn heartbeat_expired(&self, mut rng: &mut RngCore) -> bool {
        let timeout = rng.gen_range(ELECTION_TIMEOUT, ELECTION_TIMEOUT * 2);
        self.follower.last_recv_heartbeat + timeout < self.follower.time
    }

    fn heartbeated(&mut self, heartbeat_msg: HeartbeatMessage) {
        self.follower.term = heartbeat_msg.term;
        self.follower.last_recv_heartbeat = self.follower.time;
        self.follower.nodes = heartbeat_msg.nodes;
        println!(
            "[time {}] [id {}] [term {}] follower heartbeated by leader {}",
            self.follower.time, self.follower.id, self.follower.term, heartbeat_msg.from,
        );
    }

    fn announce_candidacy(self, outbox: &mut MessageQueue) -> Actor {
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
        Actor::Candidate(CandidateActor { candidate })
    }

    fn vote_for_candidate(&mut self, candidacy_msg: CandidacyMessage, outbox: &mut MessageQueue) {
        self.follower.voted = Some(candidacy_msg.candidate);
        self.follower.last_recv_heartbeat = self.follower.time;
        outbox.push_back(Message::Vote(VoteMessage {
            term: self.follower.term,
            candidate: candidacy_msg.candidate,
            elector: self.follower.id,
        }));
        println!(
            "[time {}] [id {}] [term {}] follower voted for candidate {}",
            self.follower.time, self.follower.id, self.follower.term, candidacy_msg.candidate,
        );
    }
}
