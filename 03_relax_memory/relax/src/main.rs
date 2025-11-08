use std::sync::Arc;
use std::sync::atomic::AtomicU32;

fn main() {
   let data = Arc::new(AtomicU32::new(0));
   let flag = Arc::new(AtomicU32::new(0));

   let d1 = Arc::clone(&data);
   let f1 = Arc::clone(&flag);


}
