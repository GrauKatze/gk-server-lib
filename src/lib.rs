pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn plus(one: i32, two: i32) -> i32 {
    one + two
}

pub fn minus(one: i32, two: i32) -> i32 {
    if one > two {
        one - two
    }else{
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn plus_work() {
        let result = plus(23, 43);
        assert_eq!(result, 23 + 43);
    }
}
