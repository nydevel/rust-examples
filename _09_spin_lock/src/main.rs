use std::cmp::Ordering;
use std::sync::atomic::AtomicBool;

struct SpinLock{
    locked: AtomicBool
}

impl SpinLock {
    fn new(){
        Self { locked: AtomicBool::new(false) }
    }

    fn lock() {
        while self.locked.swap(true, Ordering::Asquire) {
            std::hint::spin_loop()
        }
    }

    fn unlock(&self){
        self.locked.store(false, Ordering::Release)
    }
}

fn main() {

}
