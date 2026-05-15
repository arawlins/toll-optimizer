use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use chrono::Local;

use toll_optimizer::{DayType, csv_parser, md_output, trip_analyzer};

/// Toll Optimizer: Analyze 407 ETR statements and suggest optimizations.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the 407 ETR CSV statement file
    #[arg(value_name = "FILE")]
    csv_file: Option<PathBuf>,

    /// Get current and next timeslot pricing
    #[arg(long)]
    current_price: bool,

    /// Override date for pricing (YYYY-MM-DD)
    #[arg(long, value_name = "DATE")]
    date: Option<String>,

    /// Override time for pricing (HH:MM AM/PM or HH:MM)
    #[arg(long, value_name = "TIME")]
    time: Option<String>,


    /// Output results in JSON format
    #[arg(short, long)]
    json: bool,

    /// Output results in Markdown format
    #[arg(short, long)]
    markdown: bool,

    /// Show verbose output (detailed trip listings)
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.current_price {
        let now = Local::now();
        let date = args.date.unwrap_or_else(|| now.format("%Y-%m-%d").to_string());
        let time = args.time.unwrap_or_else(|| now.format("%H:%M").to_string());

        let pricing = trip_analyzer::get_pricing(&date, &time)?;

        println!("--- Live Pricing Analysis for {} at {} ---", date, time);
        println!("Day Type: {}", pricing.day_type);
        println!("Current Timeslot: {}", pricing.current.timeslot);
        println!("  Average EB: {:.2}¢/km", pricing.current.average_eb);
        println!("  Average WB: {:.2}¢/km", pricing.current.average_wb);
        println!("Next Timeslot: {}", pricing.next.timeslot);
        println!("  Average EB: {:.2}¢/km", pricing.next.average_eb);
        println!("  Average WB: {:.2}¢/km", pricing.next.average_wb);

        let current_avg = (pricing.current.average_eb + pricing.current.average_wb) / 2.0;
        let next_avg = (pricing.next.average_eb + pricing.next.average_wb) / 2.0;

        if next_avg < current_avg - 0.001 {
            println!("\nOptimization Tip: Waiting for the next timeslot could save you money!");
        } else if next_avg > current_avg + 0.001 {
            println!("\nOptimization Tip: Leave now to avoid higher rates in the next timeslot!");
        }

        return Ok(());
    }

    let csv_file = args.csv_file.ok_or_else(|| {
        anyhow::anyhow!("CSV file is required unless --current-price is specified.")
    })?;

    if !csv_file.exists() {
        anyhow::bail!("File '{}' not found.", csv_file.display());
    }

    let file = fs::File::open(&csv_file)
        .with_context(|| format!("Failed to open file: {}", csv_file.display()))?;

    let results = csv_parser::parse_trips(file);

    let summaries_by_time = trip_analyzer::analyze_trips_by_time(&results);
    let summaries_by_distance = trip_analyzer::analyze_trips_by_distance(&results);

    if args.json {
        let output = serde_json::json!({
            "time_based_analysis": summaries_by_time,
            "distance_based_analysis": summaries_by_distance,
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    if args.markdown {
        md_output::print_markdown(&summaries_by_time, &summaries_by_distance, args.verbose);
        return Ok(());
    }

    println!("--- Time-Based Clustering Analysis ---");
    for summary in &summaries_by_time {
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
                            optimization_msg.push_str(&format!(
                                " [Cheaper Prev: ${:.2}{}]",
                                prev_cost, target_msg
                            ));
                        }
                    }

                    if let Some(next_cost) = trip_summary.total_cost_next_timeslot {
                        if next_cost < current_cost - 0.005 {
                            let target_msg = trip_summary
                                .next_timeslot_target
                                .as_ref()
                                .map(|t| format!(" (>= {})", t))
                                .unwrap_or_default();
                            optimization_msg.push_str(&format!(
                                " [Cheaper Next: ${:.2}{}]",
                                next_cost, target_msg
                            ));
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

    println!("\n--- Distance-Based Clustering Analysis ---");
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
