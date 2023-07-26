use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!();
    }
    
    println!("{}", lettersum(args[1].as_str()))
}

fn lettersum(string: &str) -> u16 {
    string.as_bytes().into_iter().map(|c| (c - 96) as u16).sum()
}

#[cfg(test)]
mod tests {
    use crate::lettersum;

    #[test]
    fn empty() {
        assert_eq!(lettersum(""), 0);
    }

    #[test]
    fn a() {
        assert_eq!(lettersum("a"), 1);
    }

    #[test]
    fn z() {
        assert_eq!(lettersum("z"), 26);
    }

    #[test]
    fn cab() {
        assert_eq!(lettersum("cab"), 6);
    }

    #[test]
    fn excellent() {
        assert_eq!(lettersum("excellent"), 100);
    }

    #[test]
    fn microspectrophotometries() {
        assert_eq!(lettersum("microspectrophotometries"), 317);
    }
}