extern crate rand;
use std::thread;
use std::time::Duration;
use rand::random;

const NUM_THREADS: i32 = 10;

fn task(arg: i32) -> (i32, u32) {
    let x = random::<u32>();
    let s = x % 5000;
    thread::sleep(Duration::from_millis(s as u64));
    (arg, s)
}

fn main() {
    let mut handles = vec![];
    for i in 0..NUM_THREADS {
        handles.push(thread::spawn(move || task(i)));
    }

    for _ in 0..NUM_THREADS {
        let handle = handles.pop().unwrap();
        let (thread_arg, sleep_time) = handle.join().unwrap();
        println!("thread=[{}] sleep {:?}msec", thread_arg, sleep_time);
    }
}
