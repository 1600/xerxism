pub fn square_of_sum(num: i32) -> i32 {
    let mut base: i32 = 0;
    for i in 1..num+1 {
        base += i;
    }
    base * base
}
pub fn sum_of_squares(num: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..num+1 {
        sum += i * i;
    }
    sum
}



pub fn difference(num: i32) -> i32 {
    (square_of_sum(num) - sum_of_squares(num)).abs()
}
