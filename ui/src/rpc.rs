use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use eyre::Result;
use log::{info, error};
use reqwest::Client;
use std::time::Duration;

#[derive(thiserror::Error, Debug)]
pub enum RpcError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("JSON-RPC error: {0}")]
    JsonRpcError(String),
    
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcRequest {
    jsonrpc: String,
    method: String,
    params: Vec<Value>,
    id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponse {
    jsonrpc: String,
    id: u64,
    #[serde(default)]
    result: Value,
    #[serde(default)]
    error: Option<RpcErrorResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcErrorResponse {
    code: i32,
    message: String,
}

pub struct RpcClient {
    client: Client,
    url: String,
    id: u64,
}

impl RpcClient {
    pub fn new(url: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();
        
        Self {
            client,
            url: url.to_string(),
            id: 1,
        }
    }
    
    pub async fn call(&mut self, method: &str, params: Vec<Value>) -> Result<Value, RpcError> {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: self.id,
        };
        
        self.id += 1;
        
        let response = self.client.post(&self.url)
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(RpcError::HttpError(
                reqwest::Error::from(reqwest::StatusCode::from_u16(response.status().as_u16()).unwrap())
            ));
        }
        
        let rpc_response: RpcResponse = response.json().await?;
        
        if let Some(error) = rpc_response.error {
            return Err(RpcError::JsonRpcError(format!(
                "Code {}: {}", error.code, error.message
            )));
        }
        
        Ok(rpc_response.result)
    }
    
    // Ethereum specific methods
    pub async fn eth_block_number(&mut self) -> Result<u64, RpcError> {
        let result = self.call("eth_blockNumber", vec![]).await?;
        
        let hex_string = result.as_str()
            .ok_or_else(|| RpcError::InvalidResponse("Expected string result".to_string()))?;
        
        let block_number = u64::from_str_radix(hex_string.trim_start_matches("0x"), 16)
            .map_err(|e| RpcError::InvalidResponse(format!("Invalid block number: {}", e)))?;
        
        Ok(block_number)
    }
    
    pub async fn eth_syncing(&mut self) -> Result<Value, RpcError> {
        self.call("eth_syncing", vec![]).await
    }
    
    pub async fn net_peer_count(&mut self) -> Result<u64, RpcError> {
        let result = self.call("net_peerCount", vec![]).await?;
        
        let hex_string = result.as_str()
            .ok_or_else(|| RpcError::InvalidResponse("Expected string result".to_string()))?;
        
        let peer_count = u64::from_str_radix(hex_string.trim_start_matches("0x"), 16)
            .map_err(|e| RpcError::InvalidResponse(format!("Invalid peer count: {}", e)))?;
        
        Ok(peer_count)
    }
}
