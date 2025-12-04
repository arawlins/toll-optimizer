use simple_datetime_rs::Date;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

mod heavy_multiple_unit;
mod heavy_single_unit;
mod light_vehicles;
mod medium_vehicles;
mod motorcycles;

const OLD_ACCESSS_POINTS: [&str; 9] = [
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

const ACCESS_POINT_SYNONYMS: [(&str, &str); 3] = [
    ("Brock", "Brock(Hwy7)"),
    ("Brock407", "Brock(Hwy7)"),
    ("YorkDur", "York-DurhamLine"),
];

const ACCESS_POINTS: [&str; 41] = [
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
    4.659, // Dixie-Bramalea (calc)
    3.178, // Bramalea-Airport
    4.621, // Airport-Goreway (calc)
    3.235, // Goreway-Hwy427
    1.324, // Hwy427-Hwy27 (calc)
    4.061, // Hwy27-PineValley
    2.170, // PineValley-Weston
    0.0,   // Weston-Hwy400
    0.0,   // Hwy400-Jane
    2.199, // Jane-Keele
    3.521, // Keele-Dufferin
    2.194, // Dufferin-Bathurst
    2.172, // Bathurst-Yonge
    1.930, // Yonge-Bayview
    2.076, // Bayview-Leslie
    0.997, // Leslie-Hwy404 (calc)
    1.027, // Hwy404-Woodbine (calc)
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
enum Direction {
    Eastbound,
    Westbound,
}

#[derive(Debug, Clone, PartialEq)]
enum DayType {
    Weekday,
    Weekend,
    Holiday,
}

#[derive(Debug)]
struct TripRecord {
    transponder_plate: String,
    vehicle_class: String,
    date_of_trip: String,
    entry_time: String,
    entry_point: String,
    exit_point: String,
    distance_km: String,
    toll_charge: String,
    trip_toll_charge: String,
    camera_charge: String,
    direction: Option<Direction>,
    day_type: Option<DayType>,
}

impl TripRecord {
    fn from_csv_line(line: &str) -> Option<Self> {
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
    fn get_timeslot_index(&self) -> Option<usize> {
        let entry_minutes = parse_time_to_minutes(&self.entry_time)?;
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
    fn calculate_cost(&self) -> Option<f64> {
        let timeslot_idx = self.get_timeslot_index()?;
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

        match direction {
            Direction::Eastbound => {
                if start_idx >= end_idx {
                    return None;
                } // Invalid for Eastbound
                // Segments are from start_idx to end_idx - 1
                for i in start_idx..end_idx {
                    let distance = ACCESS_POINT_DISTANCES[i] as f64;

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
                    // println!(
                    //     "Index: {}\nDistance: {}\nAccess point: {}\nZone: {}\nPrice rate: {}\n",
                    //     i, distance, ap_name, zone, price_rate
                    // );
                    total_cost += distance * price_rate;
                }
            }
            Direction::Westbound => {
                if start_idx <= end_idx {
                    return None;
                } // Invalid for Westbound
                // Segments are from end_idx to start_idx - 1 (traversed in reverse)
                // For WB, if we go from index 10 to 5.
                // We traverse segments 9, 8, 7, 6, 5.
                // i goes from end_idx to start_idx - 1.
                for i in end_idx..start_idx {
                    let distance = ACCESS_POINT_DISTANCES[i] as f64;

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

        Some(total_cost / 100.0) // Convert cents to dollars
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

fn k_means_1d(data: &[u32], k: usize) -> (Vec<u32>, f64) {
    if data.is_empty() {
        return (Vec::new(), 0.0);
    }
    if k == 0 {
        return (Vec::new(), 0.0);
    }

    let min = *data.iter().min().unwrap();
    let max = *data.iter().max().unwrap();

    // Initialize centroids evenly spaced between min and max
    let mut centroids: Vec<f64> = (0..k)
        .map(|i| min as f64 + (max - min) as f64 * (i as f64 / (k.max(2) - 1) as f64))
        .collect();

    if k == 1 {
        centroids = vec![data.iter().sum::<u32>() as f64 / data.len() as f64];
    } else if k > 1 && min == max {
        return (vec![min], 0.0);
    }

    let mut final_clusters: Vec<Vec<u32>> = vec![vec![]; k];

    for _ in 0..100 {
        // Max iterations
        let mut clusters: Vec<Vec<u32>> = vec![vec![]; k];

        // Assign points to clusters
        for &point in data {
            let mut best_cluster = 0;
            let mut min_dist = f64::MAX;

            for (i, &centroid) in centroids.iter().enumerate() {
                let dist = (point as f64 - centroid).abs();
                if dist < min_dist {
                    min_dist = dist;
                    best_cluster = i;
                }
            }
            clusters[best_cluster].push(point);
        }

        // Update centroids
        let mut new_centroids = Vec::new();
        let mut changed = false;

        for (i, cluster) in clusters.iter().enumerate() {
            if cluster.is_empty() {
                // If a cluster is empty, keep the old centroid (or could re-initialize)
                new_centroids.push(centroids[i]);
            } else {
                let mean = cluster.iter().sum::<u32>() as f64 / cluster.len() as f64;
                if (mean - centroids[i]).abs() > 0.1 {
                    changed = true;
                }
                new_centroids.push(mean);
            }
        }

        centroids = new_centroids;
        final_clusters = clusters;
        if !changed {
            break;
        }
    }

    let mut result: Vec<u32> = centroids.iter().map(|&c| c.round() as u32).collect();
    result.sort();

    // Calculate WCSS
    let mut wcss = 0.0;
    for (i, cluster) in final_clusters.iter().enumerate() {
        let centroid = centroids[i];
        for &point in cluster {
            wcss += (point as f64 - centroid).powi(2);
        }
    }

    (result, wcss)
}

fn find_best_k(wcss_values: &[f64]) -> usize {
    if wcss_values.len() < 3 {
        return wcss_values.len();
    }

    let n = wcss_values.len();
    let p1 = (1.0, wcss_values[0]);
    let p2 = (n as f64, wcss_values[n - 1]);

    let mut max_dist = -1.0;
    let mut best_k = 1;

    for i in 0..n {
        let k = (i + 1) as f64;
        let wcss = wcss_values[i];

        // Perpendicular distance from point (k, wcss) to line p1-p2
        // Line equation: Ax + By + C = 0
        // (y2 - y1)x - (x2 - x1)y + x2y1 - y2x1 = 0

        let numerator =
            ((p2.1 - p1.1) * k - (p2.0 - p1.0) * wcss + p2.0 * p1.1 - p2.1 * p1.0).abs();
        let denominator = ((p2.1 - p1.1).powi(2) + (p2.0 - p1.0).powi(2)).sqrt();

        let dist = numerator / denominator;

        if dist > max_dist {
            max_dist = dist;
            best_k = i + 1;
        }
    }

    best_k
}

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

    let mut trips_by_transponder: HashMap<String, Vec<TripRecord>> = HashMap::new();

    for path in entries {
        let file = fs::File::open(&path)?;
        let reader = io::BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        // We need at least 5 lines to have a header (line 5 is index 4)
        if lines.len() < 5 {
            continue;
        }

        // Skip header (line 5), print the rest
        if lines.len() > 5 {
            for line in &lines[5..] {
                if let Some(mut record) = TripRecord::from_csv_line(line) {
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
                        // println!("Entry: {}, Exit: {}, Direction: {:?}", record.entry_point, record.exit_point, record.direction.as_ref().unwrap());

                        let plate = record.transponder_plate.clone();
                        trips_by_transponder.entry(plate).or_default().push(record);
                    } else {
                        // If we can't find the points (shouldn't happen due to previous checks, but good for safety)
                        if entry_index.is_none() {
                            println!(
                                "UNKNOWN ENTRY POINT {}: {}",
                                record.date_of_trip, record.entry_point
                            );
                        }

                        if exit_index.is_none() {
                            println!(
                                "UNKNOWN EXIT POINT {}: {}",
                                record.date_of_trip, record.exit_point
                            );
                        }
                    }
                }
            }
        }
    }

    let mut trips_by_transponder_direction: HashMap<(String, Direction), Vec<&TripRecord>> =
        HashMap::new();

    for (plate, trips) in &trips_by_transponder {
        for trip in trips {
            if let Some(direction) = &trip.direction {
                trips_by_transponder_direction
                    .entry((plate.clone(), direction.clone()))
                    .or_default()
                    .push(trip);
            }
        }
    }

    let mut results: Vec<_> = trips_by_transponder_direction.into_iter().collect();
    results.sort_by(|a, b| {
        a.0.0
            .cmp(&b.0.0)
            .then_with(|| format!("{:?}", a.0.1).cmp(&format!("{:?}", b.0.1)))
    });

    for ((plate, direction), trips) in results {
        let mut time_counts: HashMap<String, usize> = HashMap::new();
        for trip in &trips {
            *time_counts.entry(trip.entry_time.clone()).or_default() += 1;
        }

        let mut sorted_times: Vec<_> = time_counts.into_iter().collect();
        // Sort by count (descending), then by time (ascending) for stability
        sorted_times.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        if let Some((most_common_time, count)) = sorted_times.first() {
            println!(
                "Transponder: {}, Direction: {:?}, Most Common Entry Time: {} ({} times)",
                plate, direction, most_common_time, count
            );
        }

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
                println!(
                    "  Best k={} (Elbow Method): [{}]",
                    best_k,
                    formatted_centroids.join(", ")
                );

                for &centroid in best_centroids {
                    println!("    Trips near {}:", format_minutes_to_time(centroid));
                    let mut cluster_trip_minutes = Vec::new();

                    for trip in &trips {
                        if let Some(trip_minutes) = parse_time_to_minutes(&trip.entry_time) {
                            let diff = (trip_minutes as i32 - centroid as i32).abs();
                            let dist = diff.min(1440 - diff); // Handle wrap-around for time

                            if dist <= 30 {
                                let day_type_str = match &trip.day_type {
                                    Some(DayType::Holiday) => "Holiday",
                                    Some(DayType::Weekend) => "Weekend",
                                    Some(DayType::Weekday) => "Weekday",
                                    None => "Unknown",
                                };
                                let timeslot_idx = trip
                                    .get_timeslot_index()
                                    .map(|i| i.to_string())
                                    .unwrap_or("?".to_string());
                                let calculated_cost = trip
                                    .calculate_cost()
                                    .map(|c| format!("{:.2}", c))
                                    .unwrap_or("?".to_string());
                                println!(
                                    "      - {} {} ({} -> {}) [{}] [Calc: ${}] [Actual: ${}]",
                                    trip.date_of_trip,
                                    trip.entry_time,
                                    trip.entry_point,
                                    trip.exit_point,
                                    day_type_str,
                                    calculated_cost,
                                    trip.toll_charge
                                );

                                // Normalize trip minutes relative to centroid for averaging
                                // If trip is e.g. 00:10 (10) and centroid is 23:50 (1430),
                                // we want to treat 00:10 as 24:10 (1450) if it's "after" the centroid in a circular sense
                                // simpler: just use the signed difference from centroid

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

                        println!(
                            "      Average Entry Time: {}",
                            format_minutes_to_time(normalized_avg as u32)
                        );
                    }
                }
            }
        }
    }

    println!("\nTotal Trips per Transponder:");
    for (plate, trips) in &trips_by_transponder {
        println!("Transponder: {}, Total Trips: {}", plate, trips.len());
    }

    Ok(())
}
