//! Toll calculation, date classification, and trip clustering logic.
//!
//! This module contains the core domain logic used by both the CLI and tests.
//! Costs are returned in dollars unless explicitly documented as cents/km.

use crate::constants::*;
use crate::vehicle_class::{
    heavy_multiple_unit, heavy_single_unit, light_vehicles, medium_vehicles, motorcycles,
};
use simple_datetime_rs::Date;
use std::collections::HashMap;

use anyhow::{Result, anyhow};
use chrono::{Datelike, Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Direction of travel along the ordered 407 ETR access-point list.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Direction {
    /// Travel from lower-indexed access points toward higher-indexed access points.
    Eastbound,
    /// Travel from higher-indexed access points toward lower-indexed access points.
    Westbound,
}

/// Pricing day bucket used by 407 ETR rate tables.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum DayType {
    /// Non-holiday Monday through Friday.
    Weekday,
    /// Saturday or Sunday.
    Weekend,
    /// A configured statutory holiday; priced with weekend tables.
    Holiday,
}

/// Supported 407 ETR vehicle classes.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum VehicleClass {
    /// Passenger cars and other light vehicles.
    LightVehicle,
    /// Heavy single-unit vehicles.
    HeavySingleUnit,
    /// Heavy multiple-unit vehicles.
    HeavyMultipleUnit,
    /// Medium vehicles.
    MediumVehicle,
    /// Motorcycles.
    Motorcycle,
}

type RateTable = &'static [[f64; 12]];

struct VehicleRateTables {
    weekday_eb_2026: RateTable,
    weekday_wb_2026: RateTable,
    weekend_eb_2026: RateTable,
    weekend_wb_2026: RateTable,
    weekday_eb_2025: RateTable,
    weekday_wb_2025: RateTable,
    weekend_eb_2025: RateTable,
    weekend_wb_2025: RateTable,
}

impl VehicleRateTables {
    fn select(&self, day_type: &DayType, direction: &Direction, year: u32) -> RateTable {
        let is_2026 = year >= 2026;
        let is_weekday = matches!(day_type, DayType::Weekday);
        let is_eb = matches!(direction, Direction::Eastbound);

        match (is_2026, is_weekday, is_eb) {
            (true, true, true) => self.weekday_eb_2026,
            (true, true, false) => self.weekday_wb_2026,
            (true, false, true) => self.weekend_eb_2026,
            (true, false, false) => self.weekend_wb_2026,
            (false, true, true) => self.weekday_eb_2025,
            (false, true, false) => self.weekday_wb_2025,
            (false, false, true) => self.weekend_eb_2025,
            (false, false, false) => self.weekend_wb_2025,
        }
    }
}

const LIGHT_VEHICLE_RATE_TABLES: VehicleRateTables = VehicleRateTables {
    weekday_eb_2026: &light_vehicles::WEEKDAY_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekday_wb_2026: &light_vehicles::WEEKDAY_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekend_eb_2026: &light_vehicles::WEEKEND_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekend_wb_2026: &light_vehicles::WEEKEND_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekday_eb_2025: &light_vehicles::WEEKDAY_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
    weekday_wb_2025: &light_vehicles::WEEKDAY_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
    weekend_eb_2025: &light_vehicles::WEEKEND_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
    weekend_wb_2025: &light_vehicles::WEEKEND_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
};

const HEAVY_SINGLE_UNIT_RATE_TABLES: VehicleRateTables = VehicleRateTables {
    weekday_eb_2026: &heavy_single_unit::WEEKDAY_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekday_wb_2026: &heavy_single_unit::WEEKDAY_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekend_eb_2026: &heavy_single_unit::WEEKEND_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekend_wb_2026: &heavy_single_unit::WEEKEND_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekday_eb_2025: &heavy_single_unit::WEEKDAY_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
    weekday_wb_2025: &heavy_single_unit::WEEKDAY_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
    weekend_eb_2025: &heavy_single_unit::WEEKEND_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
    weekend_wb_2025: &heavy_single_unit::WEEKEND_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
};

const HEAVY_MULTIPLE_UNIT_RATE_TABLES: VehicleRateTables = VehicleRateTables {
    weekday_eb_2026: &heavy_multiple_unit::WEEKDAY_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekday_wb_2026: &heavy_multiple_unit::WEEKDAY_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekend_eb_2026: &heavy_multiple_unit::WEEKEND_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekend_wb_2026: &heavy_multiple_unit::WEEKEND_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekday_eb_2025: &heavy_multiple_unit::WEEKDAY_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
    weekday_wb_2025: &heavy_multiple_unit::WEEKDAY_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
    weekend_eb_2025: &heavy_multiple_unit::WEEKEND_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
    weekend_wb_2025: &heavy_multiple_unit::WEEKEND_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
};

const MEDIUM_VEHICLE_RATE_TABLES: VehicleRateTables = VehicleRateTables {
    weekday_eb_2026: &medium_vehicles::WEEKDAY_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekday_wb_2026: &medium_vehicles::WEEKDAY_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekend_eb_2026: &medium_vehicles::WEEKEND_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekend_wb_2026: &medium_vehicles::WEEKEND_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekday_eb_2025: &medium_vehicles::WEEKDAY_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
    weekday_wb_2025: &medium_vehicles::WEEKDAY_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
    weekend_eb_2025: &medium_vehicles::WEEKEND_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
    weekend_wb_2025: &medium_vehicles::WEEKEND_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
};

const MOTORCYCLE_RATE_TABLES: VehicleRateTables = VehicleRateTables {
    weekday_eb_2026: &motorcycles::WEEKDAY_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekday_wb_2026: &motorcycles::WEEKDAY_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekend_eb_2026: &motorcycles::WEEKEND_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekend_wb_2026: &motorcycles::WEEKEND_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026,
    weekday_eb_2025: &motorcycles::WEEKDAY_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
    weekday_wb_2025: &motorcycles::WEEKDAY_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
    weekend_eb_2025: &motorcycles::WEEKEND_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
    weekend_wb_2025: &motorcycles::WEEKEND_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025,
};

impl VehicleClass {
    /// Returns the canonical display label for this vehicle class.
    pub fn to_str(&self) -> &'static str {
        match self {
            VehicleClass::LightVehicle => "Light vehicle",
            VehicleClass::HeavySingleUnit => "Heavy Single Unit",
            VehicleClass::HeavyMultipleUnit => "Heavy Multiple Unit",
            VehicleClass::MediumVehicle => "Medium Vehicle",
            VehicleClass::Motorcycle => "Motorcycle",
        }
    }

    fn rate_tables(&self) -> &'static VehicleRateTables {
        match self {
            VehicleClass::LightVehicle => &LIGHT_VEHICLE_RATE_TABLES,
            VehicleClass::HeavySingleUnit => &HEAVY_SINGLE_UNIT_RATE_TABLES,
            VehicleClass::HeavyMultipleUnit => &HEAVY_MULTIPLE_UNIT_RATE_TABLES,
            VehicleClass::MediumVehicle => &MEDIUM_VEHICLE_RATE_TABLES,
            VehicleClass::Motorcycle => &MOTORCYCLE_RATE_TABLES,
        }
    }

    /// Returns the cents-per-kilometer rate for a specific pricing cell.
    ///
    /// `timeslot_idx` indexes the active timeslot table for the chosen year and
    /// day type. `zone_idx` is zero-based, so zone 1 is passed as `0`.
    pub fn get_rate(
        &self,
        day_type: &DayType,
        direction: &Direction,
        year: u32,
        timeslot_idx: usize,
        zone_idx: usize,
    ) -> f64 {
        self.rate_tables().select(day_type, direction, year)[timeslot_idx][zone_idx]
    }

    /// Calculates the average cents-per-kilometer rate across all 12 zones.
    pub fn get_average_rate(
        &self,
        day_type: &DayType,
        direction: &Direction,
        is_2026: bool,
        timeslot_idx: usize,
    ) -> f64 {
        // All vehicle class modules have 12 zones, so we can calculate the average if not provided
        let mut total = 0.0;
        for zone_idx in 0..12 {
            total += self.get_rate(
                day_type,
                direction,
                if is_2026 { 2026 } else { 2025 },
                timeslot_idx,
                zone_idx,
            );
        }
        (total / 12.0 * 100.0).round() / 100.0
    }
}

