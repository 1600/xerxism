

/*plain
leap year
  on every year that is evenly divisible by 4
  except every year that is evenly divisible by 100
    unless the year is also evenly divisible by 400
*/

pub fn is_leap_year( yr : i32 ) -> bool {

    if yr % 4 != 0 {
        return false;
    } else if yr % 400 == 0 {
        return true;
    } else if yr % 100 == 0 {
        return false;
    }
    true


}