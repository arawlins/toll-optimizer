use simple_datetime_rs::Date;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

mod vehicle_class;
use vehicle_class::light_vehicles;

pub const OLD_ACCESSS_POINTS: [&str; 9] = [
    "LakeRidg",
    "LakeRidge",
    "Baldwin",
    "Thickson",
    "Simcoe",
    "Hwy412",
    "35/115",
    "Hwy35/115",
    "Hwy418",
];

const WEEKDAY_TIMESLOTS_2026: [&str; 8] = [
    "5:00 AM", "7:00 AM", "9:30 AM", "10:30 AM", "2:30 PM", "3:30 PM", "6:00 PM", "9:00 PM",
];
const WEEKDAY_TIMESLOTS_2025: [&str; 9] = [
    "12:00 AM", "6:00 AM", "7:00 AM", "9:30 AM", "10:30 AM", "2:30 PM", "3:30 PM", "6:00 PM",
    "7:00 PM",
];

const WEEKEND_TIMESLOTS_2026: [&str; 4] = ["8:30 AM", "10:00 AM", "7:00 PM", "9:00 PM"];

const WEEKEND_TIMESLOTS_2025: [&str; 5] = ["12:00 AM", "8:30 AM", "10:00 AM", "7:00 PM", "9:00 PM"];

pub const ACCESS_POINT_SYNONYMS: [(&str, &str); 3] = [
    ("Brock", "Brock(Hwy7)"),
    ("Brock407", "Brock(Hwy7)"),
    ("YorkDur", "York-DurhamLine"),
];

pub const ACCESS_POINTS: [&str; 41] = [
    "QEW",
    "Dundas",
    "Appleby",
    "Bronte",
    "Neyagawa",
    "Trafalgar",
    "Hwy403",
    "Britannia",
    "Derry",
    "Hwy401",
    "Mississauga",
    "Mavis",
    "Hurontario",
    "Hwy410",
    "Dixie",
    "Bramalea",
    "Airport",
    "Goreway",
    "Hwy427",
    "Hwy27",
    "PineValley",
    "Weston",
    "Hwy400",
    "Jane",
    "Keele",
    "Dufferin",
    "Bathurst",
    "Yonge",
    "Bayview",
    "Leslie",
    "Hwy404",
    "Woodbine",
    "Warden",
    "Kennedy",
    "McCowan",
    "Markham",
    "NinthLine",
    "DonaldCousensPk",
    "York-DurhamLine",
    "Whites",
    "Brock(Hwy7)",
];

const ACCESS_POINT_DISTANCES: [f32; 40] = [
    6.062, // QEW-Dundas
    3.847, // Dundas-Appleby
    4.153, // Appleby-Bronte
    4.927, // Bronte-Neyagawa
    3.227, // Neyagawa-Trafalgar
    2.917, // Trafalgar-Hwy403
    4.299, // Hwy403-Britannia
    3.073, // Britannia-Derry
    2.507, // Derry-Hwy401
    5.291, // Hwy401-Mississauga
    3.239, // Mississauga-Mavis
    2.310, // Mavis-Hurontario
    2.147, // Hurontario-Hwy410
    2.223, // Hwy410-Dixie
    1.481, // Dixie-Bramalea (calc)
    3.178, // Bramalea-Airport
    1.386, // Airport-Goreway (calc)
    3.235, // Goreway-Hwy427
    1.324, // Hwy427-Hwy27 (calc)
    4.061, // Hwy27-PineValley
    2.170, // PineValley-Weston
    0.691, // Weston-Hwy400 (calc)
    1.183, // Hwy400-Jane (calc)
    2.199, // Jane-Keele
    3.521, // Keele-Dufferin
    2.194, // Dufferin-Bathurst
    2.172, // Bathurst-Yonge
    1.930, // Yonge-Bayview
    2.076, // Bayview-Leslie
    0.997, // Leslie-Hwy404 (calc)
    1.029, // Hwy404-Woodbine (calc)
    2.078, // Woodbine-Warden
    1.930, // Warden-Kennedy
    2.215, // Kennedy-McCowan
    2.103, // McCowan-Markham
    2.074, // Markham-NinthLine
    1.686, // NinthLine-DonaldCousensPk
    2.976, // DonaldCousensPk-York-DurhamLine
    3.771, // York-DurhamLine-Whites
    4.083, // Whites-Brock(Hwy7)
];

