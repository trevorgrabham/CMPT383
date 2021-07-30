use std::sync::mpsc;
use std::thread;

pub trait Task {
    type Output: Send;
    fn run(&self) -> Option<Self::Output>;
}

pub struct WorkQueue<TaskType: 'static + Task + Send> {
    send_tasks: Option<spmc::Sender<TaskType>>, // Option because it will be set to None to close the queue
    recv_tasks: spmc::Receiver<TaskType>,
    //send_output: mpsc::Sender<TaskType::Output>, // not need in the struct: each worker will have its own clone.
    recv_output: mpsc::Receiver<TaskType::Output>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl<TaskType: 'static + Task + Send> WorkQueue<TaskType> {
    pub fn new(n_workers: usize) -> WorkQueue<TaskType> {
        // TODO: create the channels; start the worker threads; record their JoinHandles
        let mut handles: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n_workers);
        let (mp_sender, mp_reciever) = mpsc::channel();
        let (sp_sender, sp_reciever) = spmc::channel();

        for _ in 0..n_workers {
            let snd = mp_sender.clone();
            let rcv = sp_reciever.clone();
            handles.push
                (thread::spawn(move || {
                    WorkQueue::run(rcv, snd);
                })
            );
        }

        WorkQueue {
            send_tasks: Some(sp_sender),
            recv_tasks: sp_reciever,
            recv_output: mp_reciever,
            workers: handles,
        }
    }
    

    fn run(recv_tasks: spmc::Receiver<TaskType>, send_output: mpsc::Sender<TaskType::Output>) {
        // TODO: the main logic for a worker thread
        loop {
            let task_result = recv_tasks.recv();
            // NOTE: task_result will be Err() if the spmc::Sender has been destroyed and no more messages can be received here
            match task_result {
                Err(_) => {return;},
                Ok(task) => {
                    let res = task.run();
                    match res {
                        Some(good_res) => {send_output.send(good_res).unwrap();}
                        None => {}
                    }
                }
            }
        }
    }

    pub fn enqueue(&mut self, t: TaskType) -> Result<(), spmc::SendError<TaskType>> {
        // TODO: send this task to a worker
        match self.send_tasks {
            Some(ref mut send_tasks) => {
                return send_tasks.send(t);
            },
            None => {
                return Err(spmc::SendError(t));
            }
        }

    }

    // Helper methods that let you receive results in various ways
    pub fn iter(&mut self) -> mpsc::Iter<TaskType::Output> {
        self.recv_output.iter()
    }
    pub fn recv(&mut self) -> TaskType::Output {
        self.recv_output
            .recv()
            .expect("I have been shutdown incorrectly")
    }
    pub fn try_recv(&mut self) -> Result<TaskType::Output, mpsc::TryRecvError> {
        self.recv_output.try_recv()
    }
    pub fn recv_timeout(&self, timeout: std::time::Duration) -> Result<TaskType::Output, mpsc::RecvTimeoutError> {
        self.recv_output.recv_timeout(timeout)
    }

    pub fn shutdown(&mut self) {
        // TODO: destroy the spmc::Sender so everybody knows no more tasks are incoming;
        // drain any pending tasks in the queue; wait for each worker thread to finish.
        // HINT: Vec.drain(..)
        self.send_tasks = None;
        
        loop {
            let reciever_result = self.recv_tasks.recv();
            match reciever_result {
                Err(_) => {
                    // pass
                    break;
                },
                Ok(_) => {
                    // pass
                }
            }
        }
        let workers = self.workers.drain(..);
        for worker in workers {
            worker.join().unwrap();
        }
    }
}

impl<TaskType: 'static + Task + Send> Drop for WorkQueue<TaskType> {
    fn drop(&mut self) {
        // "Finalisation in destructors" pattern: https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html
        match self.send_tasks {
            None => {} // already shut down
            Some(_) => self.shutdown(),
        }
    }
}
