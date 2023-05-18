pub type Job = Box<dyn FnOnce() + 'static + Send>;

pub enum Message {
    Job(Job),
    Exit,
}
