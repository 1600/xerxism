pub fn verse( round : u32 ) -> String {

    return match round {
        0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", round, round),
        2 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n", round, round ),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", round, round, round-1),
    } 
}

pub fn sing( round_starts : u32 , round_ends : u32 ) -> String {
    let mut result : String = "".to_owned();
    for i in (round_ends+1..round_starts+1).rev() {
        result.push_str(&verse(i));
        result.push_str("\n");
    }
    result.push_str(&verse(round_ends));
    result
}