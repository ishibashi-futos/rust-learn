use logger::logger::*;
use std::sync::{Arc, Mutex};

pub struct ThreadPool {
    #[allow(dead_code)]
    workers: Vec<Worker>,
    sender: std::sync::mpsc::Sender<Message>,
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        info("Sending terminate message to all workers.");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            logger::logger::info(&format!("Shutting down worker {}", worker.id));
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    #[allow(dead_code)]
    id: usize,
    #[allow(dead_code)]
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<std::sync::mpsc::Receiver<Message>>>) -> Worker {
        let thread = std::thread::spawn(move || loop {
            let message = reciever.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    job.call_box();
                }
                Message::Terminate => {
                    info(&format!("Worker {} was told to terminate", id));
                    break;
                }
            }
            info(&format!("Worker {} got a job; executing", id));
        });

        Worker {
            id,
            thread: Some(thread),
        }
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

enum Message {
    NewJob(Job),
    Terminate,
}
