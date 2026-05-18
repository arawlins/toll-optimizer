use crate::trip_analyzer::{
    DayType, Direction, PricingResponse, TransponderSummaryByDistance, TransponderSummaryByTime,
};
use std::collections::HashMap;

/// Input data for a complete Markdown analysis report.
pub struct AnalysisMarkdownReport<'report, 'trips> {
    /// Time-based clustering summaries to render.
    pub summaries_by_time: &'report [TransponderSummaryByTime<'trips>],
    /// Distance-based clustering summaries to render.
    pub summaries_by_distance: &'report [TransponderSummaryByDistance<'trips>],
    /// Number of statement rows accepted as trips.
    pub total_processed: usize,
    /// Number of statement rows skipped.
    pub total_skipped: usize,
    /// Total recorded bill cost in dollars.
    pub total_cost: f64,
    /// Total potential savings from time-based optimization.
    pub total_time_savings: f64,
    /// Total potential savings from distance-based optimization.
    pub total_distance_savings: f64,
    /// Unknown access points encountered while parsing.
    pub unknown_points: &'report [String],
    /// Unknown vehicle classes encountered while parsing.
    pub unknown_vehicle_classes: &'report [String],
    /// Camera charges grouped by transponder or plate.
    pub camera_charges: &'report HashMap<String, f64>,
    /// Whether to show individual trip details.
    pub show_summary: bool,
}

