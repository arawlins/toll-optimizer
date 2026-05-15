use crate::trip_analyzer::{DayType, TransponderSummaryByDistance, TransponderSummaryByTime, PricingResponse};


/// Prints a comprehensive analysis report in Markdown format.
///
/// This function generates a structured report containing both time-based and
/// distance-based clustering results. It uses tables to display metrics and
/// optionally lists detailed trip data if `verbose` is set to true.
///
/// # Arguments
///
/// * `summaries_by_time` - A slice of time-based clustering summaries.
/// * `summaries_by_distance` - A slice of distance-based clustering summaries.
/// * `verbose` - Whether to include detailed trip tables in the output.
/// * `total_processed` - Total number of trips processed.
/// * `total_skipped` - Total number of trips skipped.
/// * `unknown_points` - List of unrecognized entry/exit points.
///
/// # Example
///
/// ```rust
/// // Assuming summaries are already generated
/// toll_optimizer::md_output::print_markdown(&time_summaries, &dist_summaries, true, 10, 2, &[]);
/// ```
pub fn print_markdown(
    summaries_by_time: &[TransponderSummaryByTime],
    summaries_by_distance: &[TransponderSummaryByDistance],
    verbose: bool,
    total_processed: usize,
    total_skipped: usize,
    unknown_points: &[String],
) {
    println!("# Toll Optimizer Analysis Report\n");

    println!("## Processing Summary\n");
    println!("| Metric | Value |");
    println!("| --- | --- |");
    println!("| Trips Processed | {} |", total_processed);
    println!("| Trips Skipped | {} |", total_skipped);
    println!();

    if !unknown_points.is_empty() {
        println!("### Unrecognized Access Points\n");
        for point in unknown_points {
            println!("- {} | NOT RECOGNIZED", point);
        }
        println!();
    }

    println!("## Time-Based Clustering Analysis\n");
    for summary in summaries_by_time {
        println!("### Transponder: {}, Direction: {:?}\n", summary.transponder_plate, summary.direction);
        
        for centroid in &summary.centroids {
            println!("#### Trips near {}\n", centroid.centroid_time);
            println!("| Metric | Value |");
            println!("| --- | --- |");
            println!("| Average Entry Time | {} |", centroid.average_entry_time);
            println!("| Total Distance | {:.3} km |", centroid.total_distance);
            println!("| Total Toll Charge | ${:.2} |", centroid.total_toll_charge);
            
            if centroid.total_toll_charge_previous_timeslot < centroid.total_toll_charge - 0.005 {
                let savings = centroid.total_toll_charge - centroid.total_toll_charge_previous_timeslot;
                println!("| Toll (Prev Timeslot) | ${:.2} (Save ${:.2}) |", centroid.total_toll_charge_previous_timeslot, savings);
            }
            if centroid.total_toll_charge_next_timeslot < centroid.total_toll_charge - 0.005 {
                let savings = centroid.total_toll_charge - centroid.total_toll_charge_next_timeslot;
                println!("| Toll (Next Timeslot) | ${:.2} (Save ${:.2}) |", centroid.total_toll_charge_next_timeslot, savings);
            }
            if centroid.total_optimized_savings > 0.0 {
                println!("| Potential Savings | ${:.2} |", centroid.total_optimized_savings);
            }
            println!();

            if verbose && !centroid.trips.is_empty() {
                println!("| Date | Time | Route | Distance | Type | Cost | Optimization |");
                println!("| --- | --- | --- | --- | --- | --- | --- |");
                for trip_summary in &centroid.trips {
                    let trip = trip_summary.trip;
                    let day_type_str = match trip.day_type {
                        Some(DayType::Holiday) => "Holiday",
                        Some(DayType::Weekend) => "Weekend",
                        Some(DayType::Weekday) => "Weekday",
                        None => "Unknown",
                    };
                    
                    let mut opt_msgs = Vec::new();
                    let current_cost = trip.get_total_recorded_cost();
                    if let Some(prev) = trip_summary.total_cost_previous_timeslot {
                        if prev < current_cost - 0.005 {
                            opt_msgs.push(format!("Prev: ${:.2}", prev));
                        }
                    }
                    if let Some(next) = trip_summary.total_cost_next_timeslot {
                        if next < current_cost - 0.005 {
                            opt_msgs.push(format!("Next: ${:.2}", next));
                        }
                    }
                    
                    println!("| {} | {} | {} -> {} | {}km | {} | ${:.2} | {} |",
                        trip.date_of_trip, trip.entry_time, trip.entry_point, trip.exit_point,
                        trip.distance_km, day_type_str, current_cost, opt_msgs.join(", ")
                    );
                }
                println!();
            }
        }
    }

    println!("## Distance-Based (Zones) Clustering Analysis\n");
    for summary in summaries_by_distance {
        println!("### Transponder: {}, Direction: {:?}\n", summary.transponder_plate, summary.direction);
        
        for centroid in &summary.centroids {
            let entry = centroid.representative_entry.as_deref().unwrap_or("Unknown");
            let exit = centroid.representative_exit.as_deref().unwrap_or("Unknown");
            println!("#### {} -> {} (Avg: {:.2} km)\n", entry, exit, centroid.average_distance);
            
            println!("| Metric | Value |");
            println!("| --- | --- |");
            println!("| Total Toll Charge | ${:.2} |", centroid.total_toll_charge);
            if centroid.total_optimized_savings > 0.0 {
                println!("| Potential Savings | ${:.2} |", centroid.total_optimized_savings);
            }
            println!();

            if verbose && !centroid.trips.is_empty() {
                println!("| Date | Time | Route | Distance | Type | Cost | Note |");
                println!("| --- | --- | --- | --- | --- | --- | --- |");
                for trip_summary in &centroid.trips {
                    let trip = trip_summary.trip;
                    let day_type_str = match trip.day_type {
                        Some(DayType::Holiday) => "Holiday",
                        Some(DayType::Weekend) => "Weekend",
                        Some(DayType::Weekday) => "Weekday",
                        None => "Unknown",
                    };
                    let note = trip_summary.optimization_note.as_deref().unwrap_or("");
                    println!("| {} | {} | {} -> {} | {}km | {} | ${:.2} | {} |",
                        trip.date_of_trip, trip.entry_time, trip.entry_point, trip.exit_point,
                        trip.distance_km, day_type_str, trip.get_total_recorded_cost(), note
                    );
                }
                println!();
            }
        }
    }
}

