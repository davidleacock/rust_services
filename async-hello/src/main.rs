use std::thread::sleep;
use std::time::{Duration, Instant};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};


struct AsyncTimer {
    expire_time: Instant
}

impl Future for AsyncTimer {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.expire_time {
            println!("Future 1 Results Ready.");
            Poll::Ready(String::from("Future 1 completed."))
        } else {
            println!("Future 1 not ready yet...");
            let waker = cx.waker().clone();
            let expire_time = self.expire_time;
            std::thread::spawn(move || {
                let current_time = Instant::now();
                if current_time < expire_time {
                    sleep(expire_time - current_time);
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}



#[tokio::main]
async fn main() {
    println!("Starting...");

    let handle = tokio::spawn(async {
        let future = AsyncTimer {
            expire_time: Instant::now() + Duration::from_millis(5000)
        };
        println!("{:?}", future.await)
    });

    let handle2 = tokio::spawn(async {
        let file2_contents = read_from_file2().await;
        println!("{:?}", file2_contents)
    });

    let _ = tokio::join!(handle, handle2);
}

fn read_from_file1() -> impl Future<Output=String> {
    async {
        sleep(Duration::new(4, 0));
        println!("{:?}", "Processing from 1");
        String::from("file 1 results.")
    }
}

fn read_from_file2() -> impl Future<Output=String> {
    async {
        sleep(Duration::new(2, 0));
        println!("{:?}", "Processing from 2");
        String::from("file 2 results.")
    }
}
