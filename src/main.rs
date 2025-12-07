use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use trip_analyzer::{DayType, Direction, TripRecord};

mod vehicle_class;
use vehicle_class::light_vehicles;

mod constants;
use constants::*;

mod csv_parser;
pub mod trip_analyzer;

fn main() -> io::Result<()> {
    let csv_dir = Path::new("csv");
    if !csv_dir.exists() {
        eprintln!("Directory 'csv' not found.");
        return Ok(());
    }

    let mut entries: Vec<_> = fs::read_dir(csv_dir)?
        .filter_map(|res| res.ok())
        .map(|dir_entry| dir_entry.path())
        //.filter(|path| path.extension().map_or(false, |ext| ext == "csv"))
        .filter(|path| {
            path.file_name().and_then(|s| s.to_str())
                == Some("2025-08-28 - 573522284 Statement.csv")
        })
        .collect();

    entries.sort();

    let mut all_lines = Vec::new();
    for path in entries {
        let file = fs::File::open(&path)?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            all_lines.push(line?);
        }
    }

    let results = csv_parser::parse_trips(all_lines);

    let summaries = trip_analyzer::analyze_trips(&results);

    for summary in &summaries {
        println!(
            "Transponder: {}, Direction: {:?}",
            summary.transponder_plate, summary.direction
        );
        println!(
            "  Best k={} (Elbow Method): [{}]",
            summary.best_k,
            summary.formatted_centroids.join(", ")
        );

        for centroid_data in &summary.centroids {
            println!("    Trips near {}:", centroid_data.centroid_time);
            for trip_summary in &centroid_data.trips {
                let trip = trip_summary.trip;
                let day_type_str = match &trip.day_type {
                    Some(DayType::Holiday) => "Holiday",
                    Some(DayType::Weekend) => "Weekend",
                    Some(DayType::Weekday) => "Weekday",
                    None => "Unknown",
                };

                let mut optimization_msg = String::new();
                let current_cost = trip.get_total_recorded_cost();

                if let Some(prev_cost) = trip_summary.total_cost_previous_timeslot {
                    if prev_cost < current_cost - 0.005 {
                        optimization_msg.push_str(&format!(" [Cheaper Prev: ${:.2}]", prev_cost));
                    }
                }

                if let Some(next_cost) = trip_summary.total_cost_next_timeslot {
                    if next_cost < current_cost - 0.005 {
                        optimization_msg.push_str(&format!(" [Cheaper Next: ${:.2}]", next_cost));
                    }
                }

                println!(
                    "      - {} {} ({} -> {}: {}km) [{}] [${}]{}",
                    trip.date_of_trip,
                    trip.entry_time,
                    trip.entry_point,
                    trip.exit_point,
                    trip.distance_km,
                    day_type_str,
                    trip.toll_charge,
                    optimization_msg
                );
            }

            if !centroid_data.trips.is_empty() {
                println!(
                    "      Average Entry Time: {}",
                    centroid_data.average_entry_time
                );
                println!(
                    "      Total Distance: {:.3} km",
                    centroid_data.total_distance
                );
                println!(
                    "      Total Toll Charge: ${:.2}",
                    centroid_data.total_toll_charge
                );
                if centroid_data.total_toll_charge_previous_timeslot
                    < centroid_data.total_toll_charge - 0.005
                {
                    println!(
                        "      Total Toll Charge (Previous Timeslot): ${:.2} (Save ${:.2})",
                        centroid_data.total_toll_charge_previous_timeslot,
                        centroid_data.total_toll_charge
                            - centroid_data.total_toll_charge_previous_timeslot
                    );
                } else {
                    println!(
                        "      Total Toll Charge (Previous Timeslot): ${:.2}",
                        centroid_data.total_toll_charge_previous_timeslot
                    );
                }
                if centroid_data.total_toll_charge_next_timeslot
                    < centroid_data.total_toll_charge - 0.005
                {
                    println!(
                        "      Total Toll Charge (Next Timeslot): ${:.2} (Save ${:.2})",
                        centroid_data.total_toll_charge_next_timeslot,
                        centroid_data.total_toll_charge
                            - centroid_data.total_toll_charge_next_timeslot
                    );
                } else {
                    println!(
                        "      Total Toll Charge (Next Timeslot): ${:.2}",
                        centroid_data.total_toll_charge_next_timeslot
                    );
                }
            }
        }
    }

    println!("\nTotal Trips per Transponder:");

    for ((plate, direction), trips) in &results {
        let total_cost: f64 = trips.iter().map(|t| t.get_total_recorded_cost()).sum();
        println!(
            "Transponder: {}, Direction: {:?}, Total Trips: {}, Total Cost: ${:.2}",
            plate,
            direction,
            trips.len(),
            total_cost
        );
        for trip in trips {
            let day_type_str = match &trip.day_type {
                Some(DayType::Holiday) => "Holiday",
                Some(DayType::Weekend) => "Weekend",
                Some(DayType::Weekday) => "Weekday",
                None => "Unknown",
            };
            let calculation_result = trip.calculate_cost();
            let calculated_cost_str = calculation_result
                .map(|(c, _)| format!("{:.2}", c))
                .unwrap_or_else(|| "?".to_string());

            if calculated_cost_str != trip.toll_charge {
                let calculated_dist_str = calculation_result
                    .map(|(_, d)| format!("{:.3}", d))
                    .unwrap_or_else(|| "?".to_string());

                println!(
                    "  - {} {} ({} -> {}: {}km) [{}] [Calc: ${}] [Actual: ${}] [Calc Dist: {}km]",
                    trip.date_of_trip,
                    trip.entry_time,
                    trip.entry_point,
                    trip.exit_point,
                    trip.distance_km,
                    day_type_str,
                    calculated_cost_str,
                    trip.toll_charge,
                    calculated_dist_str
                );
            }
        }
    }

    Ok(())
}
