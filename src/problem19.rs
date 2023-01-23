/// You are given the following information, but you may prefer to do some
/// research for yourself.
///    1 Jan 1900 was a Monday.
///    Thirty days has September,
///    April, June and November.
///    All the rest have thirty-one,
///    Saving February alone,
///    Which has twenty-eight, rain or shine.
///    And on leap years, twenty-nine.
///    A leap year occurs on any year evenly divisible by 4, but not on a
///    century unless it is divisible by 400.
/// How many Sundays fell on the first of the month during the twentieth
/// century (1 Jan 1901 to 31 Dec 2000)?

pub fn main() -> i32 {
    let _days_of_the_week = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    let _month_names = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let days_per_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut res = 0; // number of Sundays on first of the month
    let mut day = 1;
    for year in 1900..=2000 {
        for month in 0..12 {
            let mut days = days_per_month[month];
            if month == 1 && (year % 400 == 0 || year % 100 != 0 && year % 4 == 0) {
                days = 29;
            }
            day = (day + days) % 7;
            if year > 1900 && day == 0 {
                res += 1;
            }
        }
    }
    return res;
}
