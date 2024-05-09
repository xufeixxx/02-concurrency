use anyhow::{anyhow, Ok, Result};
use std::{sync::mpsc, thread, time::Duration};
const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // 创建producers
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || {
            producer(i, tx).unwrap();
        });
    }

    drop(tx);

    let consumer = thread::spawn(|| {
        for msg in rx {
            println!("{:?}", msg);
        }
        println!("consumer exit");
    });

    consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));

        // random exit the producer
        if rand::random::<u8>() % 10 == 0 {
            println!("producer {} exit", idx);
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
