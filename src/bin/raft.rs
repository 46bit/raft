#![feature(box_syntax)]

extern crate raft;
extern crate rand;

use raft::*;
use rand::Rng;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

fn id(i: u64) -> Id {
    format!("node.{}", i)
}

fn new_actor(id: Id, peers: HashSet<Id>) -> Actor {
    Actor::new(
        Node {
            id: id,
            time: 0,
            last_activity: 0,
            term: 0,
            peers: peers,
        },
        Idler { vote: None }.into(),
    )
}

fn main() {
    let mut rng = rand::thread_rng();
    let actor_count = 5;
    let node_ids: HashSet<Id> = (0..actor_count).map(id).collect();
    let mut actors: Vec<Actor> = (0..actor_count)
        .map(|i| new_actor(id(i), node_ids.clone()))
        .collect();
    assert_eq!(actor_count, actors.len() as u64);
    for k in 0.. {
        let actor_count = actors.len();
        for i in 0..actor_count {
            actors[i].poll(&mut rng).unwrap();
        }
        for i in 0..actor_count {
            let out_msgs: Vec<_> = actors[i].outbox.drain(..).collect();
            for out_msg in out_msgs {
                for j in 0..actor_count {
                    if i == j {
                        continue;
                    }
                    actors[j].inbox.push(out_msg.clone());
                }
            }
        }
        thread::sleep(Duration::from_millis(1));
        if k % 400 == 399 {
            let l = rng.gen_range(0, actors.len());
            print!("reset {} {:?} ", l, actors[l]);
            actors[l] = new_actor(id(l as u64), node_ids.clone());
            println!("into {:?}", actors[l]);
        }
    }
}