const EB_ZONES: [(&str, u8); 41] = [
    ("QEW", 1),
    ("Dundas", 2),
    ("Appleby", 2),
    ("Bronte", 2),
    ("Neyagawa", 3),
    ("Trafalgar", 3),
    ("Hwy403", 4),
    ("Britannia", 4),
    ("Derry", 4),
    ("Hwy401", 5),
    ("Mississauga", 5),
    ("Mavis", 5),
    ("Hurontario", 5),
    ("Hwy410", 6),
    ("Dixie", 6),
    ("Bramalea", 6),
    ("Airport", 6),
    ("Goreway", 6),
    ("Hwy427", 7),
    ("Hwy27", 7),
    ("PineValley", 7),
    ("Weston", 7),
    ("Hwy400", 8),
    ("Jane", 8),
    ("Keele", 8),
    ("Dufferin", 8),
    ("Bathurst", 8),
    ("Yonge", 9),
    ("Bayview", 9),
    ("Leslie", 9),
    ("Hwy404", 10),
    ("Woodbine", 10),
    ("Warden", 10),
    ("Kennedy", 10),
    ("McCowan", 11),
    ("Markham", 11),
    ("NinthLine", 11),
    ("DonaldCousensPk", 11),
    ("York-DurhamLine", 12),
    ("Whites", 12),
    ("Brock(Hwy7)", 12),
];

const WB_ZONES: [(&str, u8); 41] = [
    ("QEW", 1),
    ("Dundas", 1),
    ("Appleby", 2),
    ("Bronte", 2),
    ("Neyagawa", 2),
    ("Trafalgar", 3),
    ("Hwy403", 3),
    ("Britannia", 4),
    ("Derry", 4),
    ("Hwy401", 4),
    ("Mississauga", 5),
    ("Mavis", 5),
    ("Hurontario", 5),
    ("Hwy410", 5),
    ("Dixie", 6),
    ("Bramalea", 6),
    ("Airport", 6),
    ("Goreway", 6),
    ("Hwy427", 6),
    ("Hwy27", 7),
    ("PineValley", 7),
    ("Weston", 7),
    ("Hwy400", 7),
    ("Jane", 8),
    ("Keele", 8),
    ("Dufferin", 8),
    ("Bathurst", 8),
    ("Yonge", 8),
    ("Bayview", 9),
    ("Leslie", 9),
    ("Hwy404", 9),
    ("Woodbine", 10),
    ("Warden", 10),
    ("Kennedy", 10),
    ("McCowan", 10),
    ("Markham", 11),
    ("NinthLine", 11),
    ("DonaldCousensPk", 11),
    ("York-DurhamLine", 11),
    ("Whites", 12),
    ("Brock(Hwy7)", 12),
];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Eastbound,
    Westbound,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DayType {
    Weekday,
    Weekend,
    Holiday,
}

#[derive(Debug)]
pub struct TripRecord {
    pub transponder_plate: String,
    pub vehicle_class: String,
    pub date_of_trip: String,
    pub entry_time: String,
    pub entry_point: String,
    pub exit_point: String,
    pub distance_km: String,
    pub toll_charge: String,
    pub trip_toll_charge: String,
    pub camera_charge: String,
    pub direction: Option<Direction>,
    pub day_type: Option<DayType>,
}

#[derive(Debug)]
struct CentroidData<'a> {
    centroid_time: String,
    trips: Vec<&'a TripRecord>,
    average_entry_time: String,
    total_distance: f64,
    total_toll_charge: f64,
    total_toll_charge_previous_timeslot: f64,
    total_toll_charge_next_timeslot: f64,
}

#[derive(Debug)]
struct TransponderSummaryByTime<'a> {
    transponder_plate: String,
    direction: Direction,
    centroids: Vec<CentroidData<'a>>,
    best_k: usize,
    formatted_centroids: Vec<String>,
}

