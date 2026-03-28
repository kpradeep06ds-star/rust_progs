
pub fn is_leap_year(year: u64) -> bool {
    if year.is_multiple_of(100)  && year.is_multiple_of(4) {
        return year.is_multiple_of(400) ;
    } else if !year.is_multiple_of(100)  && year.is_multiple_of(4){
        return true;
    }
    false
}
