use std::sync::{Mutex, OnceLock};

use crate::model::{Partition, Topic};

static TOPICS: OnceLock<Mutex<Vec<Topic>>> = OnceLock::new();

fn topics() -> &'static Mutex<Vec<Topic>> {
    TOPICS.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn init_topic(name: String, partition_count: usize) -> i32 {
    let mut guard = topics().lock().unwrap();
    let id = guard.len() as i32;

    let partitions = (0..partition_count)
        .map(|partition_id| Partition::new(partition_id as i32))
        .collect();

    guard.push(Topic::new(id, name, partitions));
    id
}
