/*
Convert a number to a string, the contents of which depend on the number's factors.

- If the number has 3 as a factor, output 'Pling'.
- If the number has 5 as a factor, output 'Plang'.
- If the number has 7 as a factor, output 'Plong'.
- If the number does not have 3, 5, or 7 as a factor,
  just pass the number's digits straight through.
  */

  pub fn raindrops( num : i32) -> String {

    let mut s = String::with_capacity(15);
      if num % 3 == 0 {
        s.push_str("Pling");
      }
      if num % 5 == 0 {
        s.push_str("Plang");
      }
      if num % 7 == 0 {
        s.push_str("Plong");
      }
      if s == "" { return num.to_string(); }
      s
  }