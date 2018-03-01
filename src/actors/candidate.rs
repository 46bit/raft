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

        if !self.elected() {
            return Ok(Actor::Candidate(self));
        }

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
        Actor::Leader(LeaderActor { leader }).act(inbox, outbox, rng)
    }

    fn elected(&self) -> bool {
        let necessary_votes = (self.candidate.nodes.len() / 2) as u64;
        self.candidate.votes > necessary_votes
    }

    fn process_msg(
        &mut self,
        msg: Message,
        outbox: &mut MessageQueue,
    ) -> Result<Option<Actor>, Error> {
        match msg {
            Message::Heartbeat(heartbeat_msg) => {
                if heartbeat_msg.term < self.candidate.term {
                    return Ok(None);
                }

                // Become a follower of a newer leader that appears
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
                Ok(Some(Actor::Follower(FollowerActor { follower })))
            }
            Message::Candidacy(candidacy_msg) => {
                if candidacy_msg.term <= self.candidate.term {
                    return Ok(None);
                }

                // Vote for newer-term candidates and become a follower
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
                Ok(Some(Actor::Follower(FollowerActor { follower })))
            }
            Message::Vote(vote_msg) => {
                // Record votes for same-term candidates
                if vote_msg.term == self.candidate.term && vote_msg.candidate == self.candidate.id {
                    self.candidate.votes += 1;
                    println!(
                        "[time {}] [id {}] [term {}] candidate received vote from {}",
                        self.candidate.time,
                        self.candidate.id,
                        self.candidate.term,
                        vote_msg.elector,
                    );
                }
                Ok(None)
            }
        }
    }
}
