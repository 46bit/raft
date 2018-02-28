extern crate raft;
extern crate rand;

use raft::*;

fn main() {
    let mut nodes = vec![
        Node::Follower(FollowerNode {
            id: 0,
            time: 0,
            term: 0,
            nodes: vec![0, 1, 2],
            last_recv_heartbeat: 0,
            voted: None,
        }),
        Node::Follower(FollowerNode {
            id: 1,
            time: 0,
            term: 0,
            nodes: vec![0, 1, 2],
            last_recv_heartbeat: 0,
            voted: None,
        }),
        Node::Follower(FollowerNode {
            id: 2,
            time: 0,
            term: 0,
            nodes: vec![0, 1, 2],
            last_recv_heartbeat: 0,
            voted: None,
        }),
    ];
    let mut actors: Vec<_> = nodes.iter().map(|_| Actor::new()).collect();
    loop {
        for i in 0..nodes.len() {
            let mut node = &mut nodes[i];
            let mut actor = &mut actors[i];
            actor.step(&mut node).unwrap();
        }
        let mut msg_buffer: Vec<(usize, Message)> = vec![];
        for i in 0..nodes.len() {
            let mut actor = &mut actors[i];
            while let Some(out_msg) = actor.outbox.pop_front() {
                msg_buffer.push((i, out_msg));
            }
        }
        for (i, msg) in msg_buffer {
            for j in 0..nodes.len() {
                if i == j {
                    continue;
                }
                let mut actor = &mut actors[j];
                actor.inbox.push_back(msg.clone());
            }
        }
    }
}
