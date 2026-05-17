use crate::trip_analyzer::{Direction, TripRecord};
use crate::{ACCESS_POINT_SYNONYMS, ACCESS_POINTS, OLD_ACCESS_POINTS};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize)]
pub struct ParseResult {
    pub trips: Vec<((String, Direction), Vec<TripRecord>)>,
    pub total_processed: usize,
    pub total_skipped: usize,
    pub unknown_points: Vec<String>,
    pub unknown_vehicle_classes: Vec<String>,
    pub camera_charges: HashMap<String, f64>,
}

pub fn parse_trips<R: std::io::Read>(reader: R) -> ParseResult {
    let mut trips_by_transponder: HashMap<String, Vec<TripRecord>> = HashMap::new();
    let mut header_found = false;
    let mut total_processed = 0;
    let mut total_skipped = 0;
    let mut unknown_points = HashSet::new();
    let mut unknown_vehicle_classes = HashSet::new();
    let mut camera_charges: HashMap<String, f64> = HashMap::new();

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
            if csv_record
                .iter()
                .any(|field| field.contains("Transponder/Plate Number"))
            {
                header_found = true;
            }
            continue;
        }

        let record_opt = TripRecord::from_csv_record(&csv_record);
        if record_opt.is_none() && header_found {
            total_skipped += 1;
            if csv_record.len() > 1 {
                let vc_str = csv_record[1].trim().trim_matches('"');
                if !vc_str.is_empty()
                    && crate::trip_analyzer::VehicleClass::from_str(vc_str).is_none()
                {
                    unknown_vehicle_classes.insert(vc_str.to_string());
                }
            }
            continue;
        }

        if let Some(mut record) = record_opt {
            if OLD_ACCESS_POINTS
                .iter()
                .any(|&p| p.eq_ignore_ascii_case(&record.entry_point))
                || OLD_ACCESS_POINTS
                    .iter()
                    .any(|&p| p.eq_ignore_ascii_case(&record.exit_point))
            {
                total_skipped += 1;
                continue;
            }

            for &(key, val) in &ACCESS_POINT_SYNONYMS {
                if record.entry_point.eq_ignore_ascii_case(key) {
                    record.entry_point = val.to_string();
                }
                if record.exit_point.eq_ignore_ascii_case(key) {
                    record.exit_point = val.to_string();
                }
            }

            let entry_index = ACCESS_POINTS
                .iter()
                .position(|&r| r.eq_ignore_ascii_case(&record.entry_point));
            let exit_index = ACCESS_POINTS
                .iter()
                .position(|&r| r.eq_ignore_ascii_case(&record.exit_point));

            match (entry_index, exit_index) {
                (Some(entry_idx), Some(exit_idx)) => {
                    record.direction = Some(if exit_idx > entry_idx {
                        Direction::Eastbound
                    } else {
                        Direction::Westbound
                    });

                    let plate = record.transponder_plate.clone();
                    let camera_charge = record.camera_charge.trim().parse::<f64>().unwrap_or(0.0);
                    if camera_charge > 0.0 {
                        *camera_charges.entry(plate.clone()).or_insert(0.0) += camera_charge;
                    }
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

    let mut unknown_vehicle_classes_vec: Vec<String> =
        unknown_vehicle_classes.into_iter().collect();
    unknown_vehicle_classes_vec.sort();

    ParseResult {
        trips: results,
        total_processed,
        total_skipped,
        unknown_points: unknown_points_vec,
        unknown_vehicle_classes: unknown_vehicle_classes_vec,
        camera_charges,
    }
}
