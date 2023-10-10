
use std::{thread, time};

fn main() {

    let ten_millis = time::Duration::from_millis(100000000);
    let now = time::Instant::now();

    thread::sleep(ten_millis);
    
}
