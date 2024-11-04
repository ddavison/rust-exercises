use std::time::Duration;
use tokio::time::sleep;

// #[tokio::main]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // let f = my_function();
    // println!("Future created!");
    // f.await;
    let mut handles = vec![];

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

// A simplified version of the Future trait
trait Future {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

enum FutureStateMachine {
    Start,
    FirstRead,
    SecondRead,
    Done,
}

async fn my_function(i: i32) {
    println!("[{i}] I'm an async function!");

    let s1 = read_from_database().await;
    println!("[{i}] First result: {s1}");

    let s2 = read_from_database().await;
    println!("[{i}] Second result: {s2}");
    // this is the same thing as
    // fn my_function() -> impl Future<Output = ()> {
    //     println!("I'm an async function!");
    // }

}

async fn read_from_database() -> String {
    sleep(from_secs(2)).await;
    "Hello, world!".to_owned()
}
