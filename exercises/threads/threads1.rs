// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)


use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::ops::{Deref, DerefMut};

struct JobStatus {
    jobs_completed: u32,
}

impl Deref for JobStatus {
    type Target = u32; 

    fn deref(&self) -> &u32 {
        &self.jobs_completed
    }
}

impl DerefMut for JobStatus {
    fn deref_mut(&mut self) -> &mut u32 {
        &mut self.jobs_completed
    }
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = Arc::clone(&status);
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
	    let mut status_shared = status_shared.lock().unwrap();
            status_shared.jobs_completed += 1;
        }
    });
    
    let mut jobs_completed: u32;
    loop {
        jobs_completed = status.lock().unwrap().jobs_completed;
        if jobs_completed < 10 {
            println!("waiting... ({} jobs done)", jobs_completed);
            thread::sleep(Duration::from_millis(500));
        } else {
            break;
        }
    } 
}
