use std::collections::HashMap;

fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    if numbers.len() == 0 {
        panic!("The list of numbers is empty")
    }

    let mut number_counts = HashMap::new();
    for &num in numbers {
        let counter = number_counts.entry(num).or_insert(0);
        *counter += 1;
    }

    let mut max_count = -1;
    let mut modes = vec![];
    for (num, count) in number_counts {
        if count > max_count {
            max_count = count;
            modes.clear();
            modes.push(num);
        } else if count == max_count {
            modes.push(num);
        }
    }

    modes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_mode() {
        let numbers = vec![1, 1, 2, 3, 4, 5];
        assert_eq!(mode(&numbers), vec![1]);
    }

    #[test]
    fn calculates_several_modes() {
        let numbers = vec![1, 1, 2, 2, 4, 5];
        assert_eq!(mode(&numbers), vec![1, 2]);
    }

    #[test]
    #[should_panic(expected = "The list of numbers is empty")]
    fn panics_if_list_is_empty() {
        mode(&vec![]);
    }
}
