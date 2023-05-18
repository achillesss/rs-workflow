use crate::message::Message;
use crossbeam::sync::WaitGroup;
use std::thread;

pub struct Worker {
    id: u16,
    job_count: u64,
}

impl Worker {
    pub fn build(id: u16, wait_group: WaitGroup, receiver: flume::Receiver<Message>) -> Self {
        let mut job_count = 0;
        thread::spawn(move || {
            for msg in receiver {
                match msg {
                    Message::Job(j) => {
                        job_count += 1;
                        // println!("worker {id} receive job {job_count}");
                        j();
                        // println!("worker {id} finish job {job_count}");
                    }

                    Message::Exit => {
                        drop(wait_group);
                        println!("worker {id} exit with job {job_count} finish");
                        break;
                    }
                }
            }
        });

        println!("start worker {id}");
        Worker { id, job_count }
    }

    pub fn info(&self) {
        println!("worker {} handled {}", self.id, self.job_count);
    }
}
