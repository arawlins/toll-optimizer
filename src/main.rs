use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use toll_optimizer::{DayType, csv_parser, trip_analyzer};

/// Toll Optimizer: Analyze 407 ETR statements and suggest optimizations.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the 407 ETR CSV statement file
    #[arg(value_name = "FILE")]
    csv_file: PathBuf,

    /// Output results in JSON format
    #[arg(short, long)]
    json: bool,

    /// Show verbose output (detailed trip listings)
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.csv_file.exists() {
        anyhow::bail!("File '{}' not found.", args.csv_file.display());
    }

    let file = fs::File::open(&args.csv_file)
        .with_context(|| format!("Failed to open file: {}", args.csv_file.display()))?;

    let results = csv_parser::parse_trips(file);

    if args.json {
        // For now, we'll just print a placeholder if JSON is requested
        // In a real scenario, we'd serialize the summaries.
        println!("JSON output requested (not yet fully implemented in this refactor).");
    }

    println!("--- Time-Based Clustering Analysis ---");
    let summaries = trip_analyzer::analyze_trips_by_time(&results);

    for summary in &summaries {
        println!(
            "Transponder: {}, Direction: {:?}",
            summary.transponder_plate, summary.direction
        );


        for centroid_data in &summary.centroids {
            println!("    Trips near {}:", centroid_data.centroid_time);
            
            if args.verbose {
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
                            let target_msg = trip_summary
                                .prev_timeslot_target
                                .as_ref()
                                .map(|t| format!(" (<= {})", t))
                                .unwrap_or_default();
                            optimization_msg
                                .push_str(&format!(" [Cheaper Prev: ${:.2}{}]", prev_cost, target_msg));
                        }
                    }

                    if let Some(next_cost) = trip_summary.total_cost_next_timeslot {
                        if next_cost < current_cost - 0.005 {
                            let target_msg = trip_summary
                                .next_timeslot_target
                                .as_ref()
                                .map(|t| format!(" (>= {})", t))
                                .unwrap_or_default();
                            optimization_msg
                                .push_str(&format!(" [Cheaper Next: ${:.2}{}]", next_cost, target_msg));
                        }
                    }

                    println!(
                        "      - {} {} ({} -> {}: {}km) [{}] [${:.2}]{}",
                        trip.date_of_trip,
                        trip.entry_time,
                        trip.entry_point,
                        trip.exit_point,
                        trip.distance_km,
                        day_type_str,
                        trip.get_total_recorded_cost(),
                        optimization_msg
                    );
                }
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
                }

                if centroid_data.total_optimized_savings > 0.0 {
                    println!(
                        "      Total Potential Savings: ${:.2}",
                        centroid_data.total_optimized_savings
                    );
                }
            }
        }
    }

    if args.verbose {
        println!("\nDetailed Trip Validation:");

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
    }

    let summaries_by_distance = trip_analyzer::analyze_trips_by_distance(&results);

    println!("\n--- Distance-Based (Zones) Clustering Analysis ---");
    for summary in &summaries_by_distance {
        println!(
            "Transponder: {}, Direction: {:?}",
            summary.transponder_plate, summary.direction
        );
        
        for centroid_data in &summary.centroids {
            println!(
                "    {} -> {} (Avg: {:.2} km):",
                centroid_data
                    .representative_entry
                    .as_deref()
                    .unwrap_or("Unknown"),
                centroid_data
                    .representative_exit
                    .as_deref()
                    .unwrap_or("Unknown"),
                centroid_data.average_distance
            );
            println!(
                "      Total Toll Charge: ${:.2}",
                centroid_data.total_toll_charge
            );
            
            if args.verbose {
                for trip_summary in &centroid_data.trips {
                    let trip = trip_summary.trip;
                    let day_type_str = match &trip.day_type {
                        Some(DayType::Holiday) => "Holiday",
                        Some(DayType::Weekend) => "Weekend",
                        Some(DayType::Weekday) => "Weekday",
                        None => "Unknown",
                    };

                    let optimization_msg = if let Some(note) = &trip_summary.optimization_note {
                        format!(" [{}]", note)
                    } else {
                        String::new()
                    };

                    println!(
                        "      - {} {} ({} -> {}: {}km) [{}] [${:.2}]{}",
                        trip.date_of_trip,
                        trip.entry_time,
                        trip.entry_point,
                        trip.exit_point,
                        trip.distance_km,
                        day_type_str,
                        trip.get_total_recorded_cost(),
                        optimization_msg
                    );
                }
            }

            if centroid_data.total_optimized_savings > 0.0 {
                println!(
                    "      Total Potential Savings: ${:.2}",
                    centroid_data.total_optimized_savings
                );
            }
        }
    }

    Ok(())
}
