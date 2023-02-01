use std::env;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    println!("Hello, world! ");
    let path = env::current_dir();
    // println!("The current directory is {}", path.);

    let n_workers = 4;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);
    
    let (tx, rx) = channel();

    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move|| {
            tx.send(1).expect("channel will be there waiting for the pool");
        });
    }
    
    assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
}


fn thread_testing(){
    let variable_name = thread::spawn(|| {});
    variable_name.join().unwrap();

    let a:i32 = 5;
    let handle = thread::spawn(move|| {
        // try to access the
        // variable outside
        // of this thread
        println!("a: {}", a);
    });
    handle.join().unwrap();
}


//https://www.fatalerrors.org/a/0Npz0Dw.html
