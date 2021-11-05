// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

// I AM NOT DON

fn main() {
    call_me(3);
}

fn call_me(num:i32) {
    for i in 1..num+1 {
        println!("Ring! Call number {}", i);
    }
}
