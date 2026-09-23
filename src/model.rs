pub struct Log {
    key: String,
    message: String,
    timestamp: i64,
}

impl Log {
    pub fn new(key: String, message: String, timestamp: i64) -> Self {
        Self {
            key,
            message,
            timestamp,
        }
    }
}

pub struct Partition {
    id: i32,
    logs: Vec<Log>,
}

impl Partition {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            logs: Vec::new(),
        }
    }
}

pub struct Topic {
    id: i32,
    name: String,
    partitions: Vec<Partition>,
}

impl Topic {
    pub fn new(id: i32, name: String, partitions: Vec<Partition>) -> Self {
        Self {
            id,
            name,
            partitions,
        }
    }
}