impl FromStr for VehicleClass {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "light vehicle" => Ok(VehicleClass::LightVehicle),
            "heavy single unit" => Ok(VehicleClass::HeavySingleUnit),
            "heavy multiple unit" => Ok(VehicleClass::HeavyMultipleUnit),
            "medium vehicle" => Ok(VehicleClass::MediumVehicle),
            "motorcycle" => Ok(VehicleClass::Motorcycle),
            _ => Err(format!("invalid vehicle class: {s}")),
        }
    }
}

impl fmt::Display for VehicleClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

#[derive(Debug, Clone, Copy)]
struct RouteCost {
    base_toll: f64,
    distance_km: f64,
}

fn get_access_point_index(name: &str) -> Option<usize> {
    ACCESS_POINTS
        .iter()
        .position(|&ap| ap.eq_ignore_ascii_case(name))
}

fn calculate_route_cost(
    start_idx: usize,
    end_idx: usize,
    direction: &Direction,
    day_type: &DayType,
    vehicle_class: VehicleClass,
    year: u32,
    timeslot_idx: usize,
) -> Option<RouteCost> {
    let mut total_cost_cents = 0.0;
    let mut total_distance = 0.0;

    match direction {
        Direction::Eastbound => {
            if start_idx >= end_idx {
                return None;
            }

            for (segment_idx, &distance) in ACCESS_POINT_DISTANCES
                .iter()
                .enumerate()
                .take(end_idx)
                .skip(start_idx)
            {
                let distance = distance as f64;
                total_distance += distance;

                let access_point = ACCESS_POINTS[segment_idx];
                let zone = EB_ZONES.iter().find(|&&(name, _)| name == access_point)?.1 as usize;
                let rate =
                    vehicle_class.get_rate(day_type, direction, year, timeslot_idx, zone - 1);
                total_cost_cents += distance * rate;
            }
        }
        Direction::Westbound => {
            if start_idx <= end_idx {
                return None;
            }

            for (segment_idx, &distance) in ACCESS_POINT_DISTANCES
                .iter()
                .enumerate()
                .take(start_idx)
                .skip(end_idx)
            {
                let distance = distance as f64;
                total_distance += distance;

                let access_point = ACCESS_POINTS.get(segment_idx + 1)?;
                let zone = WB_ZONES.iter().find(|&&(name, _)| name == *access_point)?.1 as usize;
                let rate =
                    vehicle_class.get_rate(day_type, direction, year, timeslot_idx, zone - 1);
                total_cost_cents += distance * rate;
            }
        }
    }

    Some(RouteCost {
        base_toll: total_cost_cents / 100.0,
        distance_km: total_distance,
    })
}

/// Average pricing for one timeslot.
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeslotPrices {
    /// Human-readable timeslot start label, such as `7:00 AM`.
    pub timeslot: String,
    /// Average westbound rate in cents per kilometer.
    pub average_wb: f64,
    /// Average eastbound rate in cents per kilometer.
    pub average_eb: f64,
}

/// Current and next timeslot pricing response for live/planned pricing.
#[derive(Debug, Serialize, Deserialize)]
pub struct PricingResponse {
    /// Pricing for the timeslot containing the requested time.
    pub current: TimeslotPrices,
    /// Pricing for the next configured timeslot.
    pub next: TimeslotPrices,
    /// Day type label including the selected vehicle class.
    pub day_type: String,
}

/// Parsed 407 ETR trip row.
///
/// Monetary and distance fields are stored as statement strings to preserve the
/// source CSV values. Use calculation helpers for normalized numeric estimates.
#[derive(Debug, Clone, Serialize)]
pub struct TripRecord {
    /// Transponder or plate number from the statement.
    pub transponder_plate: String,
    /// Parsed 407 ETR vehicle class.
    pub vehicle_class: VehicleClass,

    /// Statement trip date, usually in `DD Mon YY` format.
    pub date_of_trip: String,
    /// Statement entry time, usually in `H:MM AM/PM` format.
    pub entry_time: String,
    /// Canonical entry access point.
    pub entry_point: String,
    /// Canonical exit access point.
    pub exit_point: String,
    /// Statement distance in kilometers.
    pub distance_km: String,
    /// Statement variable toll charge in dollars.
    pub toll_charge: String,
    /// Statement fixed trip toll charge in dollars.
    pub trip_toll_charge: String,
    /// Statement camera charge in dollars.
    pub camera_charge: String,
    /// Computed direction once the trip has been validated against access points.
    pub direction: Option<Direction>,
    /// Computed day type for the statement date.
    pub day_type: Option<DayType>,
}

impl TripRecord {
    /// Builds a trip record from a raw CSV row.
    ///
    /// Returns `None` when the row is too short or the vehicle class is not
    /// recognized. Direction is filled in later by the CSV parser after access
    /// point normalization.
    pub fn from_csv_record(record: &csv::StringRecord) -> Option<Self> {
        if record.len() < 10 {
            return None;
        }

        let first = record[0].trim_start_matches('"');
        let last = record[record.len() - 1].trim_end_matches('"');

        let entry_point = record[4].to_string();
        let exit_point = record[5].to_string();
        let date_of_trip = record[2].to_string();

        let day_type = classify_day(&date_of_trip);

        let vehicle_class = record[1].parse::<VehicleClass>().ok()?;

        Some(TripRecord {
            transponder_plate: first.to_string(),
            vehicle_class,

            date_of_trip,
            entry_time: record[3].to_string(),
            entry_point,
            exit_point,
            distance_km: record[6].to_string(),
            toll_charge: record[7].to_string(),
            trip_toll_charge: record[8].to_string(),
            camera_charge: last.to_string(),
            direction: None,
            day_type,
        })
    }
    /// Returns the number of pricing timeslots for this trip's year and day type.
    pub fn get_timeslot_count(&self) -> Option<usize> {
        let (_, _, year) = parse_date(&self.date_of_trip)?;
        let count = match (year, self.day_type.as_ref()?) {
            (y, DayType::Weekday) if y <= 2025 => WEEKDAY_TIMESLOTS_2025.len(),
            (_, DayType::Weekday) => WEEKDAY_TIMESLOTS_2026.len(),
            (y, DayType::Weekend) | (y, DayType::Holiday) if y <= 2025 => {
                WEEKEND_TIMESLOTS_2025.len()
            }
            (_, DayType::Weekend) | (_, DayType::Holiday) => WEEKEND_TIMESLOTS_2026.len(),
        };
        Some(count)
    }

    /// Returns the pricing timeslot index for a specific `H:MM AM/PM` time.
    pub fn get_timeslot_index_for_time(&self, time_str: &str) -> Option<usize> {
        let entry_minutes = parse_time_to_minutes(time_str)?;
        let (_, _, year) = parse_date(&self.date_of_trip)?;

        let slots = match (year, self.day_type.as_ref()?) {
            (y, DayType::Weekday) if y <= 2025 => &WEEKDAY_TIMESLOTS_2025[..],
            (_, DayType::Weekday) => &WEEKDAY_TIMESLOTS_2026[..],
            (y, DayType::Weekend) | (y, DayType::Holiday) if y <= 2025 => {
                &WEEKEND_TIMESLOTS_2025[..]
            }
            (_, DayType::Weekend) | (_, DayType::Holiday) => &WEEKEND_TIMESLOTS_2026[..],
        };

        let slot_minutes: Vec<u32> = slots
            .iter()
            .filter_map(|&t| parse_time_to_minutes(t))
            .collect();

        if slot_minutes.is_empty() {
            return None;
        }

        // Find the index i such that slots[i] <= entry_minutes
        // If entry_minutes is before the first slot, it belongs to the last slot (wrap-around)
        let mut index = slot_minutes.len() - 1;
        for (i, &slot_time) in slot_minutes.iter().enumerate() {
            if entry_minutes < slot_time {
                if i == 0 {
                    return Some(slot_minutes.len() - 1);
                }
                return Some(i - 1);
            }
            index = i;
        }
        Some(index)
    }

