//use std::string;
extern crate regex;
#[macro_use] extern crate lazy_static;
use regex::Regex;


fn is_match_helper( text : &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\b[A-Z]+\b").unwrap();
    }
    RE.is_match(text)
}



pub fn reply( input : &str ) -> String {


    if input == "" {
        return String::from("Fine. Be that way!");
    }

    if input.chars().nth(input.len()-1).unwrap() == "?".chars().nth(0).unwrap() {
        return String::from("Sure.");
    }
    
    // let s : Vec<&str> = input.split_whitespace();
    // for i in s {
    //     if i  == i.to_uppercase() {
    //         return String::from("Whoa, chill out!");
    //     }        

    if is_match_helper(input) {
        return String::from("Whoa, chill out!");
    }

    "Whatever.".to_string()
}