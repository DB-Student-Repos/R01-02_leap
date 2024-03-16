pub fn is_leap_year(year: u64) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                return true; // Divisible by 400, leap year
            }
            return false; // Divisible by 100 but not by 400, not a leap year
        }
        return true; // Divisible by 4 but not by 100, leap year
    }
    false // Not divisible by 4, not a leap year
}