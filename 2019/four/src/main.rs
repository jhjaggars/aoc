fn test(digits: u64) -> bool {
    let first = digits.to_string().chars().next().unwrap();
    let mut min = first.to_digit(10).unwrap();
    let mut double_found = false;
    let mut consecutives = 1;
    for ch in digits.to_string()[1..].chars() {
        let digit = ch.to_digit(10).unwrap();

        if digit < min {
            return false;
        }

        if digit == min {
            consecutives += 1;
        }

        if digit > min {
            if consecutives == 2 {
                double_found = true;
            }
            min = digit;
            consecutives = 1;
        }
    }
    return double_found || consecutives == 2;
}

fn main() {
    println!("111111 -> {}", test(111111));
    println!("223450 -> {}", test(223450));
    println!("123789 -> {}", test(123789));
    println!("112233 -> {}", test(112233));
    println!("123444 -> {}", test(123444));
    println!("111122 -> {}", test(111122));

    let mut num = 0;
    for i in 372304..=847060 {
        if test(i) {
            num += 1;
        }
    }
    println!("matches = {}", num);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_111111() {
        assert!(!test(111111));
    }

    #[test]
    fn test_223450() {
        assert!(!test(223450));
    }

    #[test]
    fn test_123789() {
        assert!(!test(123789));
    }

    #[test]
    fn test_112233() {
        assert!(test(112233));
    }

    #[test]
    fn test_123444() {
        assert!(!test(123444));
    }

    #[test]
    fn test_111122() {
        assert!(test(111122));
    }
}