    /// Returns the pricing timeslot index for this trip's entry time.
    pub fn get_timeslot_index(&self) -> Option<usize> {
        self.get_timeslot_index_for_time(&self.entry_time)
    }

    /// Calculates base route toll and distance for explicit access-point indices.
    ///
    /// The returned tuple is `(base_toll_dollars, distance_km)`. Fixed trip and
    /// camera charges are not included.
    pub fn calculate_cost_from_indices(
        &self,
        start_idx: usize,
        end_idx: usize,
        timeslot_idx: usize,
    ) -> Option<(f64, f64)> {
        let direction = self.direction.as_ref()?;
        let day_type = self.day_type.as_ref()?;
        let (_, _, year) = parse_date(&self.date_of_trip)?;

        let route_cost = calculate_route_cost(
            start_idx,
            end_idx,
            direction,
            day_type,
            self.vehicle_class,
            year,
            timeslot_idx,
        )?;
        Some((route_cost.base_toll, route_cost.distance_km))
    }

    /// Calculates base route toll and distance at a specific timeslot index.
    pub fn calculate_cost_at_timeslot(&self, timeslot_idx: usize) -> Option<(f64, f64)> {
        // Use ACCESS_POINTS as the canonical list for indices
        let start_idx = get_access_point_index(&self.entry_point)?;
        let end_idx = get_access_point_index(&self.exit_point)?;

        self.calculate_cost_from_indices(start_idx, end_idx, timeslot_idx)
    }

    /// Calculates base route toll and distance for this trip's entry timeslot.
    pub fn calculate_cost(&self) -> Option<(f64, f64)> {
        let timeslot_idx = self.get_timeslot_index()?;
        self.calculate_cost_at_timeslot(timeslot_idx)
    }

    /// Returns the statement total of toll, trip toll, and camera charges.
    ///
    /// Malformed numeric fields are treated as `0.0`, matching existing parser
    /// behavior for imperfect CSV exports.
    pub fn get_total_recorded_cost(&self) -> f64 {
        let toll = self.toll_charge.trim().parse::<f64>().unwrap_or(0.0);
        let trip_toll = self.trip_toll_charge.trim().parse::<f64>().unwrap_or(0.0);
        let camera = self.camera_charge.trim().parse::<f64>().unwrap_or(0.0);
        toll + trip_toll + camera
    }

    /// Returns the west-to-east index for an access-point name.
    pub fn get_access_point_index(&self, name: &str) -> Option<usize> {
        get_access_point_index(name)
    }
    /// Returns the 2026 timeslot index for this trip's entry time.
    pub fn get_timeslot_index_2026(&self) -> Option<usize> {
        self.get_timeslot_index_for_time_2026(&self.entry_time)
    }

    /// Returns the 2026 timeslot index for a specific `H:MM AM/PM` time.
    pub fn get_timeslot_index_for_time_2026(&self, time_str: &str) -> Option<usize> {
        let entry_minutes = parse_time_to_minutes(time_str)?;
        // Always use 2026 constants
        let slots = match self.day_type.as_ref()? {
            DayType::Weekday => &WEEKDAY_TIMESLOTS_2026[..],
            DayType::Weekend | DayType::Holiday => &WEEKEND_TIMESLOTS_2026[..],
        };

        let slot_minutes: Vec<u32> = slots
            .iter()
            .filter_map(|&t| parse_time_to_minutes(t))
            .collect();

        if slot_minutes.is_empty() {
            return None;
        }

        let mut index = slot_minutes.len() - 1;
        for (i, &slot_time) in slot_minutes.iter().enumerate() {
            if entry_minutes < slot_time {
                if i == 0 {
                    return Some(slot_minutes.len() - 1);
                }
                return Some(i - 1);
            }
            index = i;
        }
        Some(index)
    }

    /// Calculates 2026 total estimated cost and distance for this trip.
    ///
    /// The returned cost includes the recalculated 2026 base toll plus the
    /// statement's fixed trip toll and camera charge.
    pub fn calculate_cost_2026(&self) -> Option<(f64, f64)> {
        let timeslot_idx = self.get_timeslot_index_2026()?;
        let direction = self.direction.as_ref()?;
        let day_type = self.day_type.as_ref()?;

        let start_idx = self.get_access_point_index(&self.entry_point)?;
        let end_idx = self.get_access_point_index(&self.exit_point)?;
        let route_cost = calculate_route_cost(
            start_idx,
            end_idx,
            direction,
            day_type,
            self.vehicle_class,
            2026,
            timeslot_idx,
        )?;

        let trip_toll = self.trip_toll_charge.trim().parse::<f64>().unwrap_or(0.0);
        let camera = self.camera_charge.trim().parse::<f64>().unwrap_or(0.0);

        Some((
            route_cost.base_toll + trip_toll + camera,
            route_cost.distance_km,
        ))
    }
    /// Returns the timeslot labels that apply to this trip.
    pub fn get_timeslots(&self) -> Option<&'static [&'static str]> {
        let (_, _, year) = parse_date(&self.date_of_trip)?;
        match (year, self.day_type.as_ref()?) {
            (y, DayType::Weekday) if y <= 2025 => Some(&WEEKDAY_TIMESLOTS_2025[..]),
            (_, DayType::Weekday) => Some(&WEEKDAY_TIMESLOTS_2026[..]),
            (y, DayType::Weekend) | (y, DayType::Holiday) if y <= 2025 => {
                Some(&WEEKEND_TIMESLOTS_2025[..])
            }
            (_, DayType::Weekend) | (_, DayType::Holiday) => Some(&WEEKEND_TIMESLOTS_2026[..]),
        }
    }
}

/// Parses a `H:MM AM/PM` or `HH:MM AM/PM` time into minutes after midnight.
pub fn parse_time_to_minutes(time: &str) -> Option<u32> {
    let parts: Vec<&str> = time.split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }
    let time_parts: Vec<&str> = parts[0].split(':').collect();
    if time_parts.len() != 2 {
        return None;
    }

    let hour: u32 = time_parts[0].parse().ok()?;
    let minute: u32 = time_parts[1].parse().ok()?;
    let period = parts[1];

    if hour > 12 || minute >= 60 {
        return None;
    }

    if period != "AM" && period != "PM" {
        return None;
    }

    let mut total_minutes = (hour % 12) * 60 + minute;
    if period == "PM" {
        total_minutes += 12 * 60;
    }
    Some(total_minutes)
}

/// Formats minutes after midnight as `H:MM AM/PM`.
pub fn format_minutes_to_time(minutes: u32) -> String {
    let hour_24 = minutes / 60;
    let minute = minutes % 60;
    let period = if hour_24 >= 12 { "PM" } else { "AM" };
    let hour_12 = if hour_24 == 0 || hour_24 == 12 {
        12
    } else {
        hour_24 % 12
    };
    format!("{}:{:02} {}", hour_12, minute, period)
}

/// Parses a statement date in `DD Mon YY` format.
///
/// Returns `(day, month, full_year)`.
pub fn parse_date(date: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = date.split_whitespace().collect();
    if parts.len() != 3 {
        return None;
    }

    let day: u32 = parts[0].parse().ok()?;
    let month_str = parts[1];
    let year_str = parts[2];
    let year: u32 = 2000 + year_str.parse::<u32>().ok()?;

    let month = match month_str {
        "Jan" => 1,
        "Feb" => 2,
        "Mar" => 3,
        "Apr" => 4,
        "May" => 5,
        "Jun" => 6,
        "Jul" => 7,
        "Aug" => 8,
        "Sep" => 9,
        "Oct" => 10,
        "Nov" => 11,
        "Dec" => 12,
        _ => return None,
    };
    Some((day, month, year))
}

