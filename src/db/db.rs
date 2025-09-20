use eyre::Result;
use rusqlite::{Connection, params};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use alloy_primitives::{Address, B256 as H256};
use log::{info, error};

use crate::reth::HashChanEvent;

// Helper functions to convert types to bytes
fn h256_to_bytes(h: &H256) -> Vec<u8> {
    h.to_vec()
}

fn address_to_bytes(addr: &Address) -> Vec<u8> {
    addr.to_vec()
}

// Struct definitions for database tables
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

pub struct HashChanDB {
    conn: Connection,
}

impl HashChanDB {
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // Create tables if they don't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS boards (
                board_id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                symbol TEXT NOT NULL,
                description TEXT,
                banner_url TEXT,
                banner_cid TEXT,
                timestamp INTEGER NOT NULL,
                block_number INTEGER NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS threads (
                thread_id BLOB PRIMARY KEY,
                board_id INTEGER NOT NULL,
                creator BLOB NOT NULL,
                title TEXT NOT NULL,
                content TEXT,
                img_url TEXT,
                img_cid TEXT,
                timestamp INTEGER NOT NULL,
                block_number INTEGER NOT NULL,
                FOREIGN KEY (board_id) REFERENCES boards (board_id)
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS posts (
                post_id BLOB PRIMARY KEY,
                thread_id BLOB NOT NULL,
                board_id INTEGER NOT NULL,
                creator BLOB NOT NULL,
                content TEXT,
                img_url TEXT,
                img_cid TEXT,
                timestamp INTEGER NOT NULL,
                block_number INTEGER NOT NULL,
                FOREIGN KEY (thread_id) REFERENCES threads (thread_id),
                FOREIGN KEY (board_id) REFERENCES boards (board_id)
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS post_replies (
                post_id BLOB NOT NULL,
                reply_to_id BLOB NOT NULL,
                PRIMARY KEY (post_id, reply_to_id),
                FOREIGN KEY (post_id) REFERENCES posts (post_id),
                FOREIGN KEY (reply_to_id) REFERENCES posts (post_id)
            )",
            [],
        )?;
        
        // Create indexes for better query performance
        conn.execute("CREATE INDEX IF NOT EXISTS idx_threads_board_id ON threads (board_id)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_posts_thread_id ON posts (thread_id)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_posts_board_id ON posts (board_id)", [])?;
        
        Ok(Self { conn })
    }
    
    pub fn from_data_dir(data_dir: &Path) -> Result<Self> {
        let db_path = data_dir.join("hashchan.db");
        Self::new(&db_path)
    }
    
    // Initialize the SQLite database
    pub fn initialize() -> Result<Self, eyre::Error> {
        // Get the data directory
        let app_data_dir = Self::get_app_data_dir()?;
        info!("Using data directory: {:?}", app_data_dir);
        
        // Create the database connection
        let db = Self::from_data_dir(&app_data_dir)?;
        info!("Database connection established");
        
        // Run migrations (tables are created in the HashChanDB::new method)
        info!("Database schema initialized");
        
        Ok(db)
    }
    
    // Get the application data directory
    pub fn get_app_data_dir() -> Result<PathBuf, eyre::Error> {
        let app_data_dir = if let Some(proj_dirs) = directories::ProjectDirs::from("com", "hashchan", "node") {
            proj_dirs.data_dir().to_path_buf()
        } else {
            // Fallback to a local directory if we can't get the project directory
            PathBuf::from("data")
        };
        
        // Create the directory if it doesn't exist
        std::fs::create_dir_all(&app_data_dir)?;
        
        Ok(app_data_dir)
    }
    
