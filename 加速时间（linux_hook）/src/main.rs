use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn main() {
    loop {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        thread::sleep(Duration::from_millis(1000));
        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("start:{} end:{} end-start:{}", start, end, end - start);
    }
}