/// Returns whether a date falls on a weekend.
pub fn is_weekend(day: u32, month: u32, year: u32) -> bool {
    let date = Date::new(year as u64, month as u64, day as u64);
    date.is_weekend()
}

#[derive(serde::Deserialize)]
struct Holiday {
    year: u32,
    month: u32,
    day: u32,
}

fn get_holidays() -> &'static std::collections::HashSet<(u32, u32, u32)> {
    static HOLIDAYS: std::sync::OnceLock<std::collections::HashSet<(u32, u32, u32)>> =
        std::sync::OnceLock::new();
    HOLIDAYS.get_or_init(|| {
        let json_str = include_str!("holidays.json");
        let parsed: Vec<Holiday> = serde_json::from_str(json_str).expect("Valid holidays.json");
        parsed
            .into_iter()
            .map(|h| (h.year, h.month, h.day))
            .collect()
    })
}

/// Returns whether a date is listed in the bundled holiday calendar.
pub fn is_holiday(day: u32, month: u32, year: u32) -> bool {
    get_holidays().contains(&(year, month, day))
}

/// Classifies a statement date as weekday, weekend, or holiday.
pub fn classify_day(date: &str) -> Option<DayType> {
    if let Some((day, month, year)) = parse_date(date) {
        if is_holiday(day, month, year) {
            return Some(DayType::Holiday);
        }
        if is_weekend(day, month, year) {
            return Some(DayType::Weekend);
        }
        return Some(DayType::Weekday);
    }
    None
}

/// Per-trip optimization details included in cluster summaries.
#[derive(Debug, Serialize)]
pub struct TripSummary<'a> {
    /// Borrowed source trip record.
    pub trip: &'a TripRecord,
    /// Timeslot index used for average-time calculations, when available.
    pub avg_idx: Option<usize>,
    /// Estimated total cost if shifted to the previous timeslot.
    pub total_cost_previous_timeslot: Option<f64>,
    /// Estimated total cost if shifted to the next timeslot.
    pub total_cost_next_timeslot: Option<f64>,
    /// Lowest estimated optimized total cost, when cheaper than recorded.
    pub optimized_cost: Option<f64>,
    /// Estimated savings relative to the recorded total cost.
    pub optimized_saved: Option<f64>,
    /// Suggested replacement entry point for distance optimization.
    pub optimized_entry: Option<String>,
    /// Suggested replacement exit point for distance optimization.
    pub optimized_exit: Option<String>,
    /// Human-readable optimization note.
    pub optimization_note: Option<String>,
    /// Latest target time that would place the trip in the previous timeslot.
    pub prev_timeslot_target: Option<String>,
    /// Earliest target time that would place the trip in the next timeslot.
    pub next_timeslot_target: Option<String>,
}

/// Time-cluster summary for a group of similar trips.
#[derive(Debug, Serialize)]
pub struct CentroidData<'a> {
    /// Cluster centroid formatted as `H:MM AM/PM`.
    pub centroid_time: String,
    /// Trips assigned to this cluster.
    pub trips: Vec<TripSummary<'a>>,
    /// Average entry time for assigned trips.
    pub average_entry_time: String,
    /// Total recorded distance for assigned trips.
    pub total_distance: f64,
    /// Total recorded toll charge for assigned trips.
    pub total_toll_charge: f64,
    /// Total estimated charge if trips shifted to the previous timeslot.
    pub total_toll_charge_previous_timeslot: f64,
    /// Total estimated charge if trips shifted to the next timeslot.
    pub total_toll_charge_next_timeslot: f64,
    /// Total estimated savings from the best adjacent-timeslot options.
    pub total_optimized_savings: f64,
    /// Human-readable time-shift advice for the cluster.
    pub optimization_advice: Option<String>,
}

/// Time-based analysis for one transponder/plate and direction.
#[derive(Debug, Serialize)]
pub struct TransponderSummaryByTime<'a> {
    /// Transponder or plate identifier.
    pub transponder_plate: String,
    /// Direction shared by all trips in the summary.
    pub direction: Direction,
    /// Computed time clusters.
    pub centroids: Vec<CentroidData<'a>>,
    /// Display-ready centroid labels.
    pub formatted_centroids: Vec<String>,
}

/// Distance-cluster summary for a group of similar trips.
#[derive(Debug, Serialize)]
pub struct CentroidDataByDistance<'a> {
    /// Cluster centroid distance in kilometers.
    pub centroid_distance: f64,
    /// Trips assigned to this distance cluster.
    pub trips: Vec<TripSummary<'a>>,
    /// Average recorded trip distance in kilometers.
    pub average_distance: f64,
    /// Total recorded toll charge for assigned trips.
    pub total_toll_charge: f64,
    /// Total estimated savings from entry/exit point adjustments.
    pub total_optimized_savings: f64,
    /// Human-readable route-adjustment advice for the cluster.
    pub optimization_advice: Option<String>,
    /// Most common entry point in the cluster.
    pub representative_entry: Option<String>,
    /// Most common exit point in the cluster.
    pub representative_exit: Option<String>,
}

/// Distance-based analysis for one transponder/plate and direction.
#[derive(Debug, Serialize)]
pub struct TransponderSummaryByDistance<'a> {
    /// Transponder or plate identifier.
    pub transponder_plate: String,
    /// Direction shared by all trips in the summary.
    pub direction: Direction,
    /// Computed distance clusters.
    pub centroids: Vec<CentroidDataByDistance<'a>>,
    /// Display-ready centroid labels.
    pub formatted_centroids: Vec<String>,
}

// 1D K-means clustering (Angular/Circular for time)
fn k_means_1d(data: &[u32], k: usize) -> (Vec<u32>, f64) {
    if data.is_empty() || k == 0 {
        return (Vec::new(), 0.0);
    }

    // Initialize centroids (simple method: pick random or evenly spaced points)
    // Here we'll just pick the first k distinct points or evenly spaced if not enough distinct
    let mut centroids: Vec<f64> = data.iter().take(k).map(|&x| x as f64).collect();
    while centroids.len() < k {
        centroids.push(data[0] as f64); // Fallback
    }

    let mut assignments = vec![0; data.len()];
    let mut wcss = 0.0;

    for _ in 0..100 {
        // Max iterations
        let mut changed = false;
        wcss = 0.0;

        // Assignment step
        for (i, &point) in data.iter().enumerate() {
            let mut min_dist = f64::MAX;
            let mut best_cluster = 0;

            for (c_idx, &centroid) in centroids.iter().enumerate() {
                let diff = (point as f64 - centroid).abs();
                // Handle wrap-around (24 hours = 1440 minutes)
                let dist = diff.min(1440.0 - diff);
                if dist < min_dist {
                    min_dist = dist;
                    best_cluster = c_idx;
                }
            }
            if assignments[i] != best_cluster {
                assignments[i] = best_cluster;
                changed = true;
            }
            wcss += min_dist * min_dist;
        }

        // Update step
        if !changed {
            break;
        }

        for (c_idx, centroid) in centroids.iter_mut().enumerate().take(k) {
            let mut sum = 0.0;
            let mut count = 0;
            // We need to handle the circular mean carefully.
            // A simple approximation for now: if points are far apart, this might be tricky.
            // But for toll data, clusters are likely tight.
            // Let's use a simple linear mean for now, assuming clusters don't span midnight widely.
            // If they do, we'd need vector averaging.
            for (i, &point) in data.iter().enumerate() {
                if assignments[i] == c_idx {
                    // Adjust point to be close to current centroid to handle wrap-around for averaging
                    let mut p = point as f64;
                    if (p - *centroid).abs() > 720.0 {
                        if p < *centroid {
                            p += 1440.0;
                        } else {
                            p -= 1440.0;
                        }
                    }
                    sum += p;
                    count += 1;
                }
            }
            if count > 0 {
                let mut new_mean = sum / count as f64;
                if new_mean < 0.0 {
                    new_mean += 1440.0;
                }
                if new_mean >= 1440.0 {
                    new_mean -= 1440.0;
                }
                *centroid = new_mean;
            }
        }
    }

    let u32_centroids: Vec<u32> = centroids.iter().map(|&c| c.round() as u32).collect();
    (u32_centroids, wcss)
}

