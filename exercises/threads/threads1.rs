// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

// ljk 这段代码中的错误是由于尝试在Arc中的共享数据上进行可变引用。Arc是一个线程安全的引用计数指针，它允许多个线程共享所有权。然而，由于共享数据的线程安全性，您不能直接在Arc中的数据上进行可变引用。
// 为了解决这个问题，您可以使用Mutex或RwLock来保护共享数据。这些类型提供了一种机制，使得只有一个线程可以访问共享数据的可变引用。

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut status = status_shared.lock().unwrap();
            println!("the {} loop in sub thread", status.jobs_completed);
            status.jobs_completed += 1;
        }
    });
    while {
        let status = status.lock().unwrap();
        status.jobs_completed < 10
    } {
        println!("waiting...");
        thread::sleep(Duration::from_millis(500));
    }
    // 下面的代码效果一样
    // while status.lock().unwrap().jobs_completed < 10 {
    //     println!("waiting... ");
    //     thread::sleep(Duration::from_millis(500));
    // }
}
