use std::{fs, thread, time::{Duration,Instant}};
use tokio::{fs as tokio_fs, try_join};
#[tokio::main]
async fn main() {
    let start = Instant::now();
    let lock = fs::read_to_string("../Cargo.lock").unwrap();
    let toml = fs::read_to_string("../Cargo.toml").unwrap();
    
    fs::write("./lock.txt", lock).unwrap();
    fs::write("./toml.txt", toml).unwrap();
    println!("{:?}",start.elapsed());

    let start = Instant::now();
    let lock_handle = thread::spawn(|| {
        let lock = fs::read_to_string("../Cargo.lock").unwrap();
        fs::write("./lock.txt", lock).unwrap();
    });
    let toml_handle = thread::spawn(|| {
        let toml = fs::read_to_string("../Cargo.toml").unwrap();
        fs::write("./toml.txt", toml).unwrap();
    });
    lock_handle.join().unwrap();
    toml_handle.join().unwrap();
    println!("{:?}",start.elapsed());
    
    let start = Instant::now();
    let tokio_lock = tokio_fs::read_to_string("../Cargo.lock");
    let tokio_toml = tokio_fs::read_to_string("../Cargo.toml");
    let (lock,toml) = try_join!(tokio_lock,tokio_toml).unwrap();
    let tokio_lock = tokio_fs::write("./lock.txt", lock);
    let tokio_toml = tokio_fs::write("./toml.txt", toml);
    try_join!(tokio_lock,tokio_toml).unwrap();
    println!("{:?}",start.elapsed());
    /*
    let start = Instant::now();
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("{}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 10..20 {
        println!("{}",i);
        thread::sleep(Duration::from_millis(1));
    }
    let duration = start.elapsed();
    println!("{:?}",duration);
    */
}
