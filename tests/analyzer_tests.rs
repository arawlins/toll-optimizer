use toll_optimizer::csv_parser;
use toll_optimizer::trip_analyzer;

#[test]
fn test_time_based_optimization_savings() {
    // A trip that falls into a peak timeslot but could be cheaper if shifted.
    // In 2025 (Aug 28 is a Thursday), 7:00 AM - 9:30 AM is peak.
    // Price at 7:00 AM slot (index 2) for Zone 1 (QEW) is 69.59.
    // Price at 6:00 AM slot (index 1) for Zone 1 (QEW) is 61.18.
    // Shifting to 6:59 AM (prev slot) should save money.
    let csv_lines = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        "\"TEST_PLATE\",\"Light vehicle\",\"28 Aug 25\",\"7:30 AM\",\"QEW\",\"Appleby\",\"10.0\",\"10.00\",\"0.00\",\"0.00\"".to_string(),
    ];

    let parsed = csv_parser::parse_trips(csv_lines.join("\n").as_bytes());
    let analysis = trip_analyzer::analyze_trips_by_time(&parsed.trips);

    assert_eq!(analysis.len(), 1);
    let summary = &analysis[0];
    let centroid = &summary.centroids[0];
    
    // We expect some potential savings to be identified
    assert!(centroid.total_optimized_savings > 0.0, "Should have identified time-based savings for 7:30 AM trip");
    assert!(centroid.optimization_advice.is_some());
}

#[test]
fn test_distance_based_optimization_savings() {
    // Eastbound: QEW (0) to Trafalgar (5)
    // If we exit at Neyagawa (4) instead, it might be cheaper?
    // Actually, the logic tries to shrink the trip by one interchange.
    let csv_lines = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        "\"TEST_PLATE\",\"Light vehicle\",\"28 Aug 25\",\"10:00 AM\",\"QEW\",\"Trafalgar\",\"10.0\",\"5.00\",\"0.00\",\"0.00\"".to_string(),
    ];

    let parsed = csv_parser::parse_trips(csv_lines.join("\n").as_bytes());
    let analysis = trip_analyzer::analyze_trips_by_distance(&parsed.trips);

    assert_eq!(analysis.len(), 1);
    let summary = &analysis[0];
    let centroid = &summary.centroids[0];

    // Distance optimization should try to move Trafalgar back to Neyagawa
    // and check if it's cheaper.
    assert!(centroid.total_optimized_savings >= 0.0); 
}

#[test]
fn test_multiple_transponders_grouping() {
    let csv_lines = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        "\"PLATE_A\",\"Light vehicle\",\"28 Aug 25\",\"10:00 AM\",\"QEW\",\"Trafalgar\",\"10.0\",\"5.00\",\"0.00\",\"0.00\"".to_string(),
        "\"PLATE_B\",\"Light vehicle\",\"28 Aug 25\",\"11:00 AM\",\"QEW\",\"Trafalgar\",\"10.0\",\"5.00\",\"0.00\",\"0.00\"".to_string(),
    ];

    let parsed = csv_parser::parse_trips(csv_lines.join("\n").as_bytes());
    let analysis_time = trip_analyzer::analyze_trips_by_time(&parsed.trips);

    // Should have 2 summaries (one per transponder since direction is same)
    // Should have 2 summaries (one per transponder since direction is same)
    assert_eq!(analysis_time.len(), 2);
    let plates: Vec<_> = analysis_time.iter().map(|s| &s.transponder_plate).collect();
    assert!(plates.contains(&&"PLATE_A".to_string()));
    assert!(plates.contains(&&"PLATE_B".to_string()));
}

#[test]
fn test_weekday_vs_weekend_pricing() {
    // 28 Aug 25 is a Thursday
    let weekday_csv = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        "\"TEST_PLATE\",\"Light vehicle\",\"28 Aug 25\",\"10:00 AM\",\"QEW\",\"Trafalgar\",\"10.0\",\"0.00\",\"0.00\",\"0.00\"".to_string(),
    ];
    let weekday_parsed = csv_parser::parse_trips(weekday_csv.join("\n").as_bytes());
    let weekday_trip = &weekday_parsed.trips[0].1[0];
    let (weekday_cost, _) = weekday_trip.calculate_cost().unwrap();

    // 30 Aug 25 is a Saturday
    let weekend_csv = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        "\"TEST_PLATE\",\"Light vehicle\",\"30 Aug 25\",\"10:00 AM\",\"QEW\",\"Trafalgar\",\"10.0\",\"0.00\",\"0.00\",\"0.00\"".to_string(),
    ];
    let weekend_parsed = csv_parser::parse_trips(weekend_csv.join("\n").as_bytes());
    let weekend_trip = &weekend_parsed.trips[0].1[0];
    let (weekend_cost, _) = weekend_trip.calculate_cost().unwrap();

    println!("Weekday: {}, Weekend: {}", weekday_cost, weekend_cost);
    assert!(weekday_cost > weekend_cost, "Weekday base toll should be higher than weekend base toll");
}

