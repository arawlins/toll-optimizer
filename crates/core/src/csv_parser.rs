use crate::trip_analyzer::{Direction, TripRecord};
use crate::{ACCESS_POINT_SYNONYMS, ACCESS_POINTS, OLD_ACCESSS_POINTS};
use std::collections::HashMap;

pub fn parse_trips<I>(lines: I) -> Vec<((String, Direction), Vec<TripRecord>)>
where
    I: IntoIterator<Item = String>,
{
    let mut trips_by_transponder: HashMap<String, Vec<TripRecord>> = HashMap::new();
    let mut header_found = false;

    for line in lines {
        if !header_found {
            if line.contains("Transponder/Plate Number") {
                header_found = true;
            }
            continue;
        }

        if let Some(mut record) = TripRecord::from_csv_line(&line) {
            if OLD_ACCESSS_POINTS.contains(&record.entry_point.as_str())
                || OLD_ACCESSS_POINTS.contains(&record.exit_point.as_str())
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

            if let (Some(entry_idx), Some(exit_idx)) = (entry_index, exit_index) {
                record.direction = Some(if exit_idx > entry_idx {
                    Direction::Eastbound
                } else {
                    Direction::Westbound
                });

                let plate = record.transponder_plate.clone();
                trips_by_transponder.entry(plate).or_default().push(record);
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

    results
}
