use deadpool_postgres::{Config, Pool, PoolError, Runtime};
use tokio_postgres::{NoTls, Error as PgError};
use eyre::Result;
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
        info!("Setting up database connection pool");
        
        // Parse the connection string and create config
        let mut config = Config::new();
        config.url = Some(database_url.to_string());
        
        // Create a connection pool
        let pool = config.create_pool(Some(Runtime::Tokio1), NoTls)
            .map_err(|e| eyre::eyre!("Failed to create connection pool: {}", e))?;
        
        // Test the connection
        let client = pool.get().await
            .map_err(|e| eyre::eyre!("Failed to get database connection: {}", e))?;
        
        client.execute("SELECT 1", &[]).await
            .map_err(|e| eyre::eyre!("Failed to execute test query: {}", e))?;
        
        info!("Database connection pool established successfully");
        
        Ok(Self { pool })
    }
    
    pub async fn verify_schema(&self) -> Result<()> {
        info!("Verifying database schema");
        
        let client = self.pool.get().await
            .map_err(|e| eyre::eyre!("Failed to get database connection: {}", e))?;
        
        // Check if tables exist
        let row = client.query_one(
            "SELECT EXISTS(SELECT 1 FROM information_schema.tables WHERE table_name = 'boards')",
            &[],
        ).await.map_err(|e| eyre::eyre!("Failed to check schema: {}", e))?;
        
        let tables_exist: bool = row.get(0);
        
        if !tables_exist {
            return Err(eyre::eyre!("Database schema not initialized. Please ensure the init.sql script was executed."));
        }
        
        info!("Database schema verified successfully");
        Ok(())
    }
    
    // Methods for storing blockchain data
    pub async fn store_board(&self, board: &Board) -> Result<(), DbError> {
        let client = self.pool.get().await?;
        
        client.execute(
            "INSERT INTO boards (board_id, name, symbol, description, banner_url, banner_cid, timestamp, block_number) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             ON CONFLICT (board_id) DO UPDATE SET
             name = EXCLUDED.name,
             symbol = EXCLUDED.symbol,
             description = EXCLUDED.description,
             banner_url = EXCLUDED.banner_url,
             banner_cid = EXCLUDED.banner_cid,
             timestamp = EXCLUDED.timestamp,
             block_number = EXCLUDED.block_number",
            &[
                &board.board_id,
                &board.name,
                &board.symbol,
                &board.description,
                &board.banner_url,
                &board.banner_cid,
                &board.timestamp,
                &board.block_number,
            ],
        ).await?;
        
        Ok(())
    }
    
    pub async fn store_thread(&self, thread: &Thread) -> Result<(), DbError> {
        let client = self.pool.get().await?;
        
        client.execute(
            "INSERT INTO threads (thread_id, board_id, creator, title, content, img_url, img_cid, timestamp, block_number) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             ON CONFLICT (thread_id) DO UPDATE SET
             board_id = EXCLUDED.board_id,
             creator = EXCLUDED.creator,
             title = EXCLUDED.title,
             content = EXCLUDED.content,
             img_url = EXCLUDED.img_url,
             img_cid = EXCLUDED.img_cid,
             timestamp = EXCLUDED.timestamp,
             block_number = EXCLUDED.block_number",
            &[
                &thread.thread_id,
                &thread.board_id,
                &thread.creator,
                &thread.title,
                &thread.content,
                &thread.img_url,
                &thread.img_cid,
                &thread.timestamp,
                &thread.block_number,
            ],
        ).await?;
        
        Ok(())
    }
    
    pub async fn store_post(&self, post: &Post, reply_ids: Option<&[Vec<u8>]>) -> Result<(), DbError> {
        let mut client = self.pool.get().await?;
        
        // Start a transaction
        let tx = client.transaction().await?;
        
        // Insert the post
        tx.execute(
            "INSERT INTO posts (post_id, thread_id, board_id, creator, content, img_url, img_cid, timestamp, block_number) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             ON CONFLICT (post_id) DO UPDATE SET
             thread_id = EXCLUDED.thread_id,
             board_id = EXCLUDED.board_id,
             creator = EXCLUDED.creator,
             content = EXCLUDED.content,
             img_url = EXCLUDED.img_url,
             img_cid = EXCLUDED.img_cid,
             timestamp = EXCLUDED.timestamp,
             block_number = EXCLUDED.block_number",
            &[
                &post.post_id,
                &post.thread_id,
                &post.board_id,
                &post.creator,
                &post.content,
                &post.img_url,
                &post.img_cid,
                &post.timestamp,
                &post.block_number,
            ],
        ).await?;
        
        // Insert reply relationships if any
        if let Some(replies) = reply_ids {
            for reply_id in replies {
                tx.execute(
                    "INSERT INTO post_replies (post_id, reply_to_id) VALUES ($1, $2)
                     ON CONFLICT (post_id, reply_to_id) DO NOTHING",
                    &[&post.post_id, &reply_id],
                ).await?;
            }
        }
        
        // Commit the transaction
        tx.commit().await?;
        
        Ok(())
    }
    
    // Query methods for retrieving data
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
