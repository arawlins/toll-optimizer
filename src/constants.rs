//! Static 407 ETR topology, timeslot, and zone data.
//!
//! Access points are ordered west-to-east. Distance entries represent the
//! segment between `ACCESS_POINTS[i]` and `ACCESS_POINTS[i + 1]`.

/// Historical or removed access-point names that should be skipped during CSV parsing.
pub const OLD_ACCESS_POINTS: [&str; 9] = [
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

/// Weekday pricing timeslot start times for 2026 and later.
pub const WEEKDAY_TIMESLOTS_2026: [&str; 8] = [
    "5:00 AM", "7:00 AM", "9:30 AM", "10:30 AM", "2:30 PM", "3:30 PM", "6:00 PM", "9:00 PM",
];
/// Minute offsets after midnight for 2026 weekday timeslots.
pub const WEEKDAY_TIMESLOT_MINUTES_2026: [u32; 8] = [300, 420, 570, 630, 870, 930, 1080, 1260];

/// Weekday pricing timeslot start times for 2025 statements.
pub const WEEKDAY_TIMESLOTS_2025: [&str; 9] = [
    "12:00 AM", "6:00 AM", "7:00 AM", "9:30 AM", "10:30 AM", "2:30 PM", "3:30 PM", "6:00 PM",
    "7:00 PM",
];
/// Minute offsets after midnight for 2025 weekday timeslots.
pub const WEEKDAY_TIMESLOT_MINUTES_2025: [u32; 9] = [0, 360, 420, 570, 630, 870, 930, 1080, 1140];

/// Weekend and holiday pricing timeslot start times for 2026 and later.
pub const WEEKEND_TIMESLOTS_2026: [&str; 4] = ["8:30 AM", "10:00 AM", "7:00 PM", "9:00 PM"];
/// Minute offsets after midnight for 2026 weekend/holiday timeslots.
pub const WEEKEND_TIMESLOT_MINUTES_2026: [u32; 4] = [510, 600, 1140, 1260];

/// Weekend and holiday pricing timeslot start times for 2025 statements.
pub const WEEKEND_TIMESLOTS_2025: [&str; 5] =
    ["12:00 AM", "8:30 AM", "10:00 AM", "7:00 PM", "9:00 PM"];
/// Minute offsets after midnight for 2025 weekend/holiday timeslots.
pub const WEEKEND_TIMESLOT_MINUTES_2025: [u32; 5] = [0, 510, 600, 1140, 1260];

/// Input aliases normalized to canonical `ACCESS_POINTS` names.
pub const ACCESS_POINT_SYNONYMS: [(&str, &str); 3] = [
    ("Brock", "Brock(Hwy7)"),
    ("Brock407", "Brock(Hwy7)"),
    ("YorkDur", "York-DurhamLine"),
];

/// Canonical 407 ETR access points ordered from west to east.
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

/// Segment distances in kilometers between adjacent access points.
pub const ACCESS_POINT_DISTANCES: [f32; 40] = [
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

/// Eastbound zone number for each canonical access point.
pub const EB_ZONES: [(&str, u8); 41] = [
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

/// Westbound zone number for each canonical access point.
pub const WB_ZONES: [(&str, u8); 41] = [
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

/// Observed 407 ETR statutory holidays from 2024 to 2028: `(year, month, day)`.
///
/// Sorted chronologically for binary search.
pub const STATUTORY_HOLIDAYS: [(u32, u32, u32); 45] = [
    (2024, 1, 1),
    (2024, 2, 19),
    (2024, 3, 29),
    (2024, 5, 20),
    (2024, 7, 1),
    (2024, 9, 2),
    (2024, 10, 14),
    (2024, 12, 25),
    (2024, 12, 26),
    (2025, 1, 1),
    (2025, 2, 17),
    (2025, 4, 18),
    (2025, 5, 19),
    (2025, 7, 1),
    (2025, 9, 1),
    (2025, 10, 13),
    (2025, 12, 25),
    (2025, 12, 26),
    (2026, 1, 1),
    (2026, 2, 16),
    (2026, 4, 3),
    (2026, 5, 18),
    (2026, 7, 1),
    (2026, 9, 7),
    (2026, 10, 12),
    (2026, 12, 25),
    (2026, 12, 26),
    (2027, 1, 1),
    (2027, 2, 15),
    (2027, 3, 26),
    (2027, 5, 24),
    (2027, 7, 1),
    (2027, 9, 6),
    (2027, 10, 11),
    (2027, 12, 25),
    (2027, 12, 26),
    (2028, 1, 1),
    (2028, 2, 21),
    (2028, 4, 14),
    (2028, 5, 22),
    (2028, 7, 1),
    (2028, 9, 4),
    (2028, 10, 9),
    (2028, 12, 25),
    (2028, 12, 26),
];
