pub fn generate_fibonacci(nth_num: u32) -> u32 {
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut sum: u32 = a + b;
    let mut i: u32 = 1;

    if nth_num == 1 {
        return sum
    }

    while i <= nth_num {
        sum = a + b;
        a = b;
        b = sum;
        i += 1;
    }

    sum
}