// @generated automatically by Diesel CLI.

diesel::table! {
    boards (board_id) {
        board_id -> Int8,
        name -> Text,
        symbol -> Text,
        description -> Nullable<Text>,
        banner_url -> Nullable<Text>,
        banner_cid -> Nullable<Text>,
        timestamp -> Int8,
        block_number -> Int8,
    }
}

diesel::table! {
    posts (post_id) {
        post_id -> Bytea,
        thread_id -> Bytea,
        board_id -> Int8,
        creator -> Bytea,
        content -> Nullable<Text>,
        img_url -> Nullable<Text>,
        img_cid -> Nullable<Text>,
        timestamp -> Int8,
        block_number -> Int8,
    }
}

diesel::table! {
    post_replies (post_id, reply_to_id) {
        post_id -> Bytea,
        reply_to_id -> Bytea,
    }
}

diesel::table! {
    threads (thread_id) {
        thread_id -> Bytea,
        board_id -> Int8,
        creator -> Bytea,
        title -> Text,
        content -> Nullable<Text>,
        img_url -> Nullable<Text>,
        img_cid -> Nullable<Text>,
        timestamp -> Int8,
        block_number -> Int8,
    }
}

diesel::joinable!(posts -> boards (board_id));
diesel::joinable!(posts -> threads (thread_id));
diesel::joinable!(threads -> boards (board_id));

diesel::allow_tables_to_appear_in_same_query!(
    boards,
    posts,
    post_replies,
    threads,
);
