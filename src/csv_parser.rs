use crate::trip_analyzer::{Direction, TripRecord};
use crate::{ACCESS_POINT_SYNONYMS, ACCESS_POINTS, OLD_ACCESS_POINTS};
use std::collections::{HashMap, HashSet};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ParseResult {
    pub trips: Vec<((String, Direction), Vec<TripRecord>)>,
    pub total_processed: usize,
    pub total_skipped: usize,
    pub unknown_points: Vec<String>,
}

pub fn parse_trips<R: std::io::Read>(reader: R) -> ParseResult {
    let mut trips_by_transponder: HashMap<String, Vec<TripRecord>> = HashMap::new();
    let mut header_found = false;
    let mut total_processed = 0;
    let mut total_skipped = 0;
    let mut unknown_points = HashSet::new();

    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(reader);

    for result in csv_reader.records() {
        let csv_record = match result {
            Ok(r) => r,
            Err(_) => continue,
        };

        if !header_found {
            if csv_record.iter().any(|field| field.contains("Transponder/Plate Number")) {
                header_found = true;
            }
            continue;
        }

        if let Some(mut record) = TripRecord::from_csv_record(&csv_record) {
            if OLD_ACCESS_POINTS.contains(&record.entry_point.as_str())
                || OLD_ACCESS_POINTS.contains(&record.exit_point.as_str())
            {
                continue;
            }
            if record.vehicle_class != "Light vehicle" {
                continue;
            }

            for &(key, val) in &ACCESS_POINT_SYNONYMS {
                if record.entry_point == key {
                    record.entry_point = val.to_string();
                }
                if record.exit_point == key {
                    record.exit_point = val.to_string();
                }
            }

            let entry_index = ACCESS_POINTS.iter().position(|&r| r == record.entry_point);
            let exit_index = ACCESS_POINTS.iter().position(|&r| r == record.exit_point);

            match (entry_index, exit_index) {
                (Some(entry_idx), Some(exit_idx)) => {
                    record.direction = Some(if exit_idx > entry_idx {
                        Direction::Eastbound
                    } else {
                        Direction::Westbound
                    });

                    let plate = record.transponder_plate.clone();
                    trips_by_transponder.entry(plate).or_default().push(record);
                    total_processed += 1;
                }
                _ => {
                    total_skipped += 1;
                    if entry_index.is_none() {
                        tracing::warn!(
                            entry_point = %record.entry_point,
                            plate = %record.transponder_plate,
                            "Unknown entry point found during CSV parsing"
                        );
                        unknown_points.insert(record.entry_point.clone());
                    }
                    if exit_index.is_none() {
                        tracing::warn!(
                            exit_point = %record.exit_point,
                            plate = %record.transponder_plate,
                            "Unknown exit point found during CSV parsing"
                        );
                        unknown_points.insert(record.exit_point.clone());
                    }
                }
            }
        }
    }

    let mut trips_by_transponder_direction: HashMap<(String, Direction), Vec<TripRecord>> =
        HashMap::new();
    for (plate, trips) in trips_by_transponder {
        for trip in trips {
            if let Some(direction) = &trip.direction {
                trips_by_transponder_direction
                    .entry((plate.clone(), direction.clone()))
                    .or_default()
                    .push(trip);
            }
        }
    }

    let mut results: Vec<((String, Direction), Vec<TripRecord>)> =
        trips_by_transponder_direction.into_iter().collect();

    // Sort results by plate and direction for consistent output
    results.sort_by(|a, b| {
        a.0.0
            .cmp(&b.0.0)
            .then_with(|| format!("{:?}", a.0.1).cmp(&format!("{:?}", b.0.1)))
    });

    let mut unknown_points_vec: Vec<String> = unknown_points.into_iter().collect();
    unknown_points_vec.sort();

    ParseResult {
        trips: results,
        total_processed,
        total_skipped,
        unknown_points: unknown_points_vec,
    }
}
