pub fn reverse(input: &str) -> String {
    let reversed = input.chars().rev().collect::<String>();
    reversed
}
