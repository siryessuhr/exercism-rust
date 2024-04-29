pub fn is_armstrong_number(num: u32) -> bool {
    let len = num.to_string().len() as u32;
    num as u64 == num.to_string()
        .chars()
        .fold(0, |acc, x| acc + x.to_digit(10).unwrap().pow(len) as u64)
}