/// Prints a live pricing report in Markdown format.
pub fn print_pricing_markdown(pricing: &PricingResponse, date: &str, time: &str) {
    println!("# Toll Optimizer Live Pricing Report\n");
    println!("**Date:** {}  ", date);
    println!("**Time:** {}  ", time);
    println!("**Day Type:** {}\n", pricing.day_type);

    println!("## Timeslot Comparison\n");
    println!("| Timeslot | Average EB | Average WB |");
    println!("| --- | --- | --- |");
    println!("| **Current:** {} | {:.2}¢/km | {:.2}¢/km |", pricing.current.timeslot, pricing.current.average_eb, pricing.current.average_wb);
    println!("| **Next:** {} | {:.2}¢/km | {:.2}¢/km |", pricing.next.timeslot, pricing.next.average_eb, pricing.next.average_wb);
    println!();

    let current_avg = (pricing.current.average_eb + pricing.current.average_wb) / 2.0;
    let next_avg = (pricing.next.average_eb + pricing.next.average_wb) / 2.0;

    println!("## Optimization Strategy\n");
    if next_avg < current_avg - 0.001 {
        println!("> **Tip:** Waiting for the next timeslot ({}) could save you money!  ", pricing.next.timeslot);
        println!("> Average rates are expected to drop by approximately {:.2}¢/km.", current_avg - next_avg);
    } else if next_avg > current_avg + 0.001 {
        println!("> **Tip:** Leave now ({}) to avoid higher rates in the next timeslot!  ", pricing.current.timeslot);
        println!("> Average rates are expected to increase by approximately {:.2}¢/km.", next_avg - current_avg);
    } else {
        println!("> Rates are expected to remain stable in the next timeslot.");
    }
    println!();
}