// 1D K-means clustering (Linear for distance)
fn k_means_1d_linear(data: &[f64], k: usize) -> (Vec<f64>, f64) {
    if data.is_empty() || k == 0 {
        return (Vec::new(), 0.0);
    }

    let mut centroids: Vec<f64> = data.iter().take(k).cloned().collect();
    while centroids.len() < k {
        centroids.push(data[0]);
    }

    let mut assignments = vec![0; data.len()];
    let mut wcss = 0.0;

    for _ in 0..100 {
        let mut changed = false;
        wcss = 0.0;

        for (i, &point) in data.iter().enumerate() {
            let mut min_dist = f64::MAX;
            let mut best_cluster = 0;

            for (c_idx, &centroid) in centroids.iter().enumerate() {
                let dist = (point - centroid).abs();
                if dist < min_dist {
                    min_dist = dist;
                    best_cluster = c_idx;
                }
            }
            if assignments[i] != best_cluster {
                assignments[i] = best_cluster;
                changed = true;
            }
            wcss += min_dist * min_dist;
        }

        if !changed {
            break;
        }

        for (c_idx, centroid) in centroids.iter_mut().enumerate().take(k) {
            let mut sum = 0.0;
            let mut count = 0;
            for (i, &point) in data.iter().enumerate() {
                if assignments[i] == c_idx {
                    sum += point;
                    count += 1;
                }
            }
            if count > 0 {
                *centroid = sum / count as f64;
            }
        }
    }

    (centroids, wcss)
}

fn find_best_k(wcss_values: &[f64]) -> usize {
    if wcss_values.len() < 2 {
        return 1;
    }
    // Simple elbow method: find the point with the maximum curvature or largest drop?
    // Let's look for the "elbow" where the reduction in WCSS slows down significantly.
    // A simple heuristic: if reduction is less than X% of previous reduction?
    // Or just pick k where WCSS is "low enough".

    // Let's try a simple angle-based method or just max distance from line connecting first and last.
    let n = wcss_values.len();
    let first = (1.0, wcss_values[0]);
    let last = (n as f64, wcss_values[n - 1]);

    let mut max_dist = -1.0;
    let mut best_k = 1;

    for (i, &wcss) in wcss_values.iter().enumerate().take(n) {
        let k = (i + 1) as f64;
        // Distance from point (k, wcss) to line defined by first and last
        // Line eq: (y2-y1)x - (x2-x1)y + x2y1 - y2x1 = 0
        let numerator = ((last.1 - first.1) * k - (last.0 - first.0) * wcss + last.0 * first.1
            - last.1 * first.0)
            .abs();
        let denominator = ((last.1 - first.1).powi(2) + (last.0 - first.0).powi(2)).sqrt();
        let dist = numerator / denominator;

        if dist > max_dist {
            max_dist = dist;
            best_k = i + 1;
        }
    }

    best_k
}

