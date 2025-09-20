mod db;

pub use db::HashChanDB;
pub use db::{Board, Thread, Post, PostReply};

// Re-export the initialize function
pub fn initialize() -> Result<HashChanDB, eyre::Error> {
    HashChanDB::initialize()
}