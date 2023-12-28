// TryFrom is a simple and safe type conversion that may fail in a controlled way under some circumstances.（转换可能会以可控的方式失败）
// Basically, this is the same as From. The main difference is that this should return a Result type
// instead of the target type itself.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.TryFrom.html
use std::convert::{TryFrom, TryInto};
use std::error;

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Your task is to complete this implementation
// and return an Ok result of inner type Color.
// You need to create an implementation for a tuple of three integers,
// an array of three integers and a slice of integers.
//
// Note that the implementation for tuple and array will be checked at compile time,
// but the slice implementation needs to check the slice length!
// Also note that correct RGB color values must be integers in the 0..=255 range.

// Tuple implementation
// 只为3元素元组实现
impl TryFrom<(i16, i16, i16)> for Color {
    //ljk TryFrom trait 的定义中，type Error 是 TryFrom trait 的实现者必须定义的类型。这个类型用于表示转换失败时返回的错误。
    type Error = Box<dyn error::Error>; // ljk type Error 固定写法
                                        //TryFrom trait 的实现者将 Err() 类型包装为 Box<dyn error::Error> 类型。这样做的原因是 Box<dyn error::Error> 类型是 Rust 中表示错误的通用类型。它可以包装任何实现了 Error trait 的类型。
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        // ljk 验证每个值都在0-255之间, 不需要用if来判断
        let red = u8::try_from(tuple.0)?;
        let green = u8::try_from(tuple.1)?;
        let blue = u8::try_from(tuple.2)?;

        Ok(Color { red, green, blue })
        // 不太明白的是Err(T) 这个返回类型就不需要管了吗
    }
}

// Array implementation
// ljk 为符合数据格式的数组实现
impl TryFrom<[i16; 3]> for Color {
    type Error = Box<dyn error::Error>; // ljk TryFrom trait 的实现者将 Err() 类型的值包装为 Box<dyn error::Error> 类型
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        let red = u8::try_from(arr[0])?;
        let green = u8::try_from(arr[1])?;
        let blue = u8::try_from(arr[2])?;

        Ok(Self { red, green, blue })
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = Box<dyn error::Error>;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err("Slice must contain exactly three elements".into());
        }

        let red = u8::try_from(slice[0])?;
        let green = u8::try_from(slice[1])?;
        let blue = u8::try_from(slice[2])?;

        Ok(Color { red, green, blue })
    }
}

fn main() {
    // Use the `from` function
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // Since From is implemented for Color, we should be able to use Into
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // With slice we should use `from` function
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // or take slice within round brackets and use Into
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert!(Color::try_from((256, 1000, 10000)).is_err());
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert!(Color::try_from((-1, -10, -256)).is_err());
    }
    #[test]
    fn test_tuple_sum() {
        assert!(Color::try_from((-1, 255, 255)).is_err());
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert!(c.is_err());
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert!(c.is_err());
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert!(c.is_err());
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert!(Color::try_from(&arr[..]).is_err());
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert!(Color::try_from(&arr[..]).is_err());
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert!(Color::try_from(&arr[..]).is_err());
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert!(Color::try_from(&v[..]).is_err());
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert!(Color::try_from(&v[..]).is_err());
    }
}
