
pub struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    hash: String,
    prev_hash: String
}

impl Block {

    pub fn new(index: u64, timestamp: u64, data: String, hash: String, prev_hash: String) -> Self {
        Self{ index, timestamp, data, prev_hash, hash }
    }

    pub fn data_size(&self) -> usize {
        self.data.len()
    }

    pub fn creation_time(&self) -> u64 {
        self.timestamp / 1000
    }

}