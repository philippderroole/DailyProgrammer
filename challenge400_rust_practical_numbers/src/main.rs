use std::env;

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