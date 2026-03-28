
pub fn is_leap_year(year: u64) -> bool {
    if year.is_multiple_of(100) == 0 && year.is_multiple_of(4) == 0 {
        return year.is_multiple_of(400) == 0;
    } else if year.is_multiple_of(100) != 0 && year.is_multiple_of(4) == 0{
        return true;
    }
    false
}
