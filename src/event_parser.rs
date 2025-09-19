use ethabi::{Event, EventParam, ParamType, Token};
use alloy_primitives::{Address, U256, B256 as H256};
// use reth::primitives::Log;  // Commented out due to compatibility issues
use crate::HashChanEvent;

// Define event signatures
pub const NEW_BOARD_SIG: &str = "0xccb9b1cd83f22fb2f6344df2f5953739f1b38df363adf90e0cf2e0b153e5bae3";
pub const NEW_THREAD_SIG: &str = "0xe3d9c787ff9a2c8c020bf81ee081af0ab9a252c80b41af438db2e09a0489ba99";
pub const NEW_POST_SIG: &str = "0x5cdd9da62c61eaafed9065aca74de82e597f04385ead6ad0574b510eaafb5b74";

// Create ethabi Event objects for each event type
fn new_board_event() -> Event {
    Event {
        name: "NewBoard".to_string(),
        inputs: vec![
            EventParam {
                name: "boardId".to_string(),
                kind: ParamType::Uint(256),
                indexed: true,
            },
            EventParam {
                name: "name".to_string(),
                kind: ParamType::String,
                indexed: false,
            },
            EventParam {
                name: "symbol".to_string(),
                kind: ParamType::String,
                indexed: false,
            },
            EventParam {
                name: "description".to_string(),
                kind: ParamType::String,
                indexed: false,
            },
            EventParam {
                name: "bannerUrl".to_string(),
                kind: ParamType::String,
                indexed: false,
            },
            EventParam {
                name: "bannerCID".to_string(),
                kind: ParamType::String,
                indexed: false,
            },
            EventParam {
                name: "rules".to_string(),
                kind: ParamType::Array(Box::new(ParamType::String)),
                indexed: false,
            },
            EventParam {
                name: "timestamp".to_string(),
                kind: ParamType::Uint(256),
                indexed: false,
            },
        ],
        anonymous: false,
    }
}

fn new_thread_event() -> Event {
    Event {
        name: "NewThread".to_string(),
        inputs: vec![
            EventParam {
                name: "boardId".to_string(),
                kind: ParamType::Uint(256),
                indexed: true,
            },
            EventParam {
                name: "threadId".to_string(),
                kind: ParamType::FixedBytes(32),
                indexed: true,
            },
            EventParam {
                name: "creator".to_string(),
                kind: ParamType::Address,
                indexed: true,
            },
            EventParam {
                name: "imgUrl".to_string(),
                kind: ParamType::String,
                indexed: false,
            },
            EventParam {
                name: "imgCID".to_string(),
                kind: ParamType::String,
                indexed: false,
            },
            EventParam {
                name: "title".to_string(),
                kind: ParamType::String,
                indexed: false,
            },
            EventParam {
                name: "content".to_string(),
                kind: ParamType::String,
                indexed: false,
            },
            EventParam {
                name: "timestamp".to_string(),
                kind: ParamType::Uint(256),
                indexed: false,
            },
        ],
        anonymous: false,
    }
}

fn new_post_event() -> Event {
    Event {
        name: "NewPost".to_string(),
        inputs: vec![
            EventParam {
                name: "boardId".to_string(),
                kind: ParamType::Uint(256),
                indexed: false,
            },
            EventParam {
                name: "threadId".to_string(),
                kind: ParamType::FixedBytes(32),
                indexed: true,
            },
            EventParam {
                name: "postId".to_string(),
                kind: ParamType::FixedBytes(32),
                indexed: true,
            },
            EventParam {
                name: "replyIds".to_string(),
                kind: ParamType::Array(Box::new(ParamType::FixedBytes(32))),
                indexed: false,
            },
            EventParam {
                name: "creator".to_string(),
                kind: ParamType::Address,
                indexed: true,
            },
            EventParam {
                name: "imgUrl".to_string(),
                kind: ParamType::String,
                indexed: false,
            },
            EventParam {
                name: "imgCID".to_string(),
                kind: ParamType::String,
                indexed: false,
            },
            EventParam {
                name: "content".to_string(),
                kind: ParamType::String,
                indexed: false,
            },
            EventParam {
                name: "timestamp".to_string(),
                kind: ParamType::Uint(256),
                indexed: false,
            },
        ],
        anonymous: false,
    }
}

