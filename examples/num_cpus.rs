use num_cpus;
use std::time::Duration;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let cpus = num_cpus::get();
    println!("logical cores: {}", cpus);

    tokio::spawn(async move {
        println!("task 1 started...");
        std::thread::sleep(Duration::from_secs(3));
        println!("task 1 woke up!");
    });

    tokio::spawn(async move {
        println!("task 2 started...");
        std::thread::sleep(Duration::from_secs(1));
        println!("task 2 woke up!");
    });

    std::thread::sleep(Duration::from_secs(5));
    println!("Done");
}
