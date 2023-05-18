use crate::message::Message;
use crate::worker::Worker;
use crossbeam::sync::WaitGroup;
use std::sync::mpsc;
use std::thread;

pub struct Factory {
    workers_count: u16,
    deliver_chan: flume::Sender<Message>,
    done_chan: mpsc::Receiver<()>,
}

impl Factory {
    pub fn deliver<F>(&self, f: F)
    where
        F: FnOnce() + 'static + Send,
    {
        let msg = Message::Job(Box::new(f));
        // TODO: handle error
        self.deliver_chan.send(msg).unwrap();
    }

    pub fn build(count: u16) -> Self {
        let (tx, rx) = flume::unbounded();
        let (done_tx, done_rx) = mpsc::sync_channel(0);
        let wg = WaitGroup::new();

        for i in 0..count {
            let recv_chan = rx.clone();
            let wait_group = wg.clone();
            Worker::build(i, wait_group, recv_chan);
        }

        thread::spawn(move || {
            wg.wait();
            drop(done_tx);
            // done_tx.send(())
        });

        Self {
            workers_count: count,
            deliver_chan: tx,
            done_chan: done_rx,
        }
    }

    pub fn closed(&self) -> bool {
        // 不用处理错误
        let result = self.done_chan.try_recv();

        match result {
            Ok(_) => false,
            Err(e) => match e {
                // 只有连接关闭才说明已经被关闭了
                mpsc::TryRecvError::Disconnected => true,
                _ => false,
            },
        }
    }
    // TODO:
    // 等待所有线程安全退出
    pub fn close(&self) {
        if self.closed() {
            println!("factory already closed");
            return;
        }

        println!("factory wait for workers before close");

        for _i in 0..self.workers_count {
            _ = self.deliver_chan.send(Message::Exit);
        }

        println!("factory closed");
    }
}
