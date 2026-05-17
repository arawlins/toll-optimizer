//! Raw toll rate tables by vehicle class.
//!
//! Each submodule contains cents-per-kilometer tables indexed by timeslot and
//! zone. Most consumers should use `VehicleClass::get_rate` instead of reading
//! these arrays directly.

/// Heavy multiple-unit vehicle rate tables.
pub mod heavy_multiple_unit;
/// Heavy single-unit vehicle rate tables.
pub mod heavy_single_unit;
/// Light vehicle rate tables.
pub mod light_vehicles;
/// Medium vehicle rate tables.
pub mod medium_vehicles;
/// Motorcycle rate tables.
pub mod motorcycles;