impl TripRecord {
    pub fn from_csv_line(line: &str) -> Option<Self> {
        // The CSV format seems to be "value","value","value"...
        // We can split by "," to separate fields.
        // Note: This assumes that the values themselves do not contain ",".
        // A robust CSV parser would be better, but we are sticking to the simple logic for now.

        let parts: Vec<&str> = line.split("\",\"").collect();
        if parts.len() < 10 {
            return None;
        }

        // The first and last elements will still have a leading/trailing quote
        let first = parts[0].trim_start_matches('"');
        let last = parts[parts.len() - 1].trim_end_matches('"');

        let entry_point = parts[4].to_string();
        let exit_point = parts[5].to_string();
        let date_of_trip = parts[2].to_string();

        let day_type = classify_day(&date_of_trip);

        Some(TripRecord {
            transponder_plate: first.to_string(),
            vehicle_class: parts[1].to_string(),
            date_of_trip,
            entry_time: parts[3].to_string(),
            entry_point,
            exit_point,
            distance_km: parts[6].to_string(),
            toll_charge: parts[7].to_string(),
            trip_toll_charge: parts[8].to_string(),
            camera_charge: last.to_string(),
            direction: None,
            day_type,
        })
    }
    fn get_timeslot_count(&self) -> Option<usize> {
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

    fn get_timeslot_index_for_time(&self, time_str: &str) -> Option<usize> {
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

    fn get_timeslot_index(&self) -> Option<usize> {
        self.get_timeslot_index_for_time(&self.entry_time)
    }

    fn calculate_cost_at_timeslot(&self, timeslot_idx: usize) -> Option<(f64, f64)> {
        let direction = self.direction.as_ref()?;
        let day_type = self.day_type.as_ref()?;
        let (_, _, year) = parse_date(&self.date_of_trip)?;

        // Use ACCESS_POINTS as the canonical list for indices
        let start_idx = ACCESS_POINTS
            .iter()
            .position(|&name| name == self.entry_point)?;
        let end_idx = ACCESS_POINTS
            .iter()
            .position(|&name| name == self.exit_point)?;

        let mut total_cost = 0.0;
        let mut total_distance = 0.0;

        match direction {
            Direction::Eastbound => {
                if start_idx >= end_idx {
                    return None;
                } // Invalid for Eastbound
                // Segments are from start_idx to end_idx - 1
                for i in start_idx..end_idx {
                    let distance = ACCESS_POINT_DISTANCES[i] as f64;
                    total_distance += distance;

                    // Look up zone using the access point name from ACCESS_POINTS
                    let ap_name = ACCESS_POINTS[i];
                    let zone = EB_ZONES.iter().find(|&&(name, _)| name == ap_name)?.1 as usize;

                    // Price lookup
                    let price_rate = match (year, day_type) {
                        (y, DayType::Weekday) if y <= 2025 => {
                            light_vehicles::WEEKDAY_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025
                                [timeslot_idx][zone - 1]
                        }
                        (_, DayType::Weekday) => {
                            light_vehicles::WEEKDAY_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026
                                [timeslot_idx][zone - 1]
                        }
                        (y, DayType::Weekend | DayType::Holiday) if y <= 2025 => {
                            light_vehicles::WEEKEND_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025
                                [timeslot_idx][zone - 1]
                        }
                        (_, DayType::Weekend | DayType::Holiday) => {
                            light_vehicles::WEEKEND_EB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026
                                [timeslot_idx][zone - 1]
                        }
                    };
                    total_cost += distance * price_rate;
                }
            }
            Direction::Westbound => {
                if start_idx <= end_idx {
                    return None;
                } // Invalid for Westbound
                // Segments are from end_idx to start_idx - 1 (traversed in reverse)
                for i in end_idx..start_idx {
                    let distance = ACCESS_POINT_DISTANCES[i] as f64;
                    total_distance += distance;

                    // For WB, use the zone of the entry point into the segment (higher index)
                    let ap_name = ACCESS_POINTS[i + 1];
                    let zone = WB_ZONES.iter().find(|&&(name, _)| name == ap_name)?.1 as usize;

                    let price_rate = match (year, day_type) {
                        (y, DayType::Weekday) if y <= 2025 => {
                            light_vehicles::WEEKDAY_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025
                                [timeslot_idx][zone - 1]
                        }
                        (_, DayType::Weekday) => {
                            light_vehicles::WEEKDAY_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026
                                [timeslot_idx][zone - 1]
                        }
                        (y, DayType::Weekend | DayType::Holiday) if y <= 2025 => {
                            light_vehicles::WEEKEND_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2025
                                [timeslot_idx][zone - 1]
                        }
                        (_, DayType::Weekend | DayType::Holiday) => {
                            light_vehicles::WEEKEND_WB_TOLL_PRICES_BY_TIMESLOT_AND_ZONE_2026
                                [timeslot_idx][zone - 1]
                        }
                    };

                    total_cost += distance * price_rate;
                }
            }
        }

        Some((total_cost / 100.0, total_distance)) // Convert cents to dollars
    }

    fn calculate_cost(&self) -> Option<(f64, f64)> {
        let timeslot_idx = self.get_timeslot_index()?;
        self.calculate_cost_at_timeslot(timeslot_idx)
    }
}

fn parse_time_to_minutes(time: &str) -> Option<u32> {
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

    let mut total_minutes = (hour % 12) * 60 + minute;
    if period == "PM" {
        total_minutes += 12 * 60;
    }
    Some(total_minutes)
}

fn format_minutes_to_time(minutes: u32) -> String {
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

fn parse_date(date: &str) -> Option<(u32, u32, u32)> {
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

fn is_weekend(day: u32, month: u32, year: u32) -> bool {
    let date = Date::new(year as u64, month as u64, day as u64);
    date.is_weekend()
}

fn is_holiday(day: u32, month: u32, year: u32) -> bool {
    match (year, month, day) {
        // 2025 Holidays
        (2025, 1, 1) => true,   // New Year's Day
        (2025, 2, 17) => true,  // Family Day
        (2025, 4, 18) => true,  // Good Friday
        (2025, 5, 19) => true,  // Victoria Day
        (2025, 7, 1) => true,   // Canada Day
        (2025, 9, 1) => true,   // Labour Day
        (2025, 10, 13) => true, // Thanksgiving
        (2025, 12, 25) => true, // Christmas Day
        (2025, 12, 26) => true, // Boxing Day

        // 2024 Holidays
        (2024, 1, 1) => true,   // New Year's Day
        (2024, 2, 19) => true,  // Family Day
        (2024, 3, 29) => true,  // Good Friday
        (2024, 5, 20) => true,  // Victoria Day
        (2024, 7, 1) => true,   // Canada Day
        (2024, 9, 2) => true,   // Labour Day
        (2024, 10, 14) => true, // Thanksgiving
        (2024, 12, 25) => true, // Christmas Day
        (2024, 12, 26) => true, // Boxing Day

        // 2026 Holidays
        (2026, 1, 1) => true,   // New Year's Day
        (2026, 2, 16) => true,  // Family Day
        (2026, 4, 3) => true,   // Good Friday
        (2026, 5, 18) => true,  // Victoria Day
        (2026, 7, 1) => true,   // Canada Day
        (2026, 9, 7) => true,   // Labour Day
        (2026, 10, 12) => true, // Thanksgiving
        (2026, 12, 25) => true, // Christmas Day
        (2026, 12, 26) => true, // Boxing Day

        _ => false,
    }
}

fn classify_day(date: &str) -> Option<DayType> {
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

// 1D K-means clustering
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

        for c_idx in 0..k {
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
                    if (p - centroids[c_idx]).abs() > 720.0 {
                        if p < centroids[c_idx] {
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
                centroids[c_idx] = new_mean;
            }
        }
    }

    let u32_centroids: Vec<u32> = centroids.iter().map(|&c| c.round() as u32).collect();
    (u32_centroids, wcss)
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

    for i in 0..n {
        let k = (i + 1) as f64;
        let wcss = wcss_values[i];
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

fn analyze_trips<'a>(
    trips_by_transponder_direction: Vec<((String, Direction), Vec<&'a TripRecord>)>,
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
                let formatted_centroids: Vec<String> = best_centroids
                    .iter()
                    .map(|&c| format_minutes_to_time(c))
                    .collect();

                let mut centroid_data_list = Vec::new();

                for &centroid in best_centroids {
                    let centroid_time_str = format_minutes_to_time(centroid);

                    let mut cluster_trips = Vec::new();
                    let mut cluster_trip_minutes = Vec::new();
                    let mut total_distance = 0.0;
                    let mut total_toll_charge = 0.0;

                    for trip in &trips {
                        if let Some(trip_minutes) = parse_time_to_minutes(&trip.entry_time) {
                            let diff = (trip_minutes as i32 - centroid as i32).abs();
                            let dist = diff.min(1440 - diff); // Handle wrap-around for time

                            if dist <= 30 {
                                cluster_trips.push(*trip);

                                // Parse distance and toll charge
                                if let Ok(d) = trip.distance_km.trim().parse::<f64>() {
                                    total_distance += d;
                                }
                                if let Ok(c) = trip.toll_charge.trim().parse::<f64>() {
                                    total_toll_charge += c;
                                }

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
                    }

                    let mut average_entry_time = "N/A".to_string();
                    let mut total_toll_charge_previous_timeslot = 0.0;
                    let mut total_toll_charge_next_timeslot = 0.0;

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
                            if let (Some(avg_idx), Some(count)) = (
                                trip.get_timeslot_index_for_time(&average_entry_time),
                                trip.get_timeslot_count(),
                            ) {
                                let prev_idx = if avg_idx == 0 { count - 1 } else { avg_idx - 1 };
                                let next_idx = if avg_idx == count - 1 { 0 } else { avg_idx + 1 };

                                if let Some((prev_cost, _)) =
                                    trip.calculate_cost_at_timeslot(prev_idx)
                                {
                                    total_toll_charge_previous_timeslot += prev_cost;
                                }
                                if let Some((next_cost, _)) =
                                    trip.calculate_cost_at_timeslot(next_idx)
                                {
                                    total_toll_charge_next_timeslot += next_cost;
                                }
                            }
                        }
                    }

                    let centroid_data = CentroidData {
                        centroid_time: centroid_time_str,
                        trips: cluster_trips,
                        average_entry_time,
                        total_distance,
                        total_toll_charge,
                        total_toll_charge_previous_timeslot,
                        total_toll_charge_next_timeslot,
                    };
                    centroid_data_list.push(centroid_data);
                }

                summaries.push(TransponderSummaryByTime {
                    transponder_plate: plate,
                    direction,
                    centroids: centroid_data_list,
                    best_k,
                    formatted_centroids,
                });
            }
        }
    }

    summaries
}

mod csv_parser;

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

    let trips_by_transponder_direction = csv_parser::parse_trips(all_lines);

    let mut results: Vec<((String, Direction), Vec<&TripRecord>)> = Vec::new();
    for ((plate, direction), trips) in &trips_by_transponder_direction {
        let trip_refs: Vec<&TripRecord> = trips.iter().collect();
        results.push(((plate.clone(), direction.clone()), trip_refs));
    }

    // Sort results by plate and direction for consistent output
    results.sort_by(|a, b| {
        a.0.0
            .cmp(&b.0.0)
            .then_with(|| format!("{:?}", a.0.1).cmp(&format!("{:?}", b.0.1)))
    });

    let summaries = analyze_trips(results.clone());

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
            for trip in &centroid_data.trips {
                let day_type_str = match &trip.day_type {
                    Some(DayType::Holiday) => "Holiday",
                    Some(DayType::Weekend) => "Weekend",
                    Some(DayType::Weekday) => "Weekday",
                    None => "Unknown",
                };

                let mut optimization_msg = String::new();
                if let (Some(avg_idx), Some(count)) = (
                    trip.get_timeslot_index_for_time(&centroid_data.average_entry_time),
                    trip.get_timeslot_count(),
                ) {
                    let prev_idx = if avg_idx == 0 { count - 1 } else { avg_idx - 1 };
                    let next_idx = if avg_idx == count - 1 { 0 } else { avg_idx + 1 };

                    let current_cost = trip.toll_charge.parse::<f64>().unwrap_or(f64::MAX);

                    if let Some((prev_cost, _)) = trip.calculate_cost_at_timeslot(prev_idx) {
                        if prev_cost < current_cost - 0.005 {
                            optimization_msg
                                .push_str(&format!(" [Cheaper Prev: ${:.2}]", prev_cost));
                        }
                    }
                    if let Some((next_cost, _)) = trip.calculate_cost_at_timeslot(next_idx) {
                        if next_cost < current_cost - 0.005 {
                            optimization_msg
                                .push_str(&format!(" [Cheaper Next: ${:.2}]", next_cost));
                        }
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
        println!(
            "Transponder: {}, Direction: {:?}, Total Trips: {}",
            plate,
            direction,
            trips.len()
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
