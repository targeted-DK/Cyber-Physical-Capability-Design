use std::arch::x86_64::_rdtsc;

fn read_tsc() -> u64 {
    unsafe { _rdtsc() }
}

fn main() {
    let start = read_tsc();
    // Simulate some work
    for _ in 0..1 {}
    let end = read_tsc();

    println!("CPU cycles elapsed: {}", end - start);
}
