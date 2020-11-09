use tokio::runtime::{Runtime as TokioRuntime};
use std::future::Future;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::pin::Pin;

fn main() {
    let rt = Runtime::new();
    let handle = rt.handle();

    thread::spawn(move || {
        rt.run();
    });

    let response = handle.async_test();
    println!("response: {}", response);
}

struct Handle {
    sender: Sender<(Pin<Box<dyn Future<Output=u32> + 'static + Send>>, Sender<u32>)>,
}


async fn my_func() -> u32 {
    println!("async_test exec");
    return 42
}

impl Handle {
    fn async_test(&self) -> u32 {
        let (tx, rx) = channel();
        self.sender.send((Box::pin(my_func()), tx)).unwrap();
        return rx.recv().unwrap()
    }
}

struct Runtime {
    sender: Sender<(Pin<Box<dyn Future<Output=u32> + 'static + Send>>, Sender<u32>)>,
    receiver: Receiver<(Pin<Box<dyn Future<Output=u32> + 'static + Send>>, Sender<u32>)>,
    inner: TokioRuntime,
}

impl Runtime {
    fn new() -> Runtime {
        let (sender, receiver) = channel();
        return Runtime {
            sender,
            receiver,
            inner: TokioRuntime::new().unwrap(),
        }
    }

    fn handle(&self) -> Handle {
        let my_sender = self.sender.clone();
        return Handle {
            sender: my_sender,
        }
    }

    pub fn run(mut self) {
        println!("Running started");
        for (async_fn, sender) in self.receiver.iter() {
            println!("Running dequeue");
            let task = async move {
                let res = async_fn.await;
                sender.send(res).unwrap();
            };
            self.inner.spawn(task);
        }
        println!("runtime exit");
    }
}
