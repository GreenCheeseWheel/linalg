use std::{thread::{self, JoinHandle}, sync::{Arc, Mutex, mpsc::Sender}};

#[derive(Debug)]
pub enum PoolError {
    CreationError(&'static str),
}

pub struct ThreadPool {
    sender: Sender<Job>,
    threads:Vec<JoinHandle<()>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {

    pub fn new(size:usize) -> Result<ThreadPool, PoolError>
    {
        if size == 0
        {
            return Err(PoolError::CreationError("Thread pool must have at least one thread"));
        }

    
        let mut threads = Vec::with_capacity(size);


        let (tx, rx) = std::sync::mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        for id in 1..=size
        {
            // We create threads here
            let rx = Arc::clone(&rx);

            let handle = thread::spawn(move || {
                let job:Result<Job, std::sync::mpsc::RecvError> = rx.lock().unwrap().recv();

                if let Ok(job) = job
                {
                    job();
                }
                else {
                    eprintln!("Handle hung up");
                }

            });

            threads.push(handle);
        }

        Ok(ThreadPool { threads, sender: tx })
    }

    pub fn execute<F>(&self, f:F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        
        match self.sender.send(job) {
            Err(msg) => eprintln!("{}", msg),
            Ok(_) => {}
        }
        
    }

}

