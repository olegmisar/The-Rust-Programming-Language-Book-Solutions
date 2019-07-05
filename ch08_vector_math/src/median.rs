pub fn median(numbers: &Vec<i32>) -> f32 {
    if numbers.len() == 0 {
        panic!("The list of numbers is empty")
    }

    let mut numbers = numbers.clone();
    numbers.sort_unstable();

    let len = numbers.len();
    if len % 2 == 1 {
        numbers[len / 2] as f32
    } else {
        (numbers[len / 2 - 1] + numbers[len / 2]) as f32 / 2.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_median_of_list_with_odd_length() {
        let numbers = vec![1, 2, 5, 6, 2];
        assert_eq!(median(&numbers), 2.);
    }

    #[test]
    fn calculates_median_of_list_with_even_length() {
        let numbers = vec![1, 2, 5, 6, 2, 3];
        assert_eq!(median(&numbers), 2.5);
    }

    #[test]
    #[should_panic(expected = "The list of numbers is empty")]

    fn panics_if_list_is_empty() {
        median(&vec![]);
    }
}
