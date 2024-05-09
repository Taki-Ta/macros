use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[tokio::main]
async fn main() {
    let fut = MyFut::new(42);
    println!("Final result: {}", fut.await);
}

#[allow(unused)]
fn poll_fut(cx: &mut Context<'_>) -> Poll<usize> {
    let mut fut = MyFut::new(42);
    let fut = Pin::new(&mut fut);
    my_ready!(fut.poll(cx))
}

struct MyFut {
    polled: bool,
    value: usize,
}

impl MyFut {
    fn new(value: usize) -> Self {
        MyFut {
            polled: false,
            value,
        }
    }
}

impl Future for MyFut {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(self.value)
        } else {
            self.polled = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[macro_export]
macro_rules! my_ready {
    ($expr:expr) => {
        match $expr {
            Poll::Ready(v) => Poll::Ready(v),
            Poll::Pending => Poll::Pending,
        }
    };
}
