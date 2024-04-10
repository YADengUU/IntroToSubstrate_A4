// sums the collection of u32 int, with param type &[u32], returns type Option<u32> and None if overflow

fn calc_sum(numbers: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;

    for &num in numbers {
        match sum.checked_add(num) {
            Some(result) => sum = result,
            None => return None,
        }
    }

    Some(sum)
}

// main function
fn main() {
    let numbers1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];// try with 1 to 10
    let numbers2 = [u32::MAX, 1];

    match calc_sum(&numbers1) {
        Some(result) => println!("Sum of numbers1: {}", result),
        None => println!("Sorry, overflow!"),
    }

    match calc_sum(&numbers2) {
        Some(result) => println!("Sum of numbers2: {}", result),
        None => println!("Sorry, overflow!"),
    }
}
