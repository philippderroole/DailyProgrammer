use std::env;

///
/// # Background
/// A *practical number* is a positive integer N such that all smaller positive integers can be represented as sums of distinct divisors of N. For example, 12 is a practical number because all the numbers from 1 to 11 can be expressed as sums of the divisors of 12, which are 1, 2, 3, 4, and 6. (Wikipedia.) However, 10 is not a practical number, because 4 and 9 cannot be expressed as a sum of 1, 2, and 5. For more detailed explanation and examples, see this recent Numberphile video.
///
/// # Challenge
/// Write a function that returns whether a given positive integer is a practical number.
/// 
/// practical(1) => true
/// practical(2) => true
/// practical(3) => false
/// practical(10) => false
/// practical(12) => true
/// You should be able to handle numbers up to 10,000 efficiently. The sum of all practical numbers up to 10,000 inclusive is 6,804,107. Test your code by verifying this value.
///
/// # Optional bonus challenge
/// Consider the numbers X in the range 1 to 10,000 inclusive. The sum of all X such that 1019 + X is a practical number is 1,451,958. Find the sum of all X such that 1020 + X is a practical number. I found the section Characterization of practical numbers in the Wikipedia article useful here.
///

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => println!("{}", sum_all_practical_numbers()),
        2 => {
            let input: u32 = args[1].parse().expect("Input is not a number!");
            println!("{}", is_practical(input))
        }
        _ => panic!()
    }
}

pub fn is_practical(input: u32) -> bool {
    let divisors = calculate_divisors(input);
    for x in 1..input {
        let result = can_sum(x, divisors.clone());
        if !result {
            return false;
        }
    }
    true
}

fn can_sum(mut target: u32, divisors: Vec<u32>) -> bool {
    //println!("{} | {:?}", target, divisors);
    for divisor in divisors.iter().rev() {
        if divisor <= &target {
            target -= divisor;
        }
        if target == 0 {
            return true;
        }
    }
    false
}

fn calculate_divisors(input: u32) -> Vec<u32> {
    let mut result = Vec::new();

    for x in 1..=input {
        if input % x == 0 {
            result.push(x);
        }
    }

    result
}

fn sum_all_practical_numbers() -> u32{
    (1..=10_000).filter(|n| is_practical(*n)).sum()
}

#[cfg(test)]
mod tests {
    use crate::{is_practical, sum_all_practical_numbers};


    #[test]
    fn practical1() {
        assert_eq!(is_practical(1), true);
    }
    
    #[test]
    fn practical2() {
        assert_eq!(is_practical(2), true);
    }
    
    #[test]
    fn practical3() {
        assert_eq!(is_practical(3), false);
    }
    
    #[test]
    fn practical10() {
        assert_eq!(is_practical(10), false);
    }
    
    #[test]
    fn practical12() {
        assert_eq!(is_practical(12), true);
    }

    #[test]
    fn challenge() {
        assert_eq!(sum_all_practical_numbers(), 6_804_107)
    }
}