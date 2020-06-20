use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub type Time = usize;

pub struct Clock {
    counter: AtomicUsize
}

impl Clock {
    pub fn new() -> Clock {
        0.into()
    }

    pub fn time(&self) -> Time {
        self.counter.load(Ordering::SeqCst)
    }

    pub fn increment(&self) -> Time {
        self.counter.fetch_add(1, Ordering::SeqCst)+1
    }

    pub fn witness(&self, other: Time) {
        loop {
            let cur = self.counter.load(Ordering::SeqCst);
            if other < cur {
                return;
            }

            if self.counter.compare_and_swap(cur, other+1, Ordering::SeqCst) == cur {
                return
            }
        }
    }
}

impl From<usize> for Clock {
    fn from(t: usize) -> Clock {
        Clock{ counter: AtomicUsize::new(t) }
    }
}

impl Iterator for Clock {
    type Item = Time;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.increment())
    }
}