    pub fn store_event(&self, event: &HashChanEvent, block_number: u64) -> Result<()> {
        match event {
            HashChanEvent::NewBoard { 
                board_id, 
                name, 
                symbol, 
                description, 
                banner_url, 
                banner_cid, 
                timestamp 
            } => {
                self.conn.execute(
                    "INSERT OR REPLACE INTO boards 
                    (board_id, name, symbol, description, banner_url, banner_cid, timestamp, block_number) 
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    params![
                        *board_id as i64,
                        name,
                        symbol,
                        description,
                        banner_url,
                        banner_cid,
                        *timestamp as i64,
                        block_number as i64
                    ],
                )?;
            },
            HashChanEvent::NewThread { 
                board_id, 
                thread_id, 
                creator, 
                title, 
                content, 
                img_url, 
                img_cid, 
                timestamp 
            } => {
                self.conn.execute(
                    "INSERT OR REPLACE INTO threads 
                    (thread_id, board_id, creator, title, content, img_url, img_cid, timestamp, block_number) 
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    params![
                        h256_to_bytes(thread_id),
                        *board_id as i64,
                        address_to_bytes(creator),
                        title,
                        content,
                        img_url,
                        img_cid,
                        *timestamp as i64,
                        block_number as i64
                    ],
                )?;
            },
            HashChanEvent::NewPost { 
                board_id, 
                thread_id, 
                post_id, 
                creator, 
                content, 
                img_url, 
                img_cid, 
                timestamp,
                reply_ids
            } => {
                // Insert the post
                self.conn.execute(
                    "INSERT OR REPLACE INTO posts 
                    (post_id, thread_id, board_id, creator, content, img_url, img_cid, timestamp, block_number) 
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    params![
                        h256_to_bytes(post_id),
                        h256_to_bytes(thread_id),
                        *board_id as i64,
                        address_to_bytes(creator),
                        content,
                        img_url,
                        img_cid,
                        *timestamp as i64,
                        block_number as i64
                    ],
                )?;
                
                // Insert reply relationships if any
                if let Some(replies) = reply_ids {
                    for reply_id in replies {
                        self.conn.execute(
                            "INSERT OR IGNORE INTO post_replies (post_id, reply_to_id) VALUES (?1, ?2)",
                            params![h256_to_bytes(post_id), h256_to_bytes(reply_id)],
                        )?;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    // Get stats about indexed data
    pub fn get_stats(&self) -> Result<(i64, i64, i64)> {
        let board_count: i64 = self.conn.query_row("SELECT COUNT(*) FROM boards", [], |row| row.get(0))?;
        let thread_count: i64 = self.conn.query_row("SELECT COUNT(*) FROM threads", [], |row| row.get(0))?;
        let post_count: i64 = self.conn.query_row("SELECT COUNT(*) FROM posts", [], |row| row.get(0))?;
        
        Ok((board_count, thread_count, post_count))
    }
    
    // Get all boards
    pub fn get_boards(&self) -> Result<Vec<Board>> {
        let mut stmt = self.conn.prepare("SELECT board_id, name, symbol, description, banner_url, banner_cid, timestamp, block_number FROM boards")?;
        let board_iter = stmt.query_map([], |row| {
            Ok(Board {
                board_id: row.get(0)?,
                name: row.get(1)?,
                symbol: row.get(2)?,
                description: row.get(3)?,
                banner_url: row.get(4)?,
                banner_cid: row.get(5)?,
                timestamp: row.get(6)?,
                block_number: row.get(7)?,
            })
        })?;
        
        let mut boards = Vec::new();
        for board in board_iter {
            boards.push(board?);
        }
        
        Ok(boards)
    }
    
    // Get threads for a specific board
    pub fn get_board_threads(&self, board_id: i64) -> Result<Vec<Thread>> {
        let mut stmt = self.conn.prepare(
            "SELECT thread_id, board_id, creator, title, content, img_url, img_cid, timestamp, block_number 
             FROM threads WHERE board_id = ?1"
        )?;
        
        let thread_iter = stmt.query_map([board_id], |row| {
            Ok(Thread {
                thread_id: row.get(0)?,
                board_id: row.get(1)?,
                creator: row.get(2)?,
                title: row.get(3)?,
                content: row.get(4)?,
                img_url: row.get(5)?,
                img_cid: row.get(6)?,
                timestamp: row.get(7)?,
                block_number: row.get(8)?,
            })
        })?;
        
        let mut threads = Vec::new();
        for thread in thread_iter {
            threads.push(thread?);
        }
        
        Ok(threads)
    }
    
    // Get posts for a specific thread
    pub fn get_thread_posts(&self, thread_id_bytes: &[u8]) -> Result<Vec<Post>> {
        let mut stmt = self.conn.prepare(
            "SELECT post_id, thread_id, board_id, creator, content, img_url, img_cid, timestamp, block_number 
             FROM posts WHERE thread_id = ?1"
        )?;
        
        let post_iter = stmt.query_map([thread_id_bytes], |row| {
            Ok(Post {
                post_id: row.get(0)?,
                thread_id: row.get(1)?,
                board_id: row.get(2)?,
                creator: row.get(3)?,
                content: row.get(4)?,
                img_url: row.get(5)?,
                img_cid: row.get(6)?,
                timestamp: row.get(7)?,
                block_number: row.get(8)?,
            })
        })?;
        
        let mut posts = Vec::new();
        for post in post_iter {
            posts.push(post?);
        }
        
        Ok(posts)
    }
}