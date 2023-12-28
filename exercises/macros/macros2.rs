// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

// 需要先定义宏，让编译器知道这个宏的存在
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
