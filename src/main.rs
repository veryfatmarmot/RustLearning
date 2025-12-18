extern crate trpl; // required for mdbook test

use std::{future::Future, time::Duration};

mod scope_time_logger;
use scope_time_logger::ScopeTimeLogger;
use trpl::Either;

fn main() {
    trpl::run(async {
        let _main_scope = ScopeTimeLogger::new("Main");

        let slow = async {
            for i in 0..10 {
                trpl::sleep(Duration::from_millis(100)).await;
                println!("{}ms passed", 100 * (i + 1));
            }
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

async fn timeout<F: Future>(future: F, timeout_dur: Duration) -> Result<F::Output, Duration>
{
    match trpl::race(future, trpl::sleep(timeout_dur)).await {
        Either::Left(res) => Ok(res),
        Either::Right(_) => Err(timeout_dur),
    }
}
