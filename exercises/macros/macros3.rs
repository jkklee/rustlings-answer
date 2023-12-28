// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

mod macros {
    // ljk 在宏定义上使用 #[macro_export] 导出宏
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}
// use macros::my_macro;
fn main() {
    // ljk 使用 #[macro_use] 引入宏
    #[macro_use]
    use crate::my_macro;
    my_macro!();
}
