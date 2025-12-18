extern crate trpl; // required for mdbook test

use std::{future::Future, time::Duration};

mod scope_time_logger;
use scope_time_logger::ScopeTimeLogger;
use trpl::Either;

fn main() {
    trpl::run(async {
        let _main_scope = ScopeTimeLogger::new("Main");

        let slow = async {
            trpl::sleep(Duration::from_millis(1000)).await;
            "I finished!"
        };

        match timeout(slow, Duration::from_millis(555)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs_f32())
            }
        }
    });
}


async fn timeout<'a, F>(
    future: F,
    timeout_dur: Duration,
) -> Result<&'a str, Duration> 
where
    F: Future<Output = &'a str>,
{
    let timeout_future = async {
        trpl::sleep(timeout_dur).await;
        timeout_dur
    };

    match trpl::race(future, timeout_future).await {
        Either::Left(res) => Ok(res),
        Either::Right(res) => Err(res),
    }
}