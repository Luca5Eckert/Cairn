mod model;
mod storage;

fn main() {
    let id = storage::init_topic(String::from("orders"), 3);
    println!("topic id: {id}");
}
