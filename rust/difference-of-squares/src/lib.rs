pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for num in 0..n + 1 {
        sum += num;
    }
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    // formula for sum of squares
    let sum_squares = (n * (n + 1) * (2 * n + 1)) / 6;
    sum_squares
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
