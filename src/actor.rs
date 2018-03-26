use super::*;
use rand::RngCore;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Actor {
    pub node: Node,
    pub role: Role,
    pub inbox: Vec<Message>,
    pub outbox: Vec<Message>,
}

impl Actor {
    pub fn new(node: Node, role: Role) -> Actor {
        Actor {
            node,
            role,
            inbox: vec![],
            outbox: vec![],
        }
    }

    pub fn poll(&mut self, rng: &mut RngCore) -> Result<(), Error> {
        self.node.time += 1;

        let in_msgs: Vec<_> = self.inbox.drain(..).collect();
        for in_msg in in_msgs {
            let (new_role, out_msgs) = match self.role.clone() {
                Role::Idler => idler::process_msg(in_msg, &mut self.node, rng),
                Role::Voter(voter_) => voter::process_msg(in_msg, &mut self.node, voter_, rng),
                Role::Follower(follower_) => {
                    follower::process_msg(in_msg, &mut self.node, follower_, rng)
                }
                Role::Candidate(candidate_) => {
                    candidate::process_msg(in_msg, &mut self.node, candidate_, rng)
                }
                Role::Leader(leader_) => leader::process_msg(in_msg, &mut self.node, leader_, rng),
            };
            self.role = new_role;
            self.outbox.extend(out_msgs.into_iter());
        }

        let (new_role, out_msgs) = match self.role.clone() {
            Role::Idler => idler::poll(&mut self.node, rng),
            Role::Voter(voter_) => voter::poll(&mut self.node, voter_, rng),
            Role::Follower(follower_) => follower::poll(&mut self.node, follower_, rng),
            Role::Candidate(candidate_) => candidate::poll(&mut self.node, candidate_, rng),
            Role::Leader(leader_) => leader::poll(&mut self.node, leader_, rng),
        };
        self.role = new_role;
        self.outbox.extend(out_msgs.into_iter());
        Ok(())
    }
}
