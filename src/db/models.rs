use diesel::prelude::*;
use diesel::AsChangeset;
use alloy_primitives::{Address, B256 as H256};

use crate::HashChanEvent;

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = super::schema::boards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(board_id))]
pub struct Board {
    pub board_id: i64,
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub banner_url: Option<String>,
    pub banner_cid: Option<String>,
    pub timestamp: i64,
    pub block_number: i64,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = super::schema::threads)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(thread_id))]
pub struct Thread {
    pub thread_id: Vec<u8>,
    pub board_id: i64,
    pub creator: Vec<u8>,
    pub title: String,
    pub content: Option<String>,
    pub img_url: Option<String>,
    pub img_cid: Option<String>,
    pub timestamp: i64,
    pub block_number: i64,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = super::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(post_id))]
pub struct Post {
    pub post_id: Vec<u8>,
    pub thread_id: Vec<u8>,
    pub board_id: i64,
    pub creator: Vec<u8>,
    pub content: Option<String>,
    pub img_url: Option<String>,
    pub img_cid: Option<String>,
    pub timestamp: i64,
    pub block_number: i64,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::post_replies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostReply {
    pub post_id: Vec<u8>,
    pub reply_to_id: Vec<u8>,
}

impl From<(&HashChanEvent, u64)> for Board {
    fn from((event, block_number): (&HashChanEvent, u64)) -> Self {
        match event {
            HashChanEvent::NewBoard { 
                board_id, 
                name, 
                symbol, 
                description, 
                banner_url, 
                banner_cid, 
                timestamp 
            } => Board {
                board_id: *board_id as i64,
                name: name.clone(),
                symbol: symbol.clone(),
                description: Some(description.clone()),
                banner_url: Some(banner_url.clone()),
                banner_cid: Some(banner_cid.clone()),
                timestamp: *timestamp as i64,
                block_number: block_number as i64,
            },
            _ => panic!("Cannot convert non-board event to Board"),
        }
    }
}

impl From<(&HashChanEvent, u64)> for Thread {
    fn from((event, block_number): (&HashChanEvent, u64)) -> Self {
        match event {
            HashChanEvent::NewThread { 
                board_id, 
                thread_id, 
                creator, 
                title, 
                content, 
                img_url, 
                img_cid, 
                timestamp 
            } => Thread {
                thread_id: thread_id.to_vec(),
                board_id: *board_id as i64,
                creator: creator.to_vec(),
                title: title.clone(),
                content: Some(content.clone()),
                img_url: Some(img_url.clone()),
                img_cid: Some(img_cid.clone()),
                timestamp: *timestamp as i64,
                block_number: block_number as i64,
            },
            _ => panic!("Cannot convert non-thread event to Thread"),
        }
    }
}

impl From<(&HashChanEvent, u64)> for Post {
    fn from((event, block_number): (&HashChanEvent, u64)) -> Self {
        match event {
            HashChanEvent::NewPost { 
                board_id, 
                thread_id, 
                post_id, 
                creator, 
                content, 
                img_url, 
                img_cid, 
                timestamp,
                .. // Ignoring reply_ids as they're handled separately
            } => Post {
                post_id: post_id.to_vec(),
                thread_id: thread_id.to_vec(),
                board_id: *board_id as i64,
                creator: creator.to_vec(),
                content: Some(content.clone()),
                img_url: Some(img_url.clone()),
                img_cid: Some(img_cid.clone()),
                timestamp: *timestamp as i64,
                block_number: block_number as i64,
            },
            _ => panic!("Cannot convert non-post event to Post"),
        }
    }
}
