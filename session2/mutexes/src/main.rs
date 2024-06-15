use std::{sync::{atomic::AtomicU32, Mutex}, thread};

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

static mut NO: i32 = 0;

fn main() {
   let mut thread_handles = Vec::new();

   for _ in 0..10 {
        let handle = thread::spawn(|| {
            let mut lock = NUMBERS.lock().unwrap();
            lock.push(1);
        });

        thread_handles.push(handle);
   }

   thread_handles.into_iter().for_each(|h| h.join().unwrap());
   let lock = NUMBERS.lock().unwrap();
   println!("{:?}", lock);
}

fn test_atomics() {
    let mut handles = Vec::new();

    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1_100 {
                unsafe {
                    NO += 1;
                }
            } 
        });

        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    unsafe {
        println!("{NO}");
    }
}


