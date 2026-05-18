use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

use toll_optimizer::{
    AnalysisMarkdownReport, DayType, SingleTripMarkdownReport, VehicleClass,
    analyze_trips_by_distance, analyze_trips_by_time, calculate_single_trip_cost, get_pricing,
    parse_trips, print_markdown, print_pricing_markdown, print_single_trip_markdown,
};

#[derive(Parser, Debug)]
#[command(author, version, long_about = None, arg_required_else_help = true)]
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

    /// Vehicle class for pricing (e.g., "Light vehicle", "Heavy Single Unit")
    #[arg(long, value_name = "CLASS", default_value = "Light vehicle")]
    vehicle_class: String,

    /// Output results in JSON format
    #[arg(short, long)]
    json: bool,

    /// Output results in Markdown format
    #[arg(short, long)]
    markdown: bool,

    /// List all recognized 407 ETR access points
    #[arg(long)]
    list_access_points: bool,

    /// List all 407 ETR pricing timeslots
    #[arg(long)]
    list_timeslots: bool,

    /// Entry point for a single trip calculation
    #[arg(long, requires = "exit")]
    entry: Option<String>,

    /// Exit point for a single trip calculation
    #[arg(long, requires = "entry")]
    exit: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.list_access_points {
        let mut points = toll_optimizer::ACCESS_POINTS.to_vec();
        points.sort();

        if args.json {
            println!("{}", serde_json::to_string_pretty(&points)?);
            return Ok(());
        }

        if args.markdown {
            println!("# Recognized 407 ETR Access Points\n");
            for point in points {
                println!("- {}", point);
            }
            return Ok(());
        }

        println!("Recognized 407 ETR Access Points:");
        for point in points {
            println!("  - {}", point);
        }
        return Ok(());
    }

    if args.list_timeslots {
        if args.json {
            let output = serde_json::json!({
                "weekday": toll_optimizer::WEEKDAY_TIMESLOTS_2026,
                "weekend_holiday": toll_optimizer::WEEKEND_TIMESLOTS_2026,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            return Ok(());
        }

        if args.markdown {
            println!("# 407 ETR Pricing Timeslots\n");
            println!("## Weekday");
            for slot in toll_optimizer::WEEKDAY_TIMESLOTS_2026 {
                println!("- {}", slot);
            }
            println!("\n## Weekend/Holiday");
            for slot in toll_optimizer::WEEKEND_TIMESLOTS_2026 {
                println!("- {}", slot);
            }
            return Ok(());
        }

        println!("407 ETR Pricing Timeslots:");
        println!("  Weekday:");
        for slot in toll_optimizer::WEEKDAY_TIMESLOTS_2026 {
            println!("    - {}", slot);
        }
        println!("  Weekend/Holiday:");
        for slot in toll_optimizer::WEEKEND_TIMESLOTS_2026 {
            println!("    - {}", slot);
        }
        return Ok(());
    }

    if let (Some(entry), Some(exit)) = (args.entry, args.exit) {
        let now = Local::now();
        let date_str = args
            .date
            .unwrap_or_else(|| now.format("%Y-%m-%d").to_string());
        let time_str = args.time.unwrap_or_else(|| now.format("%H:%M").to_string());
        let vehicle_class = args
            .vehicle_class
            .parse::<VehicleClass>()
            .map_err(|_| anyhow::anyhow!("Invalid vehicle class: {}", args.vehicle_class))?;

        let (cost, dist, direction, day_type) =
            calculate_single_trip_cost(&entry, &exit, &date_str, &time_str, vehicle_class)?;

        if args.json {
            let output = serde_json::json!({
                "entry": entry,
                "exit": exit,
                "date": date_str,
                "time": time_str,
                "vehicle_class": vehicle_class,
                "distance_km": dist,
                "direction": direction,
                "day_type": day_type,
                "base_toll": cost,
                "trip_charge": 1.00,
                "total_estimated_cost": cost + 1.00,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            return Ok(());
        }

        if args.markdown {
            print_single_trip_markdown(SingleTripMarkdownReport {
                entry: &entry,
                exit: &exit,
                date: &date_str,
                time: &time_str,
                class: vehicle_class.to_str(),
                distance_km: dist,
                direction: &direction,
                day_type: &day_type,
                cost,
            });
            return Ok(());
        }

        println!("--- Single Trip Cost Analysis ---");
        println!("Route: {} -> {}", entry, exit);
        println!("Date:  {}", date_str);
        println!("Time:  {}", time_str);
        println!("Class: {}", vehicle_class.to_str());
        println!("Distance:  {:.3} km", dist);
        println!("Direction: {:?}", direction);
        println!("Day Type:  {:?}", day_type);
        println!("Base Toll: ${:.2}", cost);
        println!("Trip Charge: $1.00");
        println!("Total Estimated Cost: ${:.2}", cost + 1.00);

        return Ok(());
    }

    if args.current_price {
        let now = Local::now();
        let date = args
            .date
            .unwrap_or_else(|| now.format("%Y-%m-%d").to_string());
        let time = args.time.unwrap_or_else(|| now.format("%H:%M").to_string());
        let vehicle_class = args
            .vehicle_class
            .parse::<VehicleClass>()
            .map_err(|_| anyhow::anyhow!("Invalid vehicle class: {}", args.vehicle_class))?;

        let pricing = get_pricing(&date, &time, vehicle_class)?;

        if args.json {
            let current_avg = (pricing.current.average_eb + pricing.current.average_wb) / 2.0;
            let next_avg = (pricing.next.average_eb + pricing.next.average_wb) / 2.0;
            let advice = if next_avg < current_avg - 0.001 {
                format!(
                    "Waiting for the next timeslot ({}) could save you money!",
                    pricing.next.timeslot
                )
            } else if next_avg > current_avg + 0.001 {
                format!(
                    "Leave now ({}) to avoid higher rates in the next timeslot!",
                    pricing.current.timeslot
                )
            } else {
                "Rates are expected to remain stable.".to_string()
            };

            let output = serde_json::json!({
                "date": date,
                "time": time,
                "pricing": pricing,
                "advice": advice,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            return Ok(());
        }

        if args.markdown {
            print_pricing_markdown(&pricing, &date, &time);
            return Ok(());
        }

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

    let results = parse_trips(file);

    let summaries_by_time = analyze_trips_by_time(&results.trips);
    let summaries_by_distance = analyze_trips_by_distance(&results.trips);

    let total_cost: f64 = results
        .trips
        .iter()
        .flat_map(|(_, trips)| trips.iter())
        .map(|t| t.get_total_recorded_cost())
        .sum();

    let total_time_savings: f64 = summaries_by_time
        .iter()
        .flat_map(|s| s.centroids.iter())
        .map(|c| c.total_optimized_savings)
        .sum();

    let total_distance_savings: f64 = summaries_by_distance
        .iter()
        .flat_map(|s| s.centroids.iter())
        .map(|c| c.total_optimized_savings)
        .sum();

    if args.json {
        let output = serde_json::json!({
            "summary": {
                "total_processed": results.total_processed,
                "total_skipped": results.total_skipped,
                "total_cost": total_cost,
                "total_potential_time_savings": total_time_savings,
                "total_potential_distance_savings": total_distance_savings,
                "unknown_points": results.unknown_points,
                "unknown_vehicle_classes": results.unknown_vehicle_classes,
            },
            "time_based_analysis": summaries_by_time,
            "distance_based_analysis": summaries_by_distance,
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    if args.markdown {
        print_markdown(AnalysisMarkdownReport {
            summaries_by_time: &summaries_by_time,
            summaries_by_distance: &summaries_by_distance,
            total_processed: results.total_processed,
            total_skipped: results.total_skipped,
            total_cost,
            total_time_savings,
            total_distance_savings,
            unknown_points: &results.unknown_points,
            unknown_vehicle_classes: &results.unknown_vehicle_classes,
            camera_charges: &results.camera_charges,
        });
        return Ok(());
    }

    println!("--- Processing Summary ---");
    println!("Trips Processed: {}", results.total_processed);
    println!("Trips Skipped:   {}", results.total_skipped);
    println!("Total Bill Cost: ${:.2}", total_cost);
    println!(
        "Potential Time-Based Savings:     ${:.2}",
        total_time_savings
    );
    println!(
        "Potential Distance-Based Savings: ${:.2}",
        total_distance_savings
    );

    if !results.unknown_points.is_empty() {
        println!("\nUnrecognized Access Points:");
        for point in &results.unknown_points {
            println!("  - {} | NOT RECOGNIZED", point);
        }
    }
    if !results.unknown_vehicle_classes.is_empty() {
        println!("\nUnrecognized Vehicle Classes:");
        for class in &results.unknown_vehicle_classes {
            println!("  - {} | NOT RECOGNIZED", class);
        }
    }

    if !results.camera_charges.is_empty() {
        println!("\nCamera Charges by Transponder/Plate:");
        let mut plates: Vec<_> = results.camera_charges.keys().collect();
        plates.sort();
        let mut show_recommendation = false;
        for plate in plates {
            let charge = results.camera_charges[plate];
            println!("  - {}: ${:.2}", plate, charge);
            if charge > 31.50 {
                show_recommendation = true;
            }
        }
        if show_recommendation {
            println!(
                "\nTip: Leasing a transponder for $31.50 (plus applicable taxes) per year will save you money on the camera charges."
            );
        }
    }
    println!();

    println!("--- Time-Based Analysis ---");
    for summary in &summaries_by_time {
        println!(
            "Transponder: {}, Direction: {:?}",
            summary.transponder_plate, summary.direction
        );

        for centroid_data in &summary.centroids {
            println!("    Trips near {}:", centroid_data.centroid_time);

            if true {
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

                    if let Some(prev_cost) = trip_summary.total_cost_previous_timeslot
                        && prev_cost < current_cost - 0.005
                    {
                        let target_msg = trip_summary
                            .prev_timeslot_target
                            .as_ref()
                            .map(|t| format!(" (<= {})", t))
                            .unwrap_or_default();
                        optimization_msg
                            .push_str(&format!(" [Cheaper Prev: ${:.2}{}]", prev_cost, target_msg));
                    }

                    if let Some(next_cost) = trip_summary.total_cost_next_timeslot
                        && next_cost < current_cost - 0.005
                    {
                        let target_msg = trip_summary
                            .next_timeslot_target
                            .as_ref()
                            .map(|t| format!(" (>= {})", t))
                            .unwrap_or_default();
                        optimization_msg
                            .push_str(&format!(" [Cheaper Next: ${:.2}{}]", next_cost, target_msg));
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

    println!("\n--- Distance-Based Analysis ---");
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

            if true {
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
