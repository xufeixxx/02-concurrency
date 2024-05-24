use std::{thread, time::Duration};

use concurrency::Metrics;
use rand::Rng;

const N: usize = 2;
const M: usize = 4;

fn main() -> anyhow::Result<()> {
    let metrics = Metrics::new();
    println!("{:?}", metrics.snapshot());

    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics);
    }

    // anyhow::Ok(())
}

fn task_worker(idx: usize, metrics: Metrics) -> anyhow::Result<()> {
    thread::spawn(move || -> anyhow::Result<()> {
        loop {
            // do long term stuff
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
    });
    anyhow::Ok(())
}

fn request_worker(metrics: Metrics) -> anyhow::Result<()> {
    thread::spawn(move || -> anyhow::Result<()> {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..256);
            metrics.inc(format!("req.page.{}", page))?;
        }
        // anyhow::Ok(())
    });
    anyhow::Ok(())
}
