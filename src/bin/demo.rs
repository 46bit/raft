extern crate rand;

use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    // How long to wait for a master's heartbeat before starting an election.
    election_timeout: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Follower {
        last_recv_heartbeat: u64,
        round: u64,
        voted: Option<u64>,
    },
    Candidate {
        round: u64,
        votes: u64,
    },
    Leader {
        last_sent_heartbeat: u64,
        round: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    state: State,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    variant: MessageVariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageVariant {
    Heartbeat(u64),
    Vote { for_: u64, by: u64 },
}

fn main() {
    let mut rng = rand::thread_rng();
    let election_timeout = 10;
    let mut nodes = vec![
        Node {
            state: State::Follower {
                last_recv_heartbeat: 0,
                round: 0,
                voted: None,
            },
        },
        Node {
            state: State::Follower {
                last_recv_heartbeat: 0,
                round: 0,
                voted: None,
            },
        },
        Node {
            state: State::Follower {
                last_recv_heartbeat: 0,
                round: 0,
                voted: None,
            },
        },
    ];
    let node_count = nodes.len() as u64;
    let mut inboxes: Vec<Vec<Message>> = vec![vec![], vec![], vec![]];
    for i in 1.. {
        //println!("{}", i);
        for (j, node) in nodes.iter_mut().enumerate() {
            node.state = match node.state {
                State::Follower {
                    mut last_recv_heartbeat,
                    round,
                    mut voted,
                } => {
                    let inbox = inboxes[j].clone();
                    for msg in inbox {
                        match msg.variant {
                            MessageVariant::Heartbeat(leader_id) => {
                                println!("{} got heartbeat from {}", j, leader_id);
                                last_recv_heartbeat = i;
                            }
                            MessageVariant::Vote { for_, by } => {
                                println!("{} notified that {} voted for {}", j, by, for_);
                                last_recv_heartbeat = i;
                                // FIXME: Base re-voting on round numbers only
                                // FIXME: Don't vote multiple times per round
                                if for_ == by && voted.is_none() {
                                    println!("{} voted for {}", j, for_);
                                    for (k, inbox) in inboxes.iter_mut().enumerate() {
                                        if j != k {
                                            let msg = Message {
                                                variant: MessageVariant::Vote {
                                                    for_: for_,
                                                    by: j as u64,
                                                },
                                            };
                                            inbox.push(msg);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    inboxes[j] = vec![];
                    let timeout = rng.gen_range(election_timeout, election_timeout * 2);
                    if last_recv_heartbeat + timeout < i {
                        println!("{} follower -> candidate", j);
                        for (k, inbox) in inboxes.iter_mut().enumerate() {
                            if j != k {
                                let msg = Message {
                                    variant: MessageVariant::Vote {
                                        for_: j as u64,
                                        by: j as u64,
                                    },
                                };
                                inbox.push(msg);
                            }
                        }
                        State::Candidate {
                            round: round + 1,
                            votes: 1,
                        }
                    } else {
                        State::Follower {
                            last_recv_heartbeat: last_recv_heartbeat,
                            round: round,
                            voted: voted,
                        }
                    }
                }
                State::Candidate { round, votes } => {
                    let mut s = State::Candidate {
                        round: round,
                        votes: votes,
                    };
                    if votes > node_count / 2 {
                        println!(
                            "candidate {:?} became leader with {}/{} votes",
                            j, votes, node_count
                        );
                        s = State::Leader {
                            last_sent_heartbeat: 0,
                            round: i,
                        };
                    } else {
                        let inbox_count = inboxes[j].len();
                        for _ in 0..inbox_count {
                            let msg = inboxes[j].remove(0);
                            match msg.variant {
                                MessageVariant::Heartbeat(leader_id) => {
                                    println!(
                                        "{} was candidate but became follower of {}",
                                        j, leader_id
                                    );
                                    s = State::Follower {
                                        last_recv_heartbeat: i as u64,
                                        round: i as u64,
                                        voted: None,
                                    };
                                    // FIXME: Finish processing messages, as a follower.
                                    break;
                                }
                                MessageVariant::Vote { for_, by } => {
                                    println!(
                                        "candidate {} notified of a vote for {} by {}",
                                        j, for_, by
                                    );
                                    if (j as u64) == for_ {
                                        s = State::Candidate {
                                            round: round,
                                            votes: votes + 1,
                                        };
                                        // FIXME: Finish adding up votes.
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    s
                }
                State::Leader {
                    last_sent_heartbeat,
                    round,
                } => {
                    if last_sent_heartbeat + election_timeout / 2 < i {
                        for (k, inbox) in inboxes.iter_mut().enumerate() {
                            if j != k {
                                let msg = Message {
                                    variant: MessageVariant::Heartbeat(j as u64),
                                };
                                inbox.push(msg);
                            }
                        }
                    }
                    State::Leader {
                        last_sent_heartbeat,
                        round,
                    }
                }
            };
        }
        if i == 100 {
            nodes[0] = Node {
                state: State::Follower {
                    last_recv_heartbeat: i - election_timeout,
                    round: i,
                    voted: None,
                },
            };
            nodes[1] = Node {
                state: State::Follower {
                    last_recv_heartbeat: i - election_timeout,
                    round: i,
                    voted: None,
                },
            };
            nodes[2] = Node {
                state: State::Follower {
                    last_recv_heartbeat: i - election_timeout,
                    round: i,
                    voted: None,
                },
            };
        }
    }
}
