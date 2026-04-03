use toll_optimizer_core::csv_parser;
use toll_optimizer_core::trip_analyzer;

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
    let analysis = trip_analyzer::analyze_trips_by_time(&parsed);

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
    let analysis = trip_analyzer::analyze_trips_by_distance(&parsed);

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
    let analysis_time = trip_analyzer::analyze_trips_by_time(&parsed);

    // Should have 2 summaries (one per transponder since direction is same)
    assert_eq!(analysis_time.len(), 2);
    let plates: Vec<_> = analysis_time.iter().map(|s| &s.transponder_plate).collect();
    assert!(plates.contains(&&"PLATE_A".to_string()));
    assert!(plates.contains(&&"PLATE_B".to_string()));
}
