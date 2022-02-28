use logger::logger::*;
use std::sync::{Arc, Mutex};

pub struct ThreadPool {
    #[allow(dead_code)]
    workers: Vec<Worker>,
    sender: std::sync::mpsc::Sender<Job>,
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(
            size > 0,
            "The size of the thread pool must be greater than zero"
        );

        let (sender, reciever) = std::sync::mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool { workers, sender }
    }
}

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    #[allow(dead_code)]
    id: usize,
    #[allow(dead_code)]
    thread: std::thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<std::sync::mpsc::Receiver<Job>>>) -> Worker {
        let thread = std::thread::spawn(move || loop {
            let job = reciever.lock().unwrap().recv().unwrap();
            info(&format!("Worker {} got a job; executing", id));

            job.call_box();
        });

        Worker { id, thread }
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}
