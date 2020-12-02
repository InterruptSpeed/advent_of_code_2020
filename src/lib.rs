//! advent of code 2020

/// two numbers add up to 2020
pub fn adds_up(x: u64, y: u64) -> bool {
    (x + y) == 2020
}

/// product of two numbers
pub fn multiply(x: u64, y: u64) -> u64 {
    x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = adds_up(1721, 299);
        assert!(result);
    }

    #[test]
    fn product_works() {
        let result = multiply(1721, 299);
        assert_eq!(result, 514579); 
    }

    #[test]
    fn sort_works() {
        let mut vals = [1721, 979, 366, 299, 675, 1456];
        let expected = [299, 366, 675, 979, 1456, 1721];

        vals.sort();

        let matching = vals.iter().zip(&expected).filter(|&(a, b)| a == b).count();

        assert!(matching == vals.len() && matching == expected.len());
    }
}