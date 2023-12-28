// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}

// ljk
// $val:expr 中的 expr 是一个宏规则中的模式（pattern），而 $val 是一个捕获模式（capture pattern）。这样的规则允许你在宏调用时捕获传递给宏的表达式，并在宏的展开中使用。

// 具体来说，$val:expr 指定了一个模式，该模式匹配任何表达式，并将其捕获为一个标识符 $val。在宏的展开中，你可以使用这个捕获到的表达式。

// 在你的代码中，这个宏规则有两个分支：

// () => { println!("Check out my macro!"); }: 匹配没有参数的情况，打印一条信息。
// ($val:expr) => { println!("Look at this other macro: {}", $val); }: 匹配带有一个表达式参数的情况，打印包含该表达式的信息。
// 因此，当你调用 my_macro!(7777); 时，它会匹配第二个分支，将 7777 捕获到 $val 中，然后打印相应的信息。