/// Performs time-based clustering and adjacent-timeslot savings analysis.
///
/// Input trips are expected to be grouped by `(transponder_or_plate, direction)`,
/// as returned by `csv_parser::parse_trips`.
pub fn analyze_trips_by_time<'a>(
    trips_by_transponder_direction: &'a [((String, Direction), Vec<TripRecord>)],
) -> Vec<TransponderSummaryByTime<'a>> {
    let mut summaries = Vec::new();

    for ((plate, direction), trips) in trips_by_transponder_direction {
        let minutes: Vec<u32> = trips
            .iter()
            .filter_map(|t| parse_time_to_minutes(&t.entry_time))
            .collect();

        if !minutes.is_empty() {
            let mut wcss_values = Vec::new();
            let mut clusters_map = HashMap::new();

            // Run for k=1 to 5 (or fewer if not enough points)
            let max_k = 5.min(minutes.len());
            for k in 1..=max_k {
                let (centroids, wcss) = k_means_1d(&minutes, k);
                wcss_values.push(wcss);
                clusters_map.insert(k, centroids);
            }

            let best_k = find_best_k(&wcss_values);

            if let Some(best_centroids) = clusters_map.get(&best_k) {
                // Create buckets for each centroid
                let mut clusters_buckets: HashMap<u32, Vec<&TripRecord>> = HashMap::new();
                for &centroid in best_centroids {
                    clusters_buckets.insert(centroid, Vec::new());
                }

                // Assign each trip to the nearest centroid
                for trip in trips {
                    if let Some(trip_minutes) = parse_time_to_minutes(&trip.entry_time) {
                        let mut min_dist = i32::MAX;
                        let mut best_c = None;

                        for &centroid in best_centroids {
                            let diff = (trip_minutes as i32 - centroid as i32).abs();
                            let dist = diff.min(1440 - diff); // Circular distance
                            if dist < min_dist {
                                min_dist = dist;
                                best_c = Some(centroid);
                            }
                        }

                        if let Some(c) = best_c
                            && min_dist <= 30
                            && let Some(bucket) = clusters_buckets.get_mut(&c)
                        {
                            bucket.push(trip);
                        }
                    }
                }

                let mut centroid_data_list = Vec::new();

                for &centroid in best_centroids {
                    let centroid_time_str = format_minutes_to_time(centroid);

                    // Retrieve trips for this centroid
                    let cluster_trips =
                        clusters_buckets.get(&centroid).cloned().unwrap_or_default();
                    let mut cluster_trip_minutes = Vec::new();

                    for trip in &cluster_trips {
                        if let Some(trip_minutes) = parse_time_to_minutes(&trip.entry_time) {
                            // Normalize trip minutes relative to centroid for averaging
                            let mut signed_diff = trip_minutes as i32 - centroid as i32;
                            if signed_diff > 720 {
                                signed_diff -= 1440;
                            } else if signed_diff < -720 {
                                signed_diff += 1440;
                            }
                            cluster_trip_minutes.push(centroid as i32 + signed_diff);
                        }
                    }

                    let mut trip_summaries = Vec::new();
                    let mut average_entry_time = "N/A".to_string();
                    let mut total_distance = 0.0;
                    let mut total_toll_charge = 0.0;
                    let mut total_toll_charge_previous_timeslot = 0.0;
                    let mut total_toll_charge_next_timeslot = 0.0;
                    let mut total_optimized_savings = 0.0;

                    if !cluster_trip_minutes.is_empty() {
                        let sum: i32 = cluster_trip_minutes.iter().sum();
                        let avg_minutes = sum as f64 / cluster_trip_minutes.len() as f64;
                        // Normalize back to 0-1439 range
                        let mut normalized_avg = avg_minutes.round() as i32;
                        while normalized_avg < 0 {
                            normalized_avg += 1440;
                        }
                        while normalized_avg >= 1440 {
                            normalized_avg -= 1440;
                        }
                        average_entry_time = format_minutes_to_time(normalized_avg as u32);

                        // Calculate previous and next timeslot totals
                        for trip in &cluster_trips {
                            let total_cost = trip.get_total_recorded_cost();
                            total_toll_charge += total_cost;
                            total_distance += trip.distance_km.trim().parse::<f64>().unwrap_or(0.0);

                            // Add fixed charges to previous/next timeslot estimates
                            let trip_toll =
                                trip.trip_toll_charge.trim().parse::<f64>().unwrap_or(0.0);
                            let camera = trip.camera_charge.trim().parse::<f64>().unwrap_or(0.0);
                            let fixed_charges = trip_toll + camera;

                            let (
                                avg_idx_opt,
                                prev_cost_opt,
                                next_cost_opt,
                                prev_target_opt,
                                next_target_opt,
                            ) = if let Some(timeslot_idx) =
                                trip.get_timeslot_index_for_time(&centroid_time_str)
                            {
                                let mut prev_c = None;
                                let mut next_c = None;
                                let timeslots_len = trip.get_timeslot_count().unwrap_or(0);
                                let mut savings: f64 = 0.0;

                                if timeslot_idx > 0 {
                                    if let Some((cost, _)) =
                                        trip.calculate_cost_at_timeslot(timeslot_idx - 1)
                                    {
                                        let full_prev_cost = cost + fixed_charges;
                                        total_toll_charge_previous_timeslot += full_prev_cost;
                                        prev_c = Some(full_prev_cost);
                                        if full_prev_cost < total_cost {
                                            savings = savings.max(total_cost - full_prev_cost);
                                        }
                                    }
                                } else if timeslot_idx == 0
                                    && let Some((cost, _)) =
                                        trip.calculate_cost_at_timeslot(timeslots_len - 1)
                                {
                                    let full_prev_cost = cost + fixed_charges;
                                    total_toll_charge_previous_timeslot += full_prev_cost;
                                    prev_c = Some(full_prev_cost);
                                    if full_prev_cost < total_cost {
                                        savings = savings.max(total_cost - full_prev_cost);
                                    }
                                }

                                if timeslot_idx < timeslots_len - 1 {
                                    if let Some((cost, _)) =
                                        trip.calculate_cost_at_timeslot(timeslot_idx + 1)
                                    {
                                        let full_next_cost = cost + fixed_charges;
                                        total_toll_charge_next_timeslot += full_next_cost;
                                        next_c = Some(full_next_cost);
                                        if full_next_cost < total_cost {
                                            savings = savings.max(total_cost - full_next_cost);
                                        }
                                    }
                                } else if timeslot_idx == timeslots_len - 1
                                    && let Some((cost, _)) = trip.calculate_cost_at_timeslot(0)
                                {
                                    let full_next_cost = cost + fixed_charges;
                                    total_toll_charge_next_timeslot += full_next_cost;
                                    next_c = Some(full_next_cost);
                                    if full_next_cost < total_cost {
                                        savings = savings.max(total_cost - full_next_cost);
                                    }
                                }

                                total_optimized_savings += savings;

                                // Calculate target times for previous/next timeslots
                                let mut prev_target_str = None;
                                let mut next_target_str = None;

                                if let Some(timeslots) = trip.get_timeslots() {
                                    // Previous Timeslot Target: 1 minute before current slot start
                                    // Current slot start is timeslots[timeslot_idx]
                                    if let Some(current_start_min) =
                                        parse_time_to_minutes(timeslots[timeslot_idx])
                                    {
                                        let target_min = if current_start_min == 0 {
                                            1439
                                        } else {
                                            current_start_min - 1
                                        };
                                        prev_target_str = Some(format_minutes_to_time(target_min));
                                    }

                                    // Next Timeslot Target: Start of next slot
                                    let next_slot_idx = if timeslot_idx == timeslots.len() - 1 {
                                        0
                                    } else {
                                        timeslot_idx + 1
                                    };
                                    next_target_str = Some(timeslots[next_slot_idx].to_string());
                                }

                                (
                                    Some(timeslot_idx),
                                    prev_c,
                                    next_c,
                                    prev_target_str,
                                    next_target_str,
                                )
                            } else {
                                (None, None, None, None, None)
                            };

                            let (optimized_cost, optimized_saved) =
                                if let (Some(pc), Some(nc)) = (prev_cost_opt, next_cost_opt) {
                                    if pc < total_cost && pc <= nc {
                                        (Some(pc), Some(total_cost - pc))
                                    } else if nc < total_cost {
                                        (Some(nc), Some(total_cost - nc))
                                    } else {
                                        (None, None)
                                    }
                                } else if let Some(pc) = prev_cost_opt {
                                    if pc < total_cost {
                                        (Some(pc), Some(total_cost - pc))
                                    } else {
                                        (None, None)
                                    }
                                } else if let Some(nc) = next_cost_opt {
                                    if nc < total_cost {
                                        (Some(nc), Some(total_cost - nc))
                                    } else {
                                        (None, None)
                                    }
                                } else {
                                    (None, None)
                                };

                            trip_summaries.push(TripSummary {
                                trip,
                                avg_idx: avg_idx_opt,
                                total_cost_previous_timeslot: prev_cost_opt,
                                total_cost_next_timeslot: next_cost_opt,
                                optimized_cost,
                                optimized_saved,
                                optimized_entry: None,
                                optimized_exit: None,
                                optimization_note: None,
                                prev_timeslot_target: prev_target_opt,
                                next_timeslot_target: next_target_opt,
                            });
                        }
                    } else {
                        // Use basic details if we can't calculate time-based stats
                        for trip in &cluster_trips {
                            let total_cost = trip.get_total_recorded_cost();
                            total_toll_charge += total_cost;
                            total_distance += trip.distance_km.trim().parse::<f64>().unwrap_or(0.0);

                            trip_summaries.push(TripSummary {
                                trip,
                                avg_idx: None,
                                total_cost_previous_timeslot: None,
                                total_cost_next_timeslot: None,
                                optimized_cost: None,
                                optimized_saved: None,
                                optimized_entry: None,
                                optimized_exit: None,
                                optimization_note: None,
                                prev_timeslot_target: None,
                                next_timeslot_target: None,
                            });
                        }
                    }

                    let mut optimization_advice = None;
                    if total_optimized_savings > 0.01 {
                        let mut targets = Vec::new();
                        for ts in &trip_summaries {
                            if let Some(saved) = ts.optimized_saved
                                && saved > 0.01
                            {
                                if let Some(prev) = ts.total_cost_previous_timeslot
                                    && ((ts.trip.get_total_recorded_cost() - prev).abs() < 0.001
                                        || ts.optimized_cost == Some(prev))
                                    && let Some(target) = &ts.prev_timeslot_target
                                {
                                    targets.push(format!("before {}", target));
                                }
                                if let Some(next) = ts.total_cost_next_timeslot
                                    && ((ts.trip.get_total_recorded_cost() - next).abs() < 0.001
                                        || ts.optimized_cost == Some(next))
                                    && let Some(target) = &ts.next_timeslot_target
                                {
                                    targets.push(format!("after {}", target));
                                }
                            }
                        }
                        if !targets.is_empty() {
                            targets.sort();
                            targets.dedup();
                            optimization_advice = Some(format!(
                                "Try leaving {} to save money",
                                targets.join(" or ")
                            ));
                        }
                    }

                    let centroid_data = CentroidData {
                        centroid_time: centroid_time_str,
                        trips: trip_summaries,
                        average_entry_time,
                        total_distance,
                        total_toll_charge,
                        total_toll_charge_previous_timeslot,
                        total_toll_charge_next_timeslot,
                        total_optimized_savings,
                        optimization_advice,
                    };
                    centroid_data_list.push(centroid_data);
                }

                // Filter out centroids with no trips
                // We need to keep formatted_centroids in sync with the filtered list
                // Since formatted_centroids in the summary is just a list of strings,
                // and we want it to match the actual displayed centroids, we should rebuild it.

                let filtered_centroid_data: Vec<CentroidData> = centroid_data_list
                    .into_iter()
                    .filter(|c| !c.trips.is_empty())
                    .collect();

                let filtered_formatted_centroids: Vec<String> = filtered_centroid_data
                    .iter()
                    .map(|c| c.centroid_time.clone())
                    .collect();

                summaries.push(TransponderSummaryByTime {
                    transponder_plate: plate.clone(),
                    direction: direction.clone(),
                    centroids: filtered_centroid_data,
                    formatted_centroids: filtered_formatted_centroids,
                });
            }
        }
    }

    summaries
}

