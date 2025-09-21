use deadpool_postgres::{Config, Pool, PoolError, Runtime};
use tokio_postgres::{NoTls, Error as PgError};
use eyre::Result;
use std::str::FromStr;
use log::{info, error};
use serde::{Serialize, Deserialize};
use tokio_postgres::Row;

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("Database pool error: {0}")]
    PoolError(#[from] PoolError),
    
    #[error("Database error: {0}")]
    PostgresError(#[from] PgError),
    
    #[error("No data found")]
    NotFound,
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub struct DbClient {
    pool: Pool,
}

impl DbClient {
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("Setting up database connection pool for UI");
        
        // Parse the connection string
        let config = Config::from_str(database_url)
            .map_err(|e| eyre::eyre!("Failed to parse database URL: {}", e))?;
        
        // Create a connection pool
        let pool = config.create_pool(Some(Runtime::Tokio1), NoTls)
            .map_err(|e| eyre::eyre!("Failed to create connection pool: {}", e))?;
        
        // Test the connection
        let client = pool.get().await
            .map_err(|e| eyre::eyre!("Failed to get database connection: {}", e))?;
        
        client.execute("SELECT 1", &[]).await
            .map_err(|e| eyre::eyre!("Failed to execute test query: {}", e))?;
        
        info!("Database connection pool established successfully for UI");
        
        Ok(Self { pool })
    }
    
    // Query methods for UI
    pub async fn get_boards(&self) -> Result<Vec<Board>, DbError> {
        let client = self.pool.get().await?;
        
        let rows = client.query(
            "SELECT * FROM boards ORDER BY board_id",
            &[],
        ).await?;
        
        let boards = rows.into_iter()
            .map(Board::from)
            .collect();
        
        Ok(boards)
    }
    
    pub async fn get_threads(&self, board_id: i64) -> Result<Vec<Thread>, DbError> {
        let client = self.pool.get().await?;
        
        let rows = client.query(
            "SELECT * FROM threads WHERE board_id = $1 ORDER BY timestamp DESC",
            &[&board_id],
        ).await?;
        
        let threads = rows.into_iter()
            .map(Thread::from)
            .collect();
        
        Ok(threads)
    }
    
    pub async fn get_posts(&self, thread_id: &[u8]) -> Result<Vec<Post>, DbError> {
        let client = self.pool.get().await?;
        
        let rows = client.query(
            "SELECT * FROM posts WHERE thread_id = $1 ORDER BY timestamp ASC",
            &[&thread_id],
        ).await?;
        
        let posts = rows.into_iter()
            .map(Post::from)
            .collect();
        
        Ok(posts)
    }
    
    pub async fn get_stats(&self) -> Result<(i64, i64, i64), DbError> {
        let client = self.pool.get().await?;
        
        let board_count: i64 = client.query_one("SELECT COUNT(*) FROM boards", &[])
            .await?
            .get(0);
            
        let thread_count: i64 = client.query_one("SELECT COUNT(*) FROM threads", &[])
            .await?
            .get(0);
            
        let post_count: i64 = client.query_one("SELECT COUNT(*) FROM posts", &[])
            .await?
            .get(0);
            
        Ok((board_count, thread_count, post_count))
    }
}

// Data models
#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl From<Row> for Board {
    fn from(row: Row) -> Self {
        Self {
            board_id: row.get("board_id"),
            name: row.get("name"),
            symbol: row.get("symbol"),
            description: row.get("description"),
            banner_url: row.get("banner_url"),
            banner_cid: row.get("banner_cid"),
            timestamp: row.get("timestamp"),
            block_number: row.get("block_number"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl From<Row> for Thread {
    fn from(row: Row) -> Self {
        Self {
            thread_id: row.get("thread_id"),
            board_id: row.get("board_id"),
            creator: row.get("creator"),
            title: row.get("title"),
            content: row.get("content"),
            img_url: row.get("img_url"),
            img_cid: row.get("img_cid"),
            timestamp: row.get("timestamp"),
            block_number: row.get("block_number"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl From<Row> for Post {
    fn from(row: Row) -> Self {
        Self {
            post_id: row.get("post_id"),
            thread_id: row.get("thread_id"),
            board_id: row.get("board_id"),
            creator: row.get("creator"),
            content: row.get("content"),
            img_url: row.get("img_url"),
            img_cid: row.get("img_cid"),
            timestamp: row.get("timestamp"),
            block_number: row.get("block_number"),
        }
    }
}
