use std::thread;
fn main() {
    let n_thread = 5;

    let mut handles = Vec::new();

    for i in 0..n_thread {
        let handle = thread::spawn(move || {
            println!(
                "Iterate: {i} , Current thread ID: {:#?}",
                thread::current().id()
            );
        });

        println!("Thread ID: {:?}", handle.thread().id());
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Hello from main thread");
}
