use std::collections::HashSet;
use std::iter::Sum;



pub fn sum_of_multiples( ceiling : i64 , factors : &Vec<i64> ) -> i64 {
    let mut sum = 0;
    let mut multiples = HashSet::new();
    for num in factors {
        //println!("{}",i);
        let mut i = 1;
        while true {
            if ceiling - i*num > 0 {
                multiples.insert(i*num);
                println!("adding {}, i is {}, num is {}", i*num, i , num);
                i+=1;
            } else {
                break;
            }
        }
    }
    println!("{:?}",multiples);
    for i in multiples.iter() {
        sum += *i;          // note i is a reference to the elements in Hash
    }
    sum
}