/// Performs distance-based clustering and entry/exit adjustment analysis.
///
/// Input trips are expected to be grouped by `(transponder_or_plate, direction)`,
/// as returned by `csv_parser::parse_trips`.
pub fn analyze_trips_by_distance<'a>(
    trips_by_transponder_direction: &'a [((String, Direction), Vec<TripRecord>)],
) -> Vec<TransponderSummaryByDistance<'a>> {
    let mut summaries = Vec::new();

    for ((plate, direction), trips) in trips_by_transponder_direction {
        let distances: Vec<f64> = trips
            .iter()
            .filter_map(|t| t.distance_km.trim().parse::<f64>().ok())
            .collect();

        if !distances.is_empty() {
            let mut wcss_values = Vec::new();
            let mut clusters_map = HashMap::new();

            // Run for k=1 to 5 (or fewer if not enough points)
            let max_k = 5.min(distances.len());
            for k in 1..=max_k {
                let (centroids, wcss) = k_means_1d_linear(&distances, k);
                wcss_values.push(wcss);
                clusters_map.insert(k, centroids);
            }

            let best_k = find_best_k(&wcss_values);

            if let Some(best_centroids) = clusters_map.get(&best_k) {
                // Create buckets for each centroid index
                let mut clusters_buckets: HashMap<usize, Vec<&TripRecord>> = HashMap::new();
                for i in 0..best_centroids.len() {
                    clusters_buckets.insert(i, Vec::new());
                }

                // Assign each trip to the nearest centroid
                for trip in trips {
                    if let Ok(dist) = trip.distance_km.trim().parse::<f64>() {
                        let mut min_diff = f64::MAX;
                        let mut best_idx = None;

                        for (i, &centroid) in best_centroids.iter().enumerate() {
                            let diff = (dist - centroid).abs();
                            if diff < min_diff {
                                min_diff = diff;
                                best_idx = Some(i);
                            }
                        }

                        if let Some(idx) = best_idx
                            && min_diff <= 5.0
                            && let Some(bucket) = clusters_buckets.get_mut(&idx)
                        {
                            bucket.push(trip);
                        }
                    }
                }

                let mut centroid_data_list = Vec::new();

                for (idx, &centroid) in best_centroids.iter().enumerate() {
                    // Retrieve trips for this centroid
                    let assigned_trips = clusters_buckets.get(&idx).cloned().unwrap_or_default();

                    let mut cluster_trips = Vec::new(); // Summaries
                    let mut total_distance = 0.0;
                    let mut total_toll_charge = 0.0;
                    let mut total_optimized_savings = 0.0;

                    for trip in &assigned_trips {
                        // All trips in this bucket are assigned here.
                        let dist = trip.distance_km.trim().parse::<f64>().unwrap_or(0.0);

                        let mut optimized_cost = None;
                        let mut optimized_saved = None;
                        let mut optimized_entry = None;
                        let mut optimized_exit = None;
                        let mut optimization_note = None;

                        if let (Some(start_idx), Some(end_idx), Some(timeslot_idx)) = (
                            trip.get_access_point_index(&trip.entry_point),
                            trip.get_access_point_index(&trip.exit_point),
                            trip.get_timeslot_index(),
                        ) {
                            let is_hwy_entry = trip.entry_point.to_lowercase().starts_with("hwy")
                                || trip.entry_point.eq_ignore_ascii_case("qew");
                            let direction = trip.direction.as_ref();

                            let mut new_start_idx = start_idx;
                            let mut new_end_idx = end_idx;
                            let mut strategy = "";

                            match direction {
                                Some(Direction::Eastbound) => {
                                    if is_hwy_entry {
                                        // Shrink from Exit side (move exit closer to entry/start)
                                        // Eastbound: start < end. Move end to end - 1.
                                        if new_end_idx > new_start_idx {
                                            new_end_idx -= 1;
                                            strategy = "Exit -1";
                                        }
                                    } else {
                                        // Shrink from Entry side (move entry closer to exit/end)
                                        // Eastbound: start < end. Move start to start + 1.
                                        if new_start_idx < new_end_idx {
                                            new_start_idx += 1;
                                            strategy = "Entry +1";
                                        }
                                    }
                                }
                                Some(Direction::Westbound) => {
                                    if is_hwy_entry {
                                        // Shrink from Exit side (move exit closer to entry/start)
                                        // Westbound: start > end. Move end to end + 1.
                                        if new_end_idx < new_start_idx {
                                            new_end_idx += 1;
                                            strategy = "Exit +1";
                                        }
                                    } else {
                                        // Shrink from Entry side (move entry closer to exit/end)
                                        // Westbound: start > end. Move start to start - 1.
                                        if new_start_idx > new_end_idx {
                                            new_start_idx -= 1;
                                            strategy = "Entry -1";
                                        }
                                    }
                                }
                                None => {}
                            }

                            if new_start_idx != start_idx || new_end_idx != end_idx {
                                // Calculate cost for optimized trip
                                if let Some((toll_cost, _)) = trip.calculate_cost_from_indices(
                                    new_start_idx,
                                    new_end_idx,
                                    timeslot_idx,
                                ) {
                                    let current_toll =
                                        trip.toll_charge.trim().parse::<f64>().unwrap_or(0.0);
                                    // Trip toll and camera charges remain constant
                                    let trip_toll =
                                        trip.trip_toll_charge.trim().parse::<f64>().unwrap_or(0.0);
                                    let camera =
                                        trip.camera_charge.trim().parse::<f64>().unwrap_or(0.0);

                                    let current_total = current_toll + trip_toll + camera;
                                    let new_total = toll_cost + trip_toll + camera;

                                    if new_total < current_total {
                                        optimized_cost = Some(new_total);
                                        optimized_saved = Some(current_total - new_total);
                                        optimized_entry =
                                            Some(ACCESS_POINTS[new_start_idx].to_string());
                                        optimized_exit =
                                            Some(ACCESS_POINTS[new_end_idx].to_string());
                                        let new_point_name = if new_start_idx != start_idx {
                                            ACCESS_POINTS[new_start_idx]
                                        } else {
                                            ACCESS_POINTS[new_end_idx]
                                        };

                                        let change_type = if strategy.contains("Entry") {
                                            "Enter on"
                                        } else {
                                            "Exit on"
                                        };

                                        optimization_note = Some(format!(
                                            "{} {} to save ${:.2}",
                                            change_type,
                                            new_point_name,
                                            current_total - new_total
                                        ));

                                        total_optimized_savings += current_total - new_total;
                                    }
                                }
                            }
                        }

                        cluster_trips.push(TripSummary {
                            trip,
                            avg_idx: None,
                            total_cost_previous_timeslot: None,
                            total_cost_next_timeslot: None,
                            optimized_cost,
                            optimized_saved,
                            optimized_entry,
                            optimized_exit,
                            optimization_note,
                            prev_timeslot_target: None,
                            next_timeslot_target: None,
                        });

                        total_distance += dist;
                        if let Ok(c) = trip.toll_charge.trim().parse::<f64>() {
                            total_toll_charge += c;
                        }
                    }

                    let average_distance = if !cluster_trips.is_empty() {
                        total_distance / cluster_trips.len() as f64
                    } else {
                        0.0
                    };

                    let mut advice_map = HashMap::new();
                    let mut entry_counts = HashMap::new();
                    let mut exit_counts = HashMap::new();

                    for ts in &cluster_trips {
                        if let Some(note) = &ts.optimization_note {
                            *advice_map.entry(note.clone()).or_insert(0) += 1;
                        }
                        *entry_counts.entry(ts.trip.entry_point.clone()).or_insert(0) += 1;
                        *exit_counts.entry(ts.trip.exit_point.clone()).or_insert(0) += 1;
                    }

                    let representative_entry = entry_counts
                        .into_iter()
                        .max_by_key(|&(_, count)| count)
                        .map(|(name, _)| name);
                    let representative_exit = exit_counts
                        .into_iter()
                        .max_by_key(|&(_, count)| count)
                        .map(|(name, _)| name);

                    let mut optimization_advice = None;
                    if !advice_map.is_empty() {
                        let mut unique_advice: Vec<String> = advice_map
                            .keys()
                            .map(|a| {
                                if let Some(idx) = a.find(" to save $") {
                                    &a[..idx]
                                } else if let Some(idx) = a.find(" (Save") {
                                    &a[..idx]
                                } else {
                                    a
                                }
                                .replace(" to save some $$$", "")
                            })
                            .collect();
                        unique_advice.sort();
                        unique_advice.dedup();

                        if !unique_advice.is_empty() {
                            optimization_advice =
                                Some(format!("{} to save money", unique_advice.join(" and ")));
                        }
                    }

                    let centroid_data = CentroidDataByDistance {
                        centroid_distance: centroid,
                        trips: cluster_trips,
                        average_distance,
                        total_toll_charge,
                        total_optimized_savings,
                        optimization_advice,
                        representative_entry,
                        representative_exit,
                    };
                    centroid_data_list.push(centroid_data);
                }

                // Sort centroids by distance
                centroid_data_list.sort_by(|a, b| {
                    a.centroid_distance
                        .partial_cmp(&b.centroid_distance)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

                let filtered_centroid_data: Vec<CentroidDataByDistance> = centroid_data_list
                    .into_iter()
                    .filter(|c| !c.trips.is_empty())
                    .collect();

                let filtered_formatted_centroids: Vec<String> = filtered_centroid_data
                    .iter()
                    .map(|c| format!("{:.2} km", c.centroid_distance))
                    .collect();

                summaries.push(TransponderSummaryByDistance {
                    transponder_plate: plate.clone(),
                    direction: direction.clone(),
                    centroids: filtered_centroid_data,
                    formatted_centroids: filtered_formatted_centroids,
                });
            }
        }
    }

    summaries
}

/// Parses either `H:MM AM/PM` or 24-hour `HH:MM` into minutes after midnight.
pub fn parse_time_flexible(time_str: &str) -> Option<u32> {
    if let Some(m) = parse_time_to_minutes(time_str) {
        return Some(m);
    }

    // Try HH:MM (24h)
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() >= 2 {
        let h: u32 = parts[0].trim().parse().ok()?;
        let m: u32 = parts[1]
            .trim()
            .get(0..2)
            .and_then(|s| s.parse::<u32>().ok())
            .or_else(|| parts[1].trim().parse().ok())?;
        if h < 24 && m < 60 {
            return Some(h * 60 + m);
        }
    }
    None
}

/// Classifies a `chrono::NaiveDate` into the pricing day bucket.
pub fn classify_day_type(date: NaiveDate) -> DayType {
    let day = date.day();
    let month = date.month();
    let year = date.year() as u32;

    if is_holiday(day, month, year) {
        DayType::Holiday
    } else if is_weekend(day, month, year) {
        DayType::Weekend
    } else {
        DayType::Weekday
    }
}

/// Returns current and next 2026 average pricing for a date, time, and vehicle.
///
/// `date_str` must be `YYYY-MM-DD`. `time_str` accepts `H:MM AM/PM` or
/// 24-hour `HH:MM`.
pub fn get_pricing(
    date_str: &str,
    time_str: &str,
    vehicle_class: VehicleClass,
) -> Result<PricingResponse> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|e| {
        anyhow!(
            "Invalid date format '{}'. Expected YYYY-MM-DD: {}",
            date_str,
            e
        )
    })?;

    let day_type = classify_day_type(date);
    let minutes = parse_time_flexible(time_str).ok_or_else(|| {
        anyhow!(
            "Invalid time format '{}'. Expected HH:MM AM/PM or HH:MM",
            time_str
        )
    })?;

    let slots = match day_type {
        DayType::Weekday => &WEEKDAY_TIMESLOTS_2026[..],
        DayType::Weekend | DayType::Holiday => &WEEKEND_TIMESLOTS_2026[..],
    };

    let slot_minutes: Vec<u32> = slots
        .iter()
        .filter_map(|&t| parse_time_to_minutes(t))
        .collect();

    if slot_minutes.is_empty() {
        return Err(anyhow!("No timeslots defined"));
    }

    let mut current_idx = slot_minutes.len() - 1;
    for (i, &slot_time) in slot_minutes.iter().enumerate() {
        if minutes < slot_time {
            if i == 0 {
                current_idx = slot_minutes.len() - 1;
            } else {
                current_idx = i - 1;
            }
            break;
        }
        current_idx = i;
    }

    let next_idx = (current_idx + 1) % slot_minutes.len();

    let (next_slots, next_day_type) = if next_idx == 0 {
        let next_date = date + Duration::days(1);
        let ndt = classify_day_type(next_date);
        let ns = match ndt {
            DayType::Weekday => &WEEKDAY_TIMESLOTS_2026[..],
            DayType::Weekend | DayType::Holiday => &WEEKEND_TIMESLOTS_2026[..],
        };
        (ns, ndt)
    } else {
        (slots, day_type.clone())
    };

    Ok(PricingResponse {
        current: TimeslotPrices {
            timeslot: slots[current_idx].to_string(),
            average_eb: vehicle_class.get_average_rate(
                &day_type,
                &Direction::Eastbound,
                true,
                current_idx,
            ),
            average_wb: vehicle_class.get_average_rate(
                &day_type,
                &Direction::Westbound,
                true,
                current_idx,
            ),
        },
        next: TimeslotPrices {
            timeslot: next_slots[next_idx].to_string(),
            average_eb: vehicle_class.get_average_rate(
                &next_day_type,
                &Direction::Eastbound,
                true,
                next_idx,
            ),
            average_wb: vehicle_class.get_average_rate(
                &next_day_type,
                &Direction::Westbound,
                true,
                next_idx,
            ),
        },
        day_type: format!("{:?} ({})", day_type, vehicle_class.to_str()),
    })
}

