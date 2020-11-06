use tokio::runtime::{Runtime as TokioRuntime};
use std::future::Future;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::pin::Pin;

fn main() {
    println!("Hello, world!");
    // Start a runtime
    let mut rt = TokioRuntime::new().unwrap();

    let my_func = async {
        println!("inline async");
    };

    let (sender, receiver) = channel();
    let (stop, stopped) = channel::<()>();

    sender.send(Box::pin(my_func)).unwrap();
    thread::spawn(move || {
        let recv_msg= receiver.recv().unwrap();
        rt.block_on(recv_msg);
        stop.send(()).unwrap();
    });

    println!("waiting");
    stopped.recv().unwrap();
}

struct Handle {
    sender: Sender<Pin<Box<dyn Future<Output=()> + 'static>>>,
}

impl Handle {
    fn async_test(&self) {
        let my_func = async {
            println!("async_test exec");
        };

        self.sender.send(Box::pin(my_func)).unwrap();
    }
}

struct Runtime {
    sender: Sender<Pin<Box<dyn Future<Output=()> + 'static>>>,
    receiver: Receiver<Pin<Box<dyn Future<Output=()> + 'static>>>,
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
        for async_fn in self.receiver.iter() {
            self.inner.block_on(async_fn);
        }
        println!("runtime exit");
    }
}
