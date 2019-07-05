pub fn mean(numbers: &Vec<i32>) -> f32 {
    if numbers.len() == 0 {
        panic!("The list of numbers is empty")
    }

    let mut sum = 0;
    for &num in numbers {
        sum += num;
    }

    sum as f32 / numbers.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_mean() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(mean(&numbers), 5.5);
    }

    #[test]
    #[should_panic(expected = "The list of numbers is empty")]
    fn panics_if_list_is_empty() {
        mean(&vec![]);
    }
}
