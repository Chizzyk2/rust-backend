pub fn divide_two_values(x: i32) -> i32{
    divider(x, 3)
}

fn divider(x: i32, y: i32) -> i32{
    x / y
}

#[cfg(test)]
    mod tests{
        use super::*;
        #[test]

        fn division(){
            assert_eq!(5, divider(15, 3))
        }
    }