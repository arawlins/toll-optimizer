use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

const OLD_ACCESSS_POINTS: [&str] = 
[
    "LakeRidg",
    "Baldwin",
    "Thickson",
    "Simcoe",
    "Hwy412",
    "35/115",
    "Hwy35/115",
    "Hwy418"
];

const ACCESS_POINT_SYNONYMS: [(&str, &str)] =
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
                    // println!("{}", line);
                    
                    // Parse the line
                    // The format is "Val","Val",...
                    // We split by "," to get the inner values.
                    // Note: This simple splitting assumes no "," inside the values.
                    // Based on the file content, this seems safe for now.
                    // Index 4 is Entry Point, Index 5 is Exit Point.
                    // Index 2 is Date of Trip
                    
                    let parts: Vec<&str> = line.split("\",\"").collect();
                    if parts.len() > 5 {
                        let entry_point = parts[4];
                        let exit_point = parts[5];
                        let date = parts[2];
                        
                        if !ACCESS_POINTS.contains(&entry_point) {
                            println!("{}: {}", date, entry_point);
                        }
                        
                        if !ACCESS_POINTS.contains(&exit_point) {
                            println!("{}: {}", date, exit_point);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
