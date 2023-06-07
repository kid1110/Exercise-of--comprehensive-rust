use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[warn(unused_must_use)]
struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::SyncSender<String>
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks...
        println!("{} is eating...", &self.name);
        self.left_fork.lock().unwrap();
        self.right_fork.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    // Create forks
    let forks = (0..PHILOSOPHERS.len()).map(|_| Arc::new(Mutex::new(Fork))).collect::<Vec<_>>();
    // Create philosophers
    let(tx,rx) = mpsc::sync_channel(10);

    for i in 0..forks.len(){
        let tx = tx.clone();
        let mut left_fork = forks[i].clone();
        let mut right_fork = forks[(i+1)%forks.len()].clone();

        if i == forks.len()-1{
            std::mem::swap(&mut left_fork, &mut right_fork);
        }
        let ph = Philosopher{
            left_fork,
            right_fork,
            name: PHILOSOPHERS[i].to_string(),
            thoughts: tx,
        };
        thread::spawn(move ||{
            for _ in 0..10{
                ph.eat();
                ph.think();
            }
        });

    }
    drop(tx);
    // Make them think and eat
    for th in rx{
        println!("th{}",th)
    }

    // Output their thoughts
}