use super::*;
use rand::Rng;

// How long to wait for a master's heartbeat before starting an election.
pub const ELECTION_TIMEOUT: Time = 150;

#[derive(Debug, Clone)]
pub struct Actor {
    pub inbox: MessageQueue,
    pub outbox: MessageQueue,
}

impl Actor {
    pub fn new() -> Actor {
        Actor {
            inbox: MessageQueue::new(),
            outbox: MessageQueue::new(),
        }
    }

    pub fn step(&mut self, node: &mut Node) -> Result<(), ()> {
        let new_node = match node {
            &mut Node::Follower(ref mut follower) => self.follower_step(follower),
            &mut Node::Candidate(ref mut candidate) => self.candidate_step(candidate),
            &mut Node::Leader(ref mut leader) => self.leader_step(leader),
        }?;
        *node = new_node;
        Ok(())
    }

    fn follower_step(&mut self, follower: &mut FollowerNode) -> Result<Node, ()> {
        follower.time += 1;

        while let Some(in_msg) = self.inbox.pop_front() {
            if let Some(out_msg) = self.follower_process_msg(in_msg, follower)? {
                self.outbox.push_back(out_msg);
            }
        }

        let mut rng = rand::thread_rng();
        let timeout = rng.gen_range(ELECTION_TIMEOUT, ELECTION_TIMEOUT * 2);
        if follower.last_recv_heartbeat + timeout < follower.time {
            let candidate = CandidateNode {
                id: follower.id,
                time: follower.time,
                term: follower.term + 1,
                nodes: follower.nodes.clone(),
                votes: 1,
            };
            println!(
                "[{}] [{}] follower {} became a candidate for term {}",
                candidate.time, candidate.term, candidate.id, candidate.term,
            );
            self.outbox.push_back(Message::Candidacy(CandidacyMessage {
                term: candidate.term,
                candidate: candidate.id,
            }));
            return Ok(Node::Candidate(candidate));
        }

        Ok(Node::Follower(follower.clone()))
    }

    fn follower_process_msg(
        &mut self,
        msg: Message,
        follower: &mut FollowerNode,
    ) -> Result<Option<Message>, ()> {
        match msg {
            Message::Heartbeat(heartbeat) => {
                println!(
                    "[{}] [{}] follower {} got heartbeat from {}",
                    follower.time, follower.term, follower.id, heartbeat.from,
                );
                follower.last_recv_heartbeat = follower.time;
                follower.nodes = heartbeat.nodes;
                Ok(None)
            }
            Message::Candidacy(candidacy) => {
                if candidacy.term < follower.term {
                    return Ok(None);
                }
                println!(
                    "[{}] [{}] follower {} notified {} is a candidate for term {}",
                    follower.time, follower.term, follower.id, candidacy.candidate, candidacy.term,
                );
                if candidacy.term > follower.term {
                    follower.term = candidacy.term;
                    follower.voted = None;
                }
                if follower.voted.is_some() {
                    return Ok(None);
                }
                follower.voted = Some(candidacy.candidate);
                Ok(Some(Message::Vote(VoteMessage {
                    term: follower.term,
                    candidate: candidacy.candidate,
                    elector: follower.id,
                })))
            }
            Message::Vote(vote) => {
                if vote.term < follower.term {
                    return Ok(None);
                }
                follower.term = vote.term;
                println!(
                    "[{}] [{}] follower {} notified {} voted for {}",
                    follower.time, follower.term, follower.id, vote.elector, vote.candidate,
                );
                Ok(None)
            }
        }
    }

    fn candidate_step(&mut self, candidate: &mut CandidateNode) -> Result<Node, ()> {
        candidate.time += 1;

        while let Some(in_msg) = self.inbox.pop_front() {
            match in_msg {
                Message::Heartbeat(heartbeat) => {
                    let follower = FollowerNode {
                        id: candidate.id,
                        time: candidate.time,
                        term: heartbeat.term,
                        nodes: heartbeat.nodes,
                        last_recv_heartbeat: candidate.time,
                        voted: None,
                    };
                    println!(
                        "[{}] [{}] candidate {} became follower of {}",
                        follower.time, follower.term, follower.id, heartbeat.from,
                    );
                    return Ok(Node::Follower(follower));
                }
                Message::Candidacy(candidacy) => {
                    if candidacy.term < candidate.term {
                        continue;
                    }
                    // FIXME: If candidate for greater term, become a follower of it
                    if candidacy.term > candidate.term {
                        let follower = FollowerNode {
                            id: candidate.id,
                            time: candidate.time,
                            term: candidacy.term,
                            nodes: candidate.nodes.clone(),
                            last_recv_heartbeat: 0,
                            voted: None,
                        };
                        return Ok(Node::Follower(follower));
                        //unimplemented!();
                        continue;
                    }
                    println!(
                        "[{}] [{}] candidate {} notified {} is a candidate for term {}",
                        candidate.time,
                        candidate.term,
                        candidate.id,
                        candidacy.candidate,
                        candidacy.term,
                    );
                }
                Message::Vote(vote) => {
                    if vote.term < candidate.term {
                        continue;
                    }
                    // FIXME: Would mean we missed a new candidacy
                    if vote.term > candidate.term {
                        //unreachable!();
                        continue;
                    }
                    println!(
                        "[{}] [{}] candidate {} notified {} voted for {}",
                        candidate.time, candidate.term, candidate.id, vote.elector, vote.candidate,
                    );
                    if vote.candidate == candidate.id {
                        candidate.votes += 1;
                    }
                }
            }
        }

        if candidate.votes > (candidate.nodes.len() / 2) as u64 {
            println!(
                "[{}] [{}] candidate {} became leader with {}/{} votes",
                candidate.time,
                candidate.term,
                candidate.id,
                candidate.votes,
                candidate.nodes.len()
            );
            let leader = LeaderNode {
                id: candidate.id,
                time: candidate.time,
                term: candidate.term,
                nodes: candidate.nodes.clone(),
                last_sent_heartbeat: 0,
            };
            return Ok(Node::Leader(leader));
        }

        Ok(Node::Candidate(candidate.clone()))
    }

    fn leader_step(&mut self, leader: &mut LeaderNode) -> Result<Node, ()> {
        leader.time += 1;

        if leader.time >= leader.last_sent_heartbeat + ELECTION_TIMEOUT/2 {
            println!(
                "[{}] [{}] leader {} heartbeating",
                leader.time, leader.term, leader.id,
            );
            self.outbox.push_back(Message::Heartbeat(HeartbeatMessage {
                term: leader.term,
                from: leader.id,
                nodes: leader.nodes.clone(),
            }));
            leader.last_sent_heartbeat = leader.time;
        }

        Ok(Node::Leader(leader.clone()))
    }
}
