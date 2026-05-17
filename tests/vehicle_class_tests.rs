use toll_optimizer::{DayType, Direction, VehicleClass, calculate_single_trip_cost};

#[test]
fn test_medium_vehicle_cost() {
    let result = calculate_single_trip_cost(
        "QEW",
        "Trafalgar",
        "2026-05-12",
        "08:00 AM",
        VehicleClass::MediumVehicle,
    );

    assert!(result.is_ok());
    let (cost, dist, direction, day_type) = result.unwrap();
    assert!(cost > 0.0);
    assert!(dist > 0.0);
    assert_eq!(direction, Direction::Eastbound);
    assert_eq!(day_type, DayType::Weekday);

    // Compare with Light Vehicle
    let light_result = calculate_single_trip_cost(
        "QEW",
        "Trafalgar",
        "2026-05-12",
        "08:00 AM",
        VehicleClass::LightVehicle,
    )
    .unwrap();

    assert!(
        cost > light_result.0,
        "Medium vehicle should be more expensive than light vehicle"
    );
}

#[test]
fn test_motorcycle_cost() {
    let result = calculate_single_trip_cost(
        "QEW",
        "Trafalgar",
        "2026-05-12",
        "08:00 AM",
        VehicleClass::Motorcycle,
    );

    assert!(result.is_ok());
    let (cost, dist, direction, day_type) = result.unwrap();
    assert!(cost > 0.0);
    assert!(dist > 0.0);
    assert_eq!(direction, Direction::Eastbound);
    assert_eq!(day_type, DayType::Weekday);

    // Motorcycle in 407 ETR is usually cheaper than light vehicle.
    let light_result = calculate_single_trip_cost(
        "QEW",
        "Trafalgar",
        "2026-05-12",
        "08:00 AM",
        VehicleClass::LightVehicle,
    )
    .unwrap();

    assert!(
        cost < light_result.0,
        "Motorcycle should be cheaper than light vehicle on 407 ETR"
    );
}

#[test]
fn test_heavy_single_unit_cost() {
    let result = calculate_single_trip_cost(
        "QEW",
        "Trafalgar",
        "2026-05-12",
        "08:00 AM",
        VehicleClass::HeavySingleUnit,
    );

    assert!(result.is_ok());
    let (cost, dist, direction, day_type) = result.unwrap();
    assert!(cost > 0.0);
    assert!(dist > 0.0);
    assert_eq!(direction, Direction::Eastbound);
    assert_eq!(day_type, DayType::Weekday);

    // Compare with Medium Vehicle
    let medium_result = calculate_single_trip_cost(
        "QEW",
        "Trafalgar",
        "2026-05-12",
        "08:00 AM",
        VehicleClass::MediumVehicle,
    )
    .unwrap();

    assert!(
        cost > medium_result.0,
        "Heavy single unit should be more expensive than medium vehicle"
    );
}

#[test]
fn test_heavy_multiple_unit_cost() {
    let result = calculate_single_trip_cost(
        "QEW",
        "Trafalgar",
        "2026-05-12",
        "08:00 AM",
        VehicleClass::HeavyMultipleUnit,
    );

    assert!(result.is_ok());
    let (cost, dist, direction, day_type) = result.unwrap();
    assert!(cost > 0.0);
    assert!(dist > 0.0);
    assert_eq!(direction, Direction::Eastbound);
    assert_eq!(day_type, DayType::Weekday);

    // Compare with Heavy Single Unit
    let heavy_single_result = calculate_single_trip_cost(
        "QEW",
        "Trafalgar",
        "2026-05-12",
        "08:00 AM",
        VehicleClass::HeavySingleUnit,
    )
    .unwrap();

    assert!(
        cost > heavy_single_result.0,
        "Heavy multiple unit should be more expensive than heavy single unit"
    );
}

#[test]
fn test_heavy_vehicle_weekend_cost() {
    // 30 Aug 25 is a Saturday
    let result = calculate_single_trip_cost(
        "QEW",
        "Trafalgar",
        "2025-08-30",
        "10:00 AM",
        VehicleClass::HeavyMultipleUnit,
    )
    .unwrap();

    let weekday_result = calculate_single_trip_cost(
        "QEW",
        "Trafalgar",
        "2025-08-28", // Thursday
        "10:00 AM",
        VehicleClass::HeavyMultipleUnit,
    )
    .unwrap();

    assert!(
        weekday_result.0 > result.0,
        "Heavy multiple unit weekday cost should be higher than weekend cost"
    );
}

#[test]
fn test_motorcycle_holiday_cost() {
    // 01 Jan 26 is a Thursday, normally a weekday, but is a holiday.
    let result = calculate_single_trip_cost(
        "QEW",
        "Trafalgar",
        "2026-01-01",
        "10:00 AM",
        VehicleClass::Motorcycle,
    )
    .unwrap();

    let weekend_result = calculate_single_trip_cost(
        "QEW",
        "Trafalgar",
        "2026-01-04", // Sunday
        "10:00 AM",
        VehicleClass::Motorcycle,
    )
    .unwrap();

    assert_eq!(
        result.0, weekend_result.0,
        "Motorcycle holiday cost should match weekend cost"
    );
}

#[test]
fn test_vehicle_class_from_str() {
    assert_eq!(
        "Light vehicle".parse::<VehicleClass>().ok(),
        Some(VehicleClass::LightVehicle)
    );
    assert_eq!(
        "Medium Vehicle".parse::<VehicleClass>().ok(),
        Some(VehicleClass::MediumVehicle)
    );
    assert_eq!(
        "Heavy Single Unit".parse::<VehicleClass>().ok(),
        Some(VehicleClass::HeavySingleUnit)
    );
    assert_eq!(
        "Heavy Multiple Unit".parse::<VehicleClass>().ok(),
        Some(VehicleClass::HeavyMultipleUnit)
    );
    assert_eq!(
        "Motorcycle".parse::<VehicleClass>().ok(),
        Some(VehicleClass::Motorcycle)
    );
    assert!("Unknown".parse::<VehicleClass>().is_err());
}
