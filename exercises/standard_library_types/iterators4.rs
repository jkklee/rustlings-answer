// iterators4.rs


pub fn factorial(num: u64) -> u64 {
    (1..=num).fold(1, |acc, x| acc * x) //ljk fold函数牛啊，它可以将一个迭代器中的所有元素折叠成一个单一的值，fold函数接受两个参数：一个累加器初始值和一个闭包。闭包接受两个参数：累加器和当前元素。在每次迭代中，闭包将累加器乘以当前元素，并返回新的累加器值。最后，fold函数返回累加器的最终值
}
// println!("{}", factorial(10));

//下面是一个同样用迭代器方式实现的斐波那契数列函数
fn fibonacci(num: u64) -> u64 {
    (0..num).fold((0, 1), |acc, _| (acc.1, acc.0 + acc.1)).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
