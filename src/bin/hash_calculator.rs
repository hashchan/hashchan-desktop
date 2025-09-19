use tiny_keccak::{Hasher, Keccak};

fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(input);
    hasher.finalize(&mut output);
    output
}

fn to_hex_string(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(2 + bytes.len() * 2);
    hex.push_str("0x");
    for byte in bytes {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}

fn main() {
    // Event signatures
    let new_board_sig = "NewBoard(uint256,string,string,string,string,string,string[],uint256)";
    let new_thread_sig = "NewThread(uint256,bytes32,address,string,string,string,string,uint256)";
    let new_post_sig = "NewPost(uint256,bytes32,bytes32,bytes32[],address,string,string,string,uint256)";
    
    println!("Event signatures:");
    println!("NEW_BOARD_SIG: {}", to_hex_string(&keccak256(new_board_sig.as_bytes())));
    println!("NEW_THREAD_SIG: {}", to_hex_string(&keccak256(new_thread_sig.as_bytes())));
    println!("NEW_POST_SIG: {}", to_hex_string(&keccak256(new_post_sig.as_bytes())));
}