/// Prints a comprehensive analysis report in Markdown format.
///
/// This function generates a structured report containing both time-based and
/// distance-based clustering results. It uses tables to display metrics and
/// lists detailed trip data for each cluster.
///
/// # Arguments
///
/// * `report` - Complete analysis report data to render.
///
/// # Example
///
/// ```rust,no_run
/// // Assuming summaries are already generated
/// # let time_summaries = vec![];
/// # let dist_summaries = vec![];
/// # let camera_charges = std::collections::HashMap::new();
/// let report = toll_optimizer::AnalysisMarkdownReport {
///     summaries_by_time: &time_summaries,
///     summaries_by_distance: &dist_summaries,
///     total_processed: 10,
///     total_skipped: 2,
///     total_cost: 100.0,
///     total_time_savings: 5.0,
///     total_distance_savings: 2.0,
///     unknown_points: &[],
///     unknown_vehicle_classes: &[],
///     camera_charges: &camera_charges,
///     show_summary: false,
/// };
/// toll_optimizer::print_markdown(report);
/// ```
pub fn print_markdown(report: AnalysisMarkdownReport<'_, '_>) {
    let AnalysisMarkdownReport {
        summaries_by_time,
        summaries_by_distance,
        total_processed,
        total_skipped,
        total_cost,
        total_time_savings,
        total_distance_savings,
        unknown_points,
        unknown_vehicle_classes,
        camera_charges,
        show_summary,
    } = report;

    println!("# Toll Optimizer Analysis Report\n");

    println!("## Processing Summary\n");
    println!("| Metric | Value |");
    println!("| --- | --- |");
    println!("| Trips Processed | {} |", total_processed);
    println!("| Trips Skipped | {} |", total_skipped);
    println!("| Total Bill Cost | ${:.2} |", total_cost);
    println!(
        "| Potential Time-Based Savings | ${:.2} |",
        total_time_savings
    );
    println!(
        "| Potential Distance-Based Savings | ${:.2} |",
        total_distance_savings
    );
    println!();

    if !unknown_points.is_empty() {
        println!("### Unrecognized Access Points\n");
        for point in unknown_points {
            println!("- {} | NOT RECOGNIZED", point);
        }
        println!();
    }

    if !unknown_vehicle_classes.is_empty() {
        println!("### Unrecognized Vehicle Classes\n");
        for class in unknown_vehicle_classes {
            println!("- {} | NOT RECOGNIZED", class);
        }
        println!();
    }

    if !camera_charges.is_empty() {
        println!("### Camera Charges\n");
        println!("| Transponder/Plate | Charge |");
        println!("| --- | --- |");
        let mut plates: Vec<_> = camera_charges.keys().collect();
        plates.sort();
        let mut show_recommendation = false;
        for plate in plates {
            let charge = camera_charges[plate];
            println!("| {} | ${:.2} |", plate, charge);
            if charge > 31.50 {
                show_recommendation = true;
            }
        }
        println!();
        if show_recommendation {
            println!(
                "> **Tip:** Leasing a transponder for $31.50 (plus applicable taxes) per year will save you money on the camera charges.\n"
            );
        }
    }

    println!("## Time-Based Analysis\n");
    for summary in summaries_by_time {
        println!(
            "### Transponder: {}, Direction: {:?}\n",
            summary.transponder_plate, summary.direction
        );

        for centroid in &summary.centroids {
            println!("#### Trips near {}\n", centroid.centroid_time);
            println!("| Metric | Value |");
            println!("| --- | --- |");
            println!("| Average Entry Time | {} |", centroid.average_entry_time);
            println!("| Total Distance | {:.3} km |", centroid.total_distance);
            println!("| Total Toll Charge | ${:.2} |", centroid.total_toll_charge);

            if centroid.total_toll_charge_previous_timeslot < centroid.total_toll_charge - 0.005 {
                let savings =
                    centroid.total_toll_charge - centroid.total_toll_charge_previous_timeslot;
                println!(
                    "| Toll (Prev Timeslot) | ${:.2} (Save ${:.2}) |",
                    centroid.total_toll_charge_previous_timeslot, savings
                );
            }
            if centroid.total_toll_charge_next_timeslot < centroid.total_toll_charge - 0.005 {
                let savings = centroid.total_toll_charge - centroid.total_toll_charge_next_timeslot;
                println!(
                    "| Toll (Next Timeslot) | ${:.2} (Save ${:.2}) |",
                    centroid.total_toll_charge_next_timeslot, savings
                );
            }
            if centroid.total_optimized_savings > 0.0 {
                println!(
                    "| Potential Savings | ${:.2} |",
                    centroid.total_optimized_savings
                );
            }
            println!();

            if !show_summary && !centroid.trips.is_empty() {
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
                    if let Some(prev) = trip_summary.total_cost_previous_timeslot
                        && prev < current_cost - 0.005
                    {
                        opt_msgs.push(format!("Prev: ${:.2}", prev));
                    }
                    if let Some(next) = trip_summary.total_cost_next_timeslot
                        && next < current_cost - 0.005
                    {
                        opt_msgs.push(format!("Next: ${:.2}", next));
                    }

                    println!(
                        "| {} | {} | {} -> {} | {}km | {} | ${:.2} | {} |",
                        trip.date_of_trip,
                        trip.entry_time,
                        trip.entry_point,
                        trip.exit_point,
                        trip.distance_km,
                        day_type_str,
                        current_cost,
                        opt_msgs.join(", ")
                    );
                }
                println!();
            }
        }
    }

    println!("## Distance-Based Analysis\n");
    for summary in summaries_by_distance {
        println!(
            "### Transponder: {}, Direction: {:?}\n",
            summary.transponder_plate, summary.direction
        );

        for centroid in &summary.centroids {
            let entry = centroid
                .representative_entry
                .as_deref()
                .unwrap_or("Unknown");
            let exit = centroid.representative_exit.as_deref().unwrap_or("Unknown");
            println!(
                "#### {} -> {} (Avg: {:.2} km)\n",
                entry, exit, centroid.average_distance
            );

            println!("| Metric | Value |");
            println!("| --- | --- |");
            println!("| Total Toll Charge | ${:.2} |", centroid.total_toll_charge);
            if centroid.total_optimized_savings > 0.0 {
                println!(
                    "| Potential Savings | ${:.2} |",
                    centroid.total_optimized_savings
                );
            }
            println!();

            if !show_summary && !centroid.trips.is_empty() {
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
                    println!(
                        "| {} | {} | {} -> {} | {}km | {} | ${:.2} | {} |",
                        trip.date_of_trip,
                        trip.entry_time,
                        trip.entry_point,
                        trip.exit_point,
                        trip.distance_km,
                        day_type_str,
                        trip.get_total_recorded_cost(),
                        note
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
    println!(
        "| **Current:** {} | {:.2}¢/km | {:.2}¢/km |",
        pricing.current.timeslot, pricing.current.average_eb, pricing.current.average_wb
    );
    println!(
        "| **Next:** {} | {:.2}¢/km | {:.2}¢/km |",
        pricing.next.timeslot, pricing.next.average_eb, pricing.next.average_wb
    );
    println!();

    let current_avg = (pricing.current.average_eb + pricing.current.average_wb) / 2.0;
    let next_avg = (pricing.next.average_eb + pricing.next.average_wb) / 2.0;

    println!("## Optimization Strategy\n");
    if next_avg < current_avg - 0.001 {
        println!(
            "> **Tip:** Waiting for the next timeslot ({}) could save you money!  ",
            pricing.next.timeslot
        );
        println!(
            "> Average rates are expected to drop by approximately {:.2}¢/km.",
            current_avg - next_avg
        );
    } else if next_avg > current_avg + 0.001 {
        println!(
            "> **Tip:** Leave now ({}) to avoid higher rates in the next timeslot!  ",
            pricing.current.timeslot
        );
        println!(
            "> Average rates are expected to increase by approximately {:.2}¢/km.",
            next_avg - current_avg
        );
    } else {
        println!("> Rates are expected to remain stable in the next timeslot.");
    }
    println!();
}

/// Input data for a single-trip Markdown cost report.
pub struct SingleTripMarkdownReport<'a> {
    /// Entry access point.
    pub entry: &'a str,
    /// Exit access point.
    pub exit: &'a str,
    /// Trip date displayed in the report.
    pub date: &'a str,
    /// Trip time displayed in the report.
    pub time: &'a str,
    /// Vehicle class label.
    pub class: &'a str,
    /// Calculated route distance in kilometers.
    pub distance_km: f64,
    /// Calculated travel direction.
    pub direction: &'a Direction,
    /// Calculated pricing day type.
    pub day_type: &'a DayType,
    /// Base route toll in dollars, excluding fixed trip charge.
    pub cost: f64,
}

/// Prints a single trip cost report in Markdown format.
pub fn print_single_trip_markdown(report: SingleTripMarkdownReport<'_>) {
    println!("# Toll Optimizer Single Trip Report\n");
    println!("| Metric | Value |");
    println!("| --- | --- |");
    println!("| **Route** | {} -> {} |", report.entry, report.exit);
    println!("| **Date** | {} |", report.date);
    println!("| **Time** | {} |", report.time);
    println!("| **Vehicle Class** | {} |", report.class);
    println!("| **Distance** | {:.3} km |", report.distance_km);
    println!("| **Direction** | {:?} |", report.direction);
    println!("| **Day Type** | {:?} |", report.day_type);
    println!("| **Base Toll** | ${:.2} |", report.cost);
    println!("| **Trip Charge** | $1.00 |");
    println!(
        "| **Total Estimated Cost** | **${:.2}** |",
        report.cost + 1.00
    );
    println!();
}
