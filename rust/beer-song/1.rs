

fn main(){
    // for i in (1..5).rev() {
    //     println!("{}",i)
    // }


    pub fn sing( round_starts : u32 , round_ends : u32 ) -> String {
        let mut result : String = "".to_owned();
        for i in (round_ends..round_starts+1).rev() {
            result.push_str(&i.to_string());
        }
        result
    }

    println!("{}",sing(8,6));
    &sing(8,6)[:2];
}
