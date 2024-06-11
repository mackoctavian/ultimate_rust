use std::{thread, time::Duration};


fn main() {
//   single_thread();
//   multiple_thread();

    // thread::Builder::new()
    //     .name("Named Thread".to_string())
    //     .stack_size(std::mem::size_of::<usize>() * 4)
    //     .spawn(my_thread)
    //     .unwrap()
    //     .join()
    //     .unwrap();

    // chunk_example();

    thread::Builder::new()
        .name("My Thread".to_string())
        .spawn(|| {
            for i in 0..10 {
                println!("Thread numbered {i}");
                thread::sleep(Duration::from_millis(1));
            }
        })
        .unwrap()
        .join()
        .unwrap();

    for i in 0..5 {
        println!("Hello from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

   

}

fn single_thread() {
    let thread_handle = thread::spawn(move || thread1(2));
    thread_handle.join().unwrap();
}

//Single thread fn
fn thread1(n: u32) {
    println!("Hello From Thread number: {}", n);
}

fn multiple_thread() {
    const N_THREAD: usize = 10;
    let mut thread_handles = Vec::new();
    let thread_data: Vec<u32> = (0..10000).collect();
    let thread_chunks = thread_data.chunks(N_THREAD);

    for chunk in thread_chunks {
        let current_chunk = chunk.to_owned();
        let handle = thread::spawn(move || current_chunk.iter().sum::<u32>());
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        let data = handle.join().unwrap();
        println!("{:?}", data);
    }
}

fn my_thread() {
    println!("Hello from a thread named {}", thread::current().name().unwrap());
}

//Chunk example using scoped thread
fn chunk_example() {
    const N_THREAD: usize = 8;
    let to_add: Vec<u32> = (0..5000).collect();
    let chunks = to_add.chunks(N_THREAD);

    let sum = thread::scope(|s| {
        let mut thread_handles = Vec::new();

        for chunk in chunks {
            let handle = s.spawn(move || chunk.iter().sum::<u32>());
            thread_handles.push(handle);
        }

        thread_handles.into_iter().map(|h| h.join().unwrap())
        .sum::<u32>()
    });

    println!("Sum: {sum}");
}

fn scoped_threads() {
    const N_THREADS: usize = 8;
    let data: Vec<u32> = (0..5000).collect();
    let chunks = data.chunks(N_THREADS);

    let sum = thread::scope(|c| {
        let mut thread_handles = Vec::new();
        for chunk in chunks {
            let handle = c.spawn(move || chunk.iter().sum::<u32>());
            thread_handles.push(handle);
        }

        thread_handles.into_iter().map(|h| h.join().unwrap()).sum::<u32>()
    });
}