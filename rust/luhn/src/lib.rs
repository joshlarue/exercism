/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.is_empty() || code.len() == 1 {
        return false;
    }

    let code_no_spaces: String = code.split(" ").collect::<Vec<&str>>().join("");
    let mut code_nums_only: Vec<u32> = vec![];

    code_no_spaces.chars().for_each(|item| {
        let digit_result = item.to_digit(10);
        if digit_result.is_none() {
            return false;
        }
        code_nums_only.push(
            item.to_digit(10)
                .expect("Unable to parse item in code_no_spaces"),
        )
    });

    println!("{:?}", code_nums_only);
    for num in code_nums_only.iter_mut().rev().skip(1).step_by(2) {
        *num *= 2;
        if *num >= 10 {
            *num -= 9
        }
    }
    println!("{:?}", code_nums_only);

    let mut sum: u32 = 0;
    code_nums_only.iter().for_each(|num| sum += num);
    println!("{}", sum);

    if sum % 10 == 0 {
        return true;
    }

    false
}
