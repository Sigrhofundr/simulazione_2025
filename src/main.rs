use std::{sync::{Arc, Condvar, Mutex}, thread::sleep, time::Duration};

fn main() {
    println!("Hello, world!");
}

#[derive(Clone)]
pub struct CountDownLatch {
    counter: Arc<(Mutex<usize>,Condvar)>
}

impl CountDownLatch {
    pub fn new(n: usize) -> Self {
        Self {
            counter: Arc::new((Mutex::new(n),Condvar::new()))
        }
    }

    pub fn wait_zero(&self, timeout: Option<std::time::Duration>) -> Result<(),()> {
        let timeout_duration = timeout.unwrap_or(Duration::from_millis(5000));
        let (lock, cvar) = &*self.counter;
        let counter = lock.lock().unwrap();
        let result = cvar.wait_timeout_while(counter, timeout_duration,|c| *c!=0).unwrap();
        if result.1.timed_out(){
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn count_down(&self) {
        let (lock, cvar) = &*self.counter;
        let mut counter = lock.lock().unwrap();
        *counter -=1;
        // quando il contatore va a zero tutti i thread bloccati su wait_zero() vengono sbloccati
        if *counter == 0 {
            cvar.notify_all();
        }
    }
}

pub fn do_some_work(message: &str) {
    sleep(Duration::from_secs(1));
    println!("{}",message.to_string());
}

#[cfg(test)]
mod tests{
    use std::{thread, time::Duration};

    use crate::{do_some_work, CountDownLatch};


    #[test]
    pub fn try_count_down() {
        let my_latch = CountDownLatch::new(4);
        my_latch.count_down();
        my_latch.count_down();
        my_latch.count_down();
        my_latch.count_down();
        let (lock,cvar) = &*my_latch.counter;
        assert_eq!(*lock.lock().unwrap(),0);
    }

    #[test]
    pub fn demo_latch() {
    let my_latch = CountDownLatch::new(4);
    let mut handles = vec![];
    for _ in 0..10 {
        let latch_clone = my_latch.clone(); // Clona il latch
        let h = thread::spawn(move ||{
            latch_clone.wait_zero(None);
            do_some_work("(2) lavoro che necessita driver");
            do_some_work("(3) altro lavoro che non necessita driver");
        });
        handles.push(h);
    } 
    do_some_work("(1) prepapara il driver");
    my_latch.count_down();
    my_latch.count_down();
    my_latch.count_down();
    my_latch.count_down();
    do_some_work("(4) rilascia il driver");
    for h in handles {
        let _ = h.join();
    }
}

}