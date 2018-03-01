use super::*;
use rand::RngCore;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaderActor {
    pub leader: Leader,
}

impl LeaderActor {
    pub fn act(
        mut self,
        _inbox: &mut MessageQueue,
        outbox: &mut MessageQueue,
        _rng: &mut RngCore,
    ) -> Result<Actor, Error> {
        self.leader.time += 1;

        if !self.time_for_heartbeat() {
            return Ok(Actor::Leader(self));
        }

        outbox.push_back(Message::Heartbeat(HeartbeatMessage {
            term: self.leader.term,
            from: self.leader.id,
            nodes: self.leader.nodes.clone(),
        }));
        self.leader.last_sent_heartbeat = self.leader.time;
        println!(
            "[time {}] [id {}] [term {}] leader sent heartbeat",
            self.leader.time, self.leader.id, self.leader.term,
        );
        Ok(Actor::Leader(self))
    }

    fn time_for_heartbeat(&self) -> bool {
        self.leader.time >= self.leader.last_sent_heartbeat + ELECTION_TIMEOUT / 2
    }
}
