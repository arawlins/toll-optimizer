use toll_optimizer::csv_parser;
use toll_optimizer::trip_analyzer::Direction;

#[test]
fn test_parse_valid_csv_line() {
    let lines = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        "\"TEST_PLATE_001\",\"Light vehicle\",\"28 Aug 25\",\"10:00 AM\",\"QEW\",\"Trafalgar\",\"10.0\",\"5.00\",\"0.00\",\"0.00\"".to_string(),
    ];

    let results = csv_parser::parse_trips(lines.join("\n").as_bytes());
    assert_eq!(results.trips.len(), 1);
    let ((plate, direction), trips) = &results.trips[0];
    assert_eq!(plate, "TEST_PLATE_001");
    assert_eq!(direction, &Direction::Eastbound);
    assert_eq!(trips.len(), 1);
    assert_eq!(trips[0].entry_point, "QEW");
    assert_eq!(trips[0].exit_point, "Trafalgar");
}

#[test]
fn test_parse_csv_with_synonyms() {
    let lines = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        "\"TEST_PLATE_001\",\"Light vehicle\",\"28 Aug 25\",\"10:00 AM\",\"Brock\",\"Trafalgar\",\"10.0\",\"5.00\",\"0.00\",\"0.00\"".to_string(),
    ];

    let results = csv_parser::parse_trips(lines.join("\n").as_bytes());
    assert_eq!(results.trips.len(), 1);
    let ((_, _), trips) = &results.trips[0];
    assert_eq!(trips[0].entry_point, "Brock(Hwy7)");
}

#[test]
fn test_parse_skips_non_light_vehicles() {
    let lines = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        "\"TEST_PLATE_HV\",\"Heavy Single Unit\",\"28 Aug 25\",\"10:00 AM\",\"QEW\",\"Trafalgar\",\"10.0\",\"5.00\",\"0.00\",\"0.00\"".to_string(),
    ];

    let results = csv_parser::parse_trips(lines.join("\n").as_bytes());
    assert_eq!(results.trips.len(), 0);
}

#[test]
fn test_parse_unknown_points() {
    let lines = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        "\"TEST_PLATE_001\",\"Light vehicle\",\"28 Aug 25\",\"10:00 AM\",\"Unknown Entry\",\"Unknown Exit\",\"10.0\",\"5.00\",\"0.00\",\"0.00\"".to_string(),
    ];

    let results = csv_parser::parse_trips(lines.join("\n").as_bytes());
    // Should skip trips with unknown entry/exit points
    assert_eq!(results.trips.len(), 0);
    assert_eq!(results.total_skipped, 1);
}

#[test]
fn test_parse_empty_input() {
    let lines: Vec<String> = vec![];
    let results = csv_parser::parse_trips(lines.join("\n").as_bytes());
    assert_eq!(results.trips.len(), 0);
}

#[test]
fn test_parse_malformed_lines() {
    let lines = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        "invalid line".to_string(),
        "\"SHORT\",\"Light vehicle\"".to_string(),
    ];

    let results = csv_parser::parse_trips(lines.join("\n").as_bytes());
    assert_eq!(results.trips.len(), 0);
}
