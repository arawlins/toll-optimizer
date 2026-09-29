//! Performance benchmarks for toll-optimizer core operations.
//!
//! Benchmarks:
//! 1. Single trip cost calculation (pricing lookups and rate table traversals)
//! 2. CSV parsing and record extraction
//! 3. Time-based trip clustering and savings optimization
//! 4. Distance-based trip clustering and savings optimization

use std::hint::black_box;
use std::time::Instant;
use toll_optimizer::{
    VehicleClass, analyze_trips_by_distance, analyze_trips_by_time, calculate_single_trip_cost,
    parse_trips,
};

const SAMPLE_CSV: &str = include_str!("../tests/csv/2025-08-28 - light vehicles.csv");

fn bench_trip_cost_calculation(iterations: usize) {
    let routes = [
        (
            "QEW",
            "Highway 401",
            "2025-01-15",
            "08:15 AM",
            VehicleClass::LightVehicle,
        ),
        (
            "Brock Rd",
            "Highway 400",
            "2025-07-20",
            "05:30 PM",
            VehicleClass::LightVehicle,
        ),
        (
            "McCowan",
            "Hwy404",
            "2026-02-10",
            "12:00 PM",
            VehicleClass::HeavySingleUnit,
        ),
        (
            "Hwy404",
            "Kennedy",
            "2026-04-12",
            "11:45 PM",
            VehicleClass::Motorcycle,
        ),
    ];

    let start = Instant::now();
    for _ in 0..iterations {
        for &(entry, exit, date, time, vehicle_class) in &routes {
            let res = calculate_single_trip_cost(entry, exit, date, time, vehicle_class);
            let _ = black_box(res);
        }
    }
    let elapsed = start.elapsed();
    let total_ops = iterations * routes.len();
    println!(
        "calculate_single_trip_cost: {:?} total for {} ops ({:.2} ns/op)",
        elapsed,
        total_ops,
        elapsed.as_nanos() as f64 / total_ops as f64
    );
}

fn bench_csv_parsing(iterations: usize) {
    let start = Instant::now();
    for _ in 0..iterations {
        let res = parse_trips(SAMPLE_CSV.as_bytes());
        black_box(res);
    }
    let elapsed = start.elapsed();
    println!(
        "parse_trips: {:?} total for {} iterations ({:.2} µs/iter)",
        elapsed,
        iterations,
        elapsed.as_micros() as f64 / iterations as f64
    );
}

fn bench_clustering_analysis(iterations: usize) {
    let parse_result = parse_trips(SAMPLE_CSV.as_bytes());
    let trips = parse_result.trips;

    let start_time = Instant::now();
    for _ in 0..iterations {
        let res = analyze_trips_by_time(&trips);
        black_box(res);
    }
    let elapsed_time = start_time.elapsed();
    println!(
        "analyze_trips_by_time: {:?} total for {} iterations ({:.2} µs/iter)",
        elapsed_time,
        iterations,
        elapsed_time.as_micros() as f64 / iterations as f64
    );

    let start_dist = Instant::now();
    for _ in 0..iterations {
        let res = analyze_trips_by_distance(&trips);
        black_box(res);
    }
    let elapsed_dist = start_dist.elapsed();
    println!(
        "analyze_trips_by_distance: {:?} total for {} iterations ({:.2} µs/iter)",
        elapsed_dist,
        iterations,
        elapsed_dist.as_micros() as f64 / iterations as f64
    );
}

fn main() {
    println!("=== Toll Optimizer Performance Benchmarks ===\n");
    bench_trip_cost_calculation(25_000);
    bench_csv_parsing(1_000);
    bench_clustering_analysis(500);
    println!("\nBenchmarks completed successfully.");
}