/* 
// Convert reth Log to ethabi RawLog - Commented out due to compatibility issues
fn convert_to_raw_log(log: &Log) -> RawLog {
    let topics: Vec<_> = log.topics.iter().cloned().collect();
    RawLog {
        topics,
        data: log.data.clone(),
    }
}

// Parse NewBoard event - Commented out due to compatibility issues
pub fn parse_board_event(log: &Log) -> Option<HashChanEvent> {
    let event = new_board_event();
    let raw_log = convert_to_raw_log(log);
    
    match event.parse_log(raw_log) {
        Ok(log) => {
            let mut board_id = 0;
            let mut name = String::new();
            let mut symbol = String::new();
            let mut description = String::new();
            let mut banner_url = String::new();
            let mut banner_cid = String::new();
            let mut timestamp = 0;
            
            for param in log.params {
                match param.name.as_str() {
                    "boardId" => {
                        if let Token::Uint(val) = param.value {
                            board_id = val.low_u64();
                        }
                    },
                    "name" => {
                        if let Token::String(val) = param.value {
                            name = val;
                        }
                    },
                    "symbol" => {
                        if let Token::String(val) = param.value {
                            symbol = val;
                        }
                    },
                    "description" => {
                        if let Token::String(val) = param.value {
                            description = val;
                        }
                    },
                    "bannerUrl" => {
                        if let Token::String(val) = param.value {
                            banner_url = val;
                        }
                    },
                    "bannerCID" => {
                        if let Token::String(val) = param.value {
                            banner_cid = val;
                        }
                    },
                    "timestamp" => {
                        if let Token::Uint(val) = param.value {
                            timestamp = val.low_u64();
                        }
                    },
                    _ => {}
                }
            }
            
            Some(HashChanEvent::NewBoard {
                board_id,
                name,
                symbol,
                description,
                banner_url,
                banner_cid,
                timestamp,
            })
        },
        Err(_) => {
            // Fallback to simple parsing if ethabi parsing fails
            if log.topics.len() > 1 {
                let board_id = U256::from_big_endian(&log.topics[1].0).low_u64();
                Some(HashChanEvent::NewBoard {
                    board_id,
                    name: format!("Board #{}", board_id),
                    symbol: String::new(),
                    description: String::new(),
                    banner_url: String::new(),
                    banner_cid: String::new(),
                    timestamp: 0,
                })
            } else {
                None
            }
        }
    }
}

// Parse NewThread event - Commented out due to compatibility issues
pub fn parse_thread_event(log: &Log) -> Option<HashChanEvent> {
    let event = new_thread_event();
    let raw_log = convert_to_raw_log(log);
    
    match event.parse_log(raw_log) {
        Ok(log) => {
            let mut board_id = 0;
            let mut thread_id = H256::default();
            let mut creator = Address::default();
            let mut img_url = String::new();
            let mut img_cid = String::new();
            let mut title = String::new();
            let mut content = String::new();
            let mut timestamp = 0;
            
            for param in log.params {
                match param.name.as_str() {
                    "boardId" => {
                        if let Token::Uint(val) = param.value {
                            board_id = val.low_u64();
                        }
                    },
                    "threadId" => {
                        if let Token::FixedBytes(val) = param.value {
                            if val.len() == 32 {
                                thread_id = H256::from_slice(&val);
                            }
                        }
                    },
                    "creator" => {
                        if let Token::Address(val) = param.value {
                            creator = Address::from_slice(&val.0);
                        }
                    },
                    "imgUrl" => {
                        if let Token::String(val) = param.value {
                            img_url = val;
                        }
                    },
                    "imgCID" => {
                        if let Token::String(val) = param.value {
                            img_cid = val;
                        }
                    },
                    "title" => {
                        if let Token::String(val) = param.value {
                            title = val;
                        }
                    },
                    "content" => {
                        if let Token::String(val) = param.value {
                            content = val;
                        }
                    },
                    "timestamp" => {
                        if let Token::Uint(val) = param.value {
                            timestamp = val.low_u64();
                        }
                    },
                    _ => {}
                }
            }
            
            Some(HashChanEvent::NewThread {
                board_id,
                thread_id,
                creator,
                img_url,
                img_cid,
                title,
                content,
                timestamp,
            })
        },
        Err(_) => {
            // Fallback to simple parsing if ethabi parsing fails
            if log.topics.len() > 2 {
                let board_id = U256::from_be_bytes(log.topics()[1].to_fixed_bytes()).low_u64();
                let thread_id = log.topics()[2];
                let creator = if log.topics().len() > 3 {
                    Address::from_slice(&log.topics()[3].as_fixed_bytes()[12..32])
                } else {
                    Address::default()
                };
                
                Some(HashChanEvent::NewThread {
                    board_id,
                    thread_id,
                    creator,
                    img_url: String::new(),
                    img_cid: String::new(),
                    title: format!("Thread in board #{}", board_id),
                    content: String::new(),
                    timestamp: 0,
                })
            } else {
                None
            }
        }
    }
}

// Parse NewPost event - Commented out due to compatibility issues
pub fn parse_post_event(log: &Log) -> Option<HashChanEvent> {
    let event = new_post_event();
    let raw_log = convert_to_raw_log(log);
    
    match event.parse_log(raw_log) {
        Ok(log) => {
            let mut board_id = 0;
            let mut thread_id = H256::default();
            let mut post_id = H256::default();
            let mut creator = Address::default();
            let mut img_url = String::new();
            let mut img_cid = String::new();
            let mut content = String::new();
            let mut timestamp = 0;
            let mut reply_ids = None;
            
            for param in log.params {
                match param.name.as_str() {
                    "boardId" => {
                        if let Token::Uint(val) = param.value {
                            board_id = val.low_u64();
                        }
                    },
                    "threadId" => {
                        if let Token::FixedBytes(val) = param.value {
                            if val.len() == 32 {
                                thread_id = H256::from_slice(&val);
                            }
                        }
                    },
                    "postId" => {
                        if let Token::FixedBytes(val) = param.value {
                            if val.len() == 32 {
                                post_id = H256::from_slice(&val);
                            }
                        }
                    },
                    "replyIds" => {
                        if let Token::Array(vals) = param.value {
                            let mut ids = Vec::new();
                            for val in vals {
                                if let Token::FixedBytes(bytes) = val {
                                    if bytes.len() == 32 {
                                        ids.push(H256::from_slice(&bytes));
                                    }
                                }
                            }
                            if !ids.is_empty() {
                                reply_ids = Some(ids);
                            }
                        }
                    },
                    "creator" => {
                        if let Token::Address(val) = param.value {
                            creator = Address::from_slice(&val.0);
                        }
                    },
                    "imgUrl" => {
                        if let Token::String(val) = param.value {
                            img_url = val;
                        }
                    },
                    "imgCID" => {
                        if let Token::String(val) = param.value {
                            img_cid = val;
                        }
                    },
                    "content" => {
                        if let Token::String(val) = param.value {
                            content = val;
                        }
                    },
                    "timestamp" => {
                        if let Token::Uint(val) = param.value {
                            timestamp = val.low_u64();
                        }
                    },
                    _ => {}
                }
            }
            
            Some(HashChanEvent::NewPost {
                board_id,
                thread_id,
                post_id,
                creator,
                img_url,
                img_cid,
                content,
                timestamp,
                reply_ids,
            })
        },
        Err(_) => {
            // Fallback to simple parsing if ethabi parsing fails
            if log.topics.len() > 2 {
                let thread_id = log.topics[1];
                let post_id = log.topics[2];
                let creator = if log.topics.len() > 3 {
                    Address::from_slice(&log.topics[3].0[12..])
                } else {
                    Address::default()
                };
                
                Some(HashChanEvent::NewPost {
                    board_id: 0, // Can't determine from indexed params
                    thread_id,
                    post_id,
                    creator,
                    img_url: String::new(),
                    img_cid: String::new(),
                    content: format!("Post in thread {:?}", thread_id),
                    timestamp: 0,
                    reply_ids: None,
                })
            } else {
                None
            }
        }
    }
}
*/

// Placeholder functions for compile-time compatibility
pub fn parse_board_event(_log: &impl std::fmt::Debug) -> Option<HashChanEvent> {
    None
}

pub fn parse_thread_event(_log: &impl std::fmt::Debug) -> Option<HashChanEvent> {
    None
}

pub fn parse_post_event(_log: &impl std::fmt::Debug) -> Option<HashChanEvent> {
    None
}