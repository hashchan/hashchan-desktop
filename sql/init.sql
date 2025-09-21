-- Create boards table
CREATE TABLE IF NOT EXISTS boards (
    board_id BIGINT PRIMARY KEY,
    name VARCHAR NOT NULL,
    symbol VARCHAR NOT NULL,
    description TEXT,
    banner_url TEXT,
    banner_cid TEXT,
    timestamp BIGINT NOT NULL,
    block_number BIGINT NOT NULL
);

-- Create threads table
CREATE TABLE IF NOT EXISTS threads (
    thread_id BYTEA PRIMARY KEY,
    board_id BIGINT NOT NULL,
    creator BYTEA NOT NULL,
    title VARCHAR NOT NULL,
    content TEXT,
    img_url TEXT,
    img_cid TEXT,
    timestamp BIGINT NOT NULL,
    block_number BIGINT NOT NULL,
    FOREIGN KEY (board_id) REFERENCES boards (board_id)
);

-- Create posts table
CREATE TABLE IF NOT EXISTS posts (
    post_id BYTEA PRIMARY KEY,
    thread_id BYTEA NOT NULL,
    board_id BIGINT NOT NULL,
    creator BYTEA NOT NULL,
    content TEXT,
    img_url TEXT,
    img_cid TEXT,
    timestamp BIGINT NOT NULL,
    block_number BIGINT NOT NULL,
    FOREIGN KEY (thread_id) REFERENCES threads (thread_id),
    FOREIGN KEY (board_id) REFERENCES boards (board_id)
);

-- Create post_replies table
CREATE TABLE IF NOT EXISTS post_replies (
    post_id BYTEA NOT NULL,
    reply_to_id BYTEA NOT NULL,
    PRIMARY KEY (post_id, reply_to_id),
    FOREIGN KEY (post_id) REFERENCES posts (post_id),
    FOREIGN KEY (reply_to_id) REFERENCES posts (post_id)
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_threads_board_id ON threads (board_id);
CREATE INDEX IF NOT EXISTS idx_posts_thread_id ON posts (thread_id);
CREATE INDEX IF NOT EXISTS idx_posts_board_id ON posts (board_id);