#[test]
fn test_year_boundary_pricing() {
    // Same trip on 28 Aug 2025 vs 28 Aug 2026. Because 28 Aug 26 is a Friday, both are weekdays.
    let year25_csv = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        "\"TEST_PLATE\",\"Light vehicle\",\"28 Aug 25\",\"10:00 AM\",\"QEW\",\"Hwy404\",\"10.0\",\"0.00\",\"0.00\",\"0.00\"".to_string(),
    ];
    let year25_parsed = csv_parser::parse_trips(year25_csv.join("\n").as_bytes());
    let year25_trip = &year25_parsed.trips[0].1[0];
    let (cost_25, _) = year25_trip.calculate_cost().unwrap();

    let year26_csv = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        "\"TEST_PLATE\",\"Light vehicle\",\"28 Aug 26\",\"10:00 AM\",\"QEW\",\"Hwy404\",\"10.0\",\"0.00\",\"0.00\",\"0.00\"".to_string(),
    ];
    let year26_parsed = csv_parser::parse_trips(year26_csv.join("\n").as_bytes());
    let year26_trip = &year26_parsed.trips[0].1[0];
    let (cost_26, _) = year26_trip.calculate_cost_2026().unwrap();

    println!("Cost 2025: {}, Cost 2026: {}", cost_25, cost_26);
    assert!(cost_26 > cost_25, "2026 toll pricing should be higher than 2025 pricing");
}

#[test]
fn test_holiday_classification() {
    // 01 Jan 25 is a Wednesday, normally a weekday, but is a holiday.
    let holiday_csv = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        "\"TEST_PLATE\",\"Light vehicle\",\"01 Jan 25\",\"10:00 AM\",\"QEW\",\"Trafalgar\",\"10.0\",\"0.00\",\"0.00\",\"0.00\"".to_string(),
    ];
    let holiday_parsed = csv_parser::parse_trips(holiday_csv.join("\n").as_bytes());
    let holiday_trip = &holiday_parsed.trips[0].1[0];
    let (holiday_cost, _) = holiday_trip.calculate_cost().unwrap();

    // Compare to weekend pricing
    let weekend_csv = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        "\"TEST_PLATE\",\"Light vehicle\",\"04 Jan 25\",\"10:00 AM\",\"QEW\",\"Trafalgar\",\"10.0\",\"0.00\",\"0.00\",\"0.00\"".to_string(), // Saturday
    ];
    let weekend_parsed = csv_parser::parse_trips(weekend_csv.join("\n").as_bytes());
    let weekend_trip = &weekend_parsed.trips[0].1[0];
    let (weekend_cost, _) = weekend_trip.calculate_cost().unwrap();

    assert_eq!(holiday_cost, weekend_cost, "Holiday toll should match weekend toll calculation");
}

#[test]
fn test_westbound_zone_boundary() {
    // Westbound goes in reverse order. Exit at QEW (which is index 0 in ACCESS_POINTS?).
    // Actually the access points might just be QEW -> Brock. Westbound is Brock -> QEW.
    let wb_csv = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        // Valid westbound trip that ends exactly at QEW or starts exactly at Brock
        "\"TEST_PLATE\",\"Light vehicle\",\"28 Aug 25\",\"10:00 AM\",\"Brock(Hwy7)\",\"QEW\",\"40.0\",\"0.00\",\"0.00\",\"0.00\"".to_string(),
    ];
    let wb_parsed = csv_parser::parse_trips(wb_csv.join("\n").as_bytes());
    let wb_analysis = trip_analyzer::analyze_trips_by_distance(&wb_parsed.trips);
    
    // Testing there was no panic, and that we have a result
    assert_eq!(wb_analysis.len(), 1);
}

#[test]
fn test_midnight_trip_parsing() {
    // 12:00 AM is midnight
    let midnight_csv = vec![
        "\"Transponder/Plate Number\",\"Vehicle Class\",\"Date of Trip\",\"Entry Time\",\"Entry Point\",\"Exit Point\",\"Distance (km)\",\"Toll Charge ($)\",\"Trip Toll Charge ($)\",\"Camera Charge ($)\"".to_string(),
        "\"TEST_PLATE\",\"Light vehicle\",\"28 Aug 25\",\"12:00 AM\",\"QEW\",\"Trafalgar\",\"10.0\",\"0.00\",\"0.00\",\"0.00\"".to_string(),
    ];
    let midnight_parsed = csv_parser::parse_trips(midnight_csv.join("\n").as_bytes());
    
    // There should be 1 trip successfully parsed
    assert_eq!(midnight_parsed.trips.len(), 1, "Midnight trip was not parsed successfully");
    assert_eq!(midnight_parsed.trips[0].1.len(), 1);
    
    // Let's verify timeslot mapping
    let analysis = trip_analyzer::analyze_trips_by_time(&midnight_parsed.trips);
    assert_eq!(analysis.len(), 1);
    let summary = &analysis[0];
    let centroid = &summary.centroids[0];
    // In 2025, 12:00 AM slot is valid and the trip should be placed there
    assert_eq!(centroid.centroid_time, "12:00 AM");
}
