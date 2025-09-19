use std::{str::FromStr, sync::Arc, path::Path};
use std::{future::Future, pin::Pin, task::{ready, Context, Poll}};
use std::sync::atomic::{AtomicBool, Ordering};

use futures_util::{FutureExt, TryStreamExt};
use reth::{
    api::FullNodeComponents, 
    builder::NodeTypes, 
    primitives::EthPrimitives
};
use alloy_primitives::{Address, B256 as H256, BlockNumber};
use reth_exex::{ExExContext, ExExEvent, ExExNotification};
use reth_node_ethereum::EthereumNode;
use reth_tracing::tracing::{info, warn};

use crate::db::HashChanDB;

// HashChan3 event types
#[derive(Debug)]
pub enum HashChanEvent {
    NewBoard {
        board_id: u64,
        name: String,
        symbol: String,
        description: String,
        banner_url: String,
        banner_cid: String,
        timestamp: u64,
    },
    NewThread {
        board_id: u64,
        thread_id: H256,
        creator: Address,
        img_url: String,
        img_cid: String,
        title: String,
        content: String,
        timestamp: u64,
    },
    NewPost {
        board_id: u64,
        thread_id: H256,
        post_id: H256,
        creator: Address,
        img_url: String,
        img_cid: String,
        content: String,
        timestamp: u64,
        reply_ids: Option<Vec<H256>>,
    }
}

// Stateful ExEx struct for HashChan3 indexer
pub struct HashChanExEx<Node: FullNodeComponents> {
    ctx: ExExContext<Node>,
    // Contract address for HashChan3
    hashchan_address: Address,
    // Database connection
    db: Option<Arc<std::sync::Mutex<HashChanDB>>>,
    // First block that was committed since the start of the ExEx
    first_block: Option<BlockNumber>,
    // Total number of events processed
    board_events: u64,
    thread_events: u64,
    post_events: u64,
}

impl<Node: FullNodeComponents> HashChanExEx<Node> {
    pub fn new(ctx: ExExContext<Node>, db: Option<Arc<std::sync::Mutex<HashChanDB>>>) -> Self {
        // HashChan3 contract address
        let hashchan_address = match Address::from_str("0x458c27D5a6421AfAFF435e27E870584Fe03a938F") {
            Ok(addr) => addr,
            Err(e) => {
                warn!("Failed to parse HashChan3 contract address: {}", e);
                // Use default address as fallback
                Address::default()
            }
        };
        
        info!("Starting HashChan3 indexer for contract: {}", hashchan_address);
        info!("Database connection: {}", if db.is_some() { "available" } else { "unavailable" });
        
        Self {
            ctx,
            hashchan_address,
            db,
            first_block: None,
            board_events: 0,
            thread_events: 0,
            post_events: 0,
        }
    }
    
    // Process blocks for HashChan3 events
    // Using a generic parameter instead of BlockSource which isn't found
    /*
    fn process_committed_blocks<B>(&mut self, blocks: &B) -> eyre::Result<(u64, u64, u64)> 
    where B: reth::api::BlockNumReader + reth::api::BlockReader {
        // Implementation commented out for now
        Ok((0, 0, 0))
    }
    */
}

impl<Node: FullNodeComponents<Types: NodeTypes<Primitives = EthPrimitives>>> Future
    for HashChanExEx<Node>
{
    type Output = eyre::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        while let Some(notification) = ready!(this.ctx.notifications.try_next().poll_unpin(cx))? {
            match &notification {
                ExExNotification::ChainCommitted { new } => {
                    let range = new.range();
                    this.first_block.get_or_insert(range.start().clone());
                    
                    info!(
                        committed_chain = ?range,
                        from_block = *range.start(),
                        to_block = *range.end(),
                        "Received commit"
                    );
                    
                    // Process blocks for HashChan3 events - implementation commented out for now
                }
                ExExNotification::ChainReorged { old, new } => {
                    let old_range = old.range();
                    let new_range = new.range();
                    info!(
                        from_chain = ?old_range,
                        to_chain = ?new_range,
                        old_from = *old_range.start(),
                        old_to = *old_range.end(),
                        new_from = *new_range.start(),
                        new_to = *new_range.end(),
                        "Received reorg"
                    );
                    
                    // In a production system, you would:
                    // 1. Remove events from the old chain
                    // 2. Process events from the new chain
                }
                ExExNotification::ChainReverted { old } => {
                    let range = old.range();
                    info!(
                        reverted_chain = ?range,
                        from_block = *range.start(),
                        to_block = *range.end(),
                        "Received revert"
                    );
                    
                    // In a production system, you would:
                    // Remove events from the reverted blocks
                }
            };

            if let Some(committed_chain) = notification.committed_chain() {
                this.ctx.events.send(ExExEvent::FinishedHeight(committed_chain.tip().num_hash()))?;
            }
            
            // Log overall stats
            if let Some(first_block) = this.first_block {
                info!(
                    %first_block,
                    board_events = %this.board_events,
                    thread_events = %this.thread_events,
                    post_events = %this.post_events,
                    "Total HashChan3 events processed"
                );
            }
        }
        
        Poll::Ready(Ok(()))
    }
}

// Helper function to start the Reth node
pub fn start_reth_node(db: Arc<std::sync::Mutex<HashChanDB>>, shutdown_flag: Arc<AtomicBool>) -> eyre::Result<()> {
    // Create a data directory for Reth
    let data_dir = std::env::current_dir()?.join("data");
    std::fs::create_dir_all(&data_dir)?;
    
    // Create a vector of arguments to pass to the CLI
    let args = vec![
        "reth", 
        "node", 
        "--datadir", data_dir.to_str().unwrap_or("./data"),
        "--chain", "sepolia", // Use Sepolia testnet
        "--http",
        "--ws",
        "--log.stdout.filter", "info,reth=debug,hashchan_indexer=trace",
        "--verbosity" // Increase verbosity
    ];
    
    // Use try_parse_args_from to parse the arguments
    match reth::cli::Cli::try_parse_args_from(args) {
        Ok(cli) => {
            // Check if we should shut down before starting the node
            if shutdown_flag.load(Ordering::Relaxed) {
                info!("Shutdown flag set before starting Reth node, exiting");
                return Ok(());
            }
            
            // Spawn a thread to check for shutdown and send Ctrl+C signal if needed
            let shutdown_checker = shutdown_flag.clone();
            let shutdown_thread = std::thread::spawn(move || {
                while !shutdown_checker.load(Ordering::Relaxed) {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
                info!("Sending shutdown signal to Reth node");
                // Send a SIGINT signal to the process
                // This is equivalent to pressing Ctrl+C
                unsafe {
                    libc::raise(libc::SIGINT);
                }
            });
            
            let result = cli.run(
                async move |builder, _| {
                    let handle = builder
                        .node(EthereumNode::default())
                        .install_exex("hashchan_indexer", async move |ctx| {
                            Ok(HashChanExEx::new(ctx, Some(db.clone())))
                        })
                        .launch()
                        .await?;

                    handle.wait_for_node_exit().await
                }
            );
            
            // Clean up the shutdown checker thread
            if !shutdown_flag.load(Ordering::Relaxed) {
                shutdown_flag.store(true, Ordering::Relaxed);
            }
            
            // Wait for the shutdown thread to finish
            let _ = shutdown_thread.join();
            
            result
        },
        Err(e) => {
            eprintln!("Failed to parse Reth CLI arguments: {}", e);
            // If parsing fails, check for shutdown flag
            while !shutdown_flag.load(Ordering::Relaxed) {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Ok(())
        }
    }
}
