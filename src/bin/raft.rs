#![feature(box_syntax)]

extern crate raft;
extern crate rand;

use raft::*;
use rand::Rng;
use std::thread;
use std::time::Duration;

fn new_follower_actor(id: Id, nodes: Vec<Id>) -> Actor {
    Actor::Follower(FollowerActor {
        follower: Follower {
            id: id,
            time: 0,
            term: 0,
            nodes: nodes,
            last_recv_heartbeat: 0,
            voted: None,
        },
    })
}

fn main() {
    let mut rng = rand::thread_rng();
    let actor_count = 5;
    let mut actors: Vec<Actor> = (0..actor_count)
        .map(|_| new_follower_actor(0, (0..actor_count).collect()))
        .collect();
    let mut inboxes: Vec<_> = actors.iter().map(|_| MessageQueue::new()).collect();
    for k in 0.. {
        let mut outboxes: Vec<_> = actors.iter().map(|_| MessageQueue::new()).collect();
        for i in 0..actors.len() {
            let mut inbox = &mut inboxes[i];
            let mut outbox = &mut outboxes[i];
            let actor = actors.remove(i);
            actors.insert(i, actor.act(inbox, outbox, &mut rng).unwrap());
        }
        for (i, outbox) in outboxes.into_iter().enumerate() {
            for (j, inbox) in inboxes.iter_mut().enumerate() {
                if i == j {
                    continue;
                }
                for out_msg in &outbox {
                    inbox.push_back(out_msg.clone());
                }
            }
        }
        //thread::sleep(Duration::from_millis(1));
        if k % 400 == 399 {
            let l = rng.gen_range(0, actors.len());
            println!("reset {} {:?}", l, actors[l]);
            actors[l] = new_follower_actor(l as u64, (0..actor_count).collect());
        }
    }
}
