fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max = i32::MIN;
    let mut min = i32::MAX;
    let mut sum : f64 = 0.0;
    let mean: f64;

    for number in numbers.iter() {
        if *number > max {
            max = *number;
        }
        if *number < min {
            min = *number
        }

        sum += *number as f64
    }

    mean = sum / numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}