/// Calculates the base toll and distance for a single planned trip.
///
/// `date_str` must be `YYYY-MM-DD`. `time_str` accepts `H:MM AM/PM` or
/// 24-hour `HH:MM`. The returned tuple is
/// `(base_toll_dollars, distance_km, direction, day_type)`.
pub fn calculate_single_trip_cost(
    entry_point: &str,
    exit_point: &str,
    date_str: &str,
    time_str: &str,
    vehicle_class: VehicleClass,
) -> Result<(f64, f64, Direction, DayType)> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|e| {
        anyhow!(
            "Invalid date format '{}'. Expected YYYY-MM-DD: {}",
            date_str,
            e
        )
    })?;

    let day_type = classify_day_type(date);
    let minutes = parse_time_flexible(time_str).ok_or_else(|| {
        anyhow!(
            "Invalid time format '{}'. Expected HH:MM AM/PM or HH:MM",
            time_str
        )
    })?;

    let start_idx = get_access_point_index(entry_point)
        .ok_or_else(|| anyhow!("Invalid entry point: {}", entry_point))?;
    let end_idx = get_access_point_index(exit_point)
        .ok_or_else(|| anyhow!("Invalid exit point: {}", exit_point))?;

    let direction = if end_idx > start_idx {
        Direction::Eastbound
    } else {
        Direction::Westbound
    };

    let year = date.year() as u32;

    // Find timeslot index
    let slots = match &day_type {
        DayType::Weekday => {
            if year <= 2025 {
                &WEEKDAY_TIMESLOTS_2025[..]
            } else {
                &WEEKDAY_TIMESLOTS_2026[..]
            }
        }
        DayType::Weekend | DayType::Holiday => {
            if year <= 2025 {
                &WEEKEND_TIMESLOTS_2025[..]
            } else {
                &WEEKEND_TIMESLOTS_2026[..]
            }
        }
    };

    let slot_minutes: Vec<u32> = slots
        .iter()
        .filter_map(|&t| parse_time_to_minutes(t))
        .collect();

    if slot_minutes.is_empty() {
        return Err(anyhow!("No timeslots defined"));
    }

    let mut timeslot_idx = slot_minutes.len() - 1;
    for (i, &slot_time) in slot_minutes.iter().enumerate() {
        if minutes < slot_time {
            if i == 0 {
                timeslot_idx = slot_minutes.len() - 1;
            } else {
                timeslot_idx = i - 1;
            }
            break;
        }
        timeslot_idx = i;
    }

    let route_cost = calculate_route_cost(
        start_idx,
        end_idx,
        &direction,
        &day_type,
        vehicle_class,
        year,
        timeslot_idx,
    )
    .ok_or_else(|| anyhow!("Invalid route: {} -> {}", entry_point, exit_point))?;

    Ok((
        route_cost.base_toll,
        route_cost.distance_km,
        direction,
        day_type,
    ))
}
