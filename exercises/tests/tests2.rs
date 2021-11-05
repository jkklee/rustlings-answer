// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

// I AM NOT DON

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        let mut v =Vec::new();
        v.push(1); v.push(2);
        let v1 = vec!(1,2,3);
        assert_eq!(&v[0],&v1[0]);
        assert_eq!(1,v1[0]);
        assert_eq!(&v[0],&1);
        assert_eq!(1,*&1);
    }
}
