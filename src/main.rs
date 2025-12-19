extern crate trpl; // required for mdbook test

use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

//use futures::stream::FuturesUnordered;

mod scope_time_logger;
use scope_time_logger::ScopeTimeLogger;

fn main() {
    let _scope_time = ScopeTimeLogger::new("Main");

    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(40);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ('a'..='j').map(|i| i.to_string());

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 {
                Duration::from_millis(50)
            } else {
                Duration::from_millis(150)
            };
            trpl::sleep(time_to_sleep).await;
            
            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
