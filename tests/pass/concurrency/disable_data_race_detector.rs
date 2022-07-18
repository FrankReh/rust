//@ignore-target-windows: Concurrency on Windows is not supported yet.
//@compile-flags: -Zmiri-disable-data-race-detector

use std::thread::spawn;

#[derive(Copy, Clone)]
struct EvilSend<T>(pub T);

unsafe impl<T> Send for EvilSend<T> {}
unsafe impl<T> Sync for EvilSend<T> {}

pub fn main() {
    let mut a = 0u32;
    let b = &mut a as *mut u32;
    let c = EvilSend(b);
    unsafe {
        let j1 = spawn(move || {
            *c.0 = 32;
        });

        let j2 = spawn(move || {
            *c.0 = 64; // Data race (but not detected as the detector is disabled)
        });

        j1.join().unwrap();
        j2.join().unwrap();
    }
}
