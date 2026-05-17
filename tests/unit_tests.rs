use toll_optimizer::trip_analyzer::{format_minutes_to_time, parse_time_to_minutes};

#[test]
fn test_parse_time_to_minutes() {
    assert_eq!(parse_time_to_minutes("12:00 AM"), Some(0));
    assert_eq!(parse_time_to_minutes("12:01 AM"), Some(1));
    assert_eq!(parse_time_to_minutes("1:00 AM"), Some(60));
    assert_eq!(parse_time_to_minutes("11:59 AM"), Some(11 * 60 + 59));
    assert_eq!(parse_time_to_minutes("12:00 PM"), Some(12 * 60));
    assert_eq!(parse_time_to_minutes("12:30 PM"), Some(12 * 60 + 30));
    assert_eq!(parse_time_to_minutes("1:00 PM"), Some(13 * 60));
    assert_eq!(parse_time_to_minutes("11:59 PM"), Some(23 * 60 + 59));
}

#[test]
fn test_parse_time_invalid() {
    assert_eq!(parse_time_to_minutes("12:00"), None);
    assert_eq!(parse_time_to_minutes("AM"), None);
    assert_eq!(parse_time_to_minutes("13:00 AM"), None); // Logic handles 13 % 12, but input is weird
    assert_eq!(parse_time_to_minutes("abc"), None);
}

#[test]
fn test_format_minutes_to_time() {
    assert_eq!(format_minutes_to_time(0), "12:00 AM");
    assert_eq!(format_minutes_to_time(1), "12:01 AM");
    assert_eq!(format_minutes_to_time(60), "1:00 AM");
    assert_eq!(format_minutes_to_time(12 * 60), "12:00 PM");
    assert_eq!(format_minutes_to_time(13 * 60), "1:00 PM");
    assert_eq!(format_minutes_to_time(23 * 60 + 59), "11:59 PM");
}
