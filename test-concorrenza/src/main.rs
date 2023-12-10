use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::Duration;

fn main() {
    let v: Vec<JoinHandle<i32>> = (1..=4).map(|tid| {
        thread::spawn(move || {
            (1..10).for_each(|i| {
                println!("Thread {} iterazione #{}", tid, i);
                sleep(Duration::from_secs(1));
            });
            println!("Thread {} fatto", tid);
            tid
        })
    }).collect();

    println!("Questo è il thread principale");
    sleep(Duration::from_secs(4));
    println!("Attendo la terminazione dei thread secondari");
    v.into_iter().for_each(|h| {
        let res = h.join().unwrap();
        println!("Il risultato è {}", res);
    });
    println!("Thread terminato");
}
