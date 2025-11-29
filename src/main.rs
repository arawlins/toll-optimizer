use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

const OLD_ACCESSS_POINTS: [&str; 9] = 
[
    "LakeRidg",
    "LakeRidge",
    "Baldwin",
    "Thickson",
    "Simcoe",
    "Hwy412",
    "35/115",
    "Hwy35/115",
    "Hwy418"
];

const ACCESS_POINT_SYNONYMS: [(&str, &str); 3] =
[
    ("Brock", "Brock(Hwy7)"),
    ("Brock407", "Brock(Hwy7)"),
    ("YorkDur", "York-DurhamLine")
];

const ACCESS_POINTS: [&str; 41] = 
[
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
    "Brock(Hwy7)"
];

const EB_ZONES: [(&str, u8); 41] =
[
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

const WB_ZONES: [(&str, u8); 41] =
[
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
}

impl TripRecord {
    fn from_csv_line(line: &str) -> Option<Self> {
        // The format is "Val","Val",...
        // We split by "," to get the inner values.
        // We also need to trim the quotes from the start and end of the line if they exist,
        // but the split method below handles the internal quotes.
        // A robust CSV parser would be better, but we are sticking to the simple logic for now.
        
        let parts: Vec<&str> = line.split("\",\"").collect();
        if parts.len() < 10 {
            return None;
        }

        // Clean up the first and last element which might have leading/trailing quote
        let first = parts[0].trim_start_matches('"');
        let last = parts[parts.len() - 1].trim_end_matches('"');

        Some(TripRecord {
            transponder_plate: first.to_string(),
            vehicle_class: parts[1].to_string(),
            date_of_trip: parts[2].to_string(),
            entry_time: parts[3].to_string(),
            entry_point: parts[4].to_string(),
            exit_point: parts[5].to_string(),
            distance_km: parts[6].to_string(),
            toll_charge: parts[7].to_string(),
            trip_toll_charge: parts[8].to_string(),
            camera_charge: last.to_string(),
        })
    }
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
        .filter(|path| path.extension().map_or(false, |ext| ext == "csv"))
        .collect();

    entries.sort();

    let mut first = true;

    for path in entries {
        let file = fs::File::open(&path)?;
        let reader = io::BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        // We need at least 5 lines to have a header (line 5 is index 4)
        if lines.len() < 5 {
            continue;
        }

        if first {
            // Print header (line 5) and the rest
            // for line in &lines[4..] {
            //     println!("{}", line);
            // }
            first = false;
        } else {
            // Skip header (line 5), print the rest
            if lines.len() > 5 {
                for line in &lines[5..] {
                    if let Some(record) = TripRecord::from_csv_line(line) {
                        if OLD_ACCESSS_POINTS.contains(&record.entry_point.as_str()) || OLD_ACCESSS_POINTS.contains(&record.exit_point.as_str()) {
                            continue;
                        }

                        if !ACCESS_POINTS.contains(&record.entry_point.as_str()) {
                            println!("{}: {}", record.date_of_trip, record.entry_point);
                        }
                        
                        if !ACCESS_POINTS.contains(&record.exit_point.as_str()) {
                            println!("{}: {}", record.date_of_trip, record.exit_point);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
