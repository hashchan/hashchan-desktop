use serde::{Serialize, Deserialize};
use reqwest::Client;
use eyre::Result;

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

pub struct ApiClient {
    db_client: Client,
    reth_client: Client,
    db_base_url: String,
    reth_base_url: String,
}

impl ApiClient {
    pub fn new(db_base_url: &str, reth_base_url: &str) -> Self {
        Self {
            db_client: Client::new(),
            reth_client: Client::new(),
            db_base_url: db_base_url.to_string(),
            reth_base_url: reth_base_url.to_string(),
        }
    }
    
    pub async fn get_boards(&self) -> Result<Vec<Board>> {
        let url = format!("{}/boards", self.db_base_url);
        let response = self.db_client.get(&url).send().await?;
        let boards = response.json::<Vec<Board>>().await?;
        Ok(boards)
    }
    
    pub async fn get_threads(&self, board_id: i64) -> Result<Vec<Thread>> {
        let url = format!("{}/boards/{}/threads", self.db_base_url, board_id);
        let response = self.db_client.get(&url).send().await?;
        let threads = response.json::<Vec<Thread>>().await?;
        Ok(threads)
    }
    
    pub async fn get_posts(&self, thread_id: Vec<u8>) -> Result<Vec<Post>> {
        // Convert thread_id bytes to hex string for URL
        let thread_id_hex = hex::encode(&thread_id);
        let url = format!("{}/threads/{}/posts", self.db_base_url, thread_id_hex);
        let response = self.db_client.get(&url).send().await?;
        let posts = response.json::<Vec<Post>>().await?;
        Ok(posts)
    }
    
    // Add methods for interacting with Reth node via JSON-RPC
    pub async fn get_node_status(&self) -> Result<serde_json::Value> {
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_syncing",
            "params": []
        });
        
        let response = self.reth_client.post(&self.reth_base_url)
            .json(&payload)
            .send()
            .await?;
        
        let result = response.json::<serde_json::Value>().await?;
        Ok(result)
    }
}
