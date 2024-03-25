fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            return year % 400 == 0;
        }
        return true;
    }
    false
}

fn main() {
    let year = 1997; // Change the year here as needed
    if is_leap_year(year) {
        println!("{} is a leap year", year);
    } else {
        println!("{} is not a leap year", year);
    }
}
