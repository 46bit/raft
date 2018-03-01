mod follower;
pub use self::follower::*;
mod candidate;
pub use self::candidate::*;
mod leader;
pub use self::leader::*;

use super::*;
use rand::RngCore;

// How long to wait for a master's heartbeat before starting an election.
pub const ELECTION_TIMEOUT: Time = 150;
// How long between sending heartbeats
pub const HEARTBEAT_PERIOD: Time = 15;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Actor {
    Follower(FollowerActor),
    Candidate(CandidateActor),
    Leader(LeaderActor),
}

impl Actor {
    pub fn act(
        self,
        inbox: &mut MessageQueue,
        outbox: &mut MessageQueue,
        rng: &mut RngCore,
    ) -> Result<Actor, Error> {
        match self {
            Actor::Follower(follower_actor) => follower_actor.act(inbox, outbox, rng),
            Actor::Candidate(candidate_actor) => candidate_actor.act(inbox, outbox, rng),
            Actor::Leader(leader_actor) => leader_actor.act(inbox, outbox, rng),
        }
    }
}
