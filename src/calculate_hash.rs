use sha2::{Sha256, Digest};


pub fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8>{
    let data = serde_json::json!({
        "id": id, 
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce,
    });
    
    // create a Sha256 object
    let mut hasher = Sha256::default();
    
    // write input message
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}