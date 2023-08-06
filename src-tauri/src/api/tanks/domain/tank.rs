pub struct Tank{
    id: String,
    name: String,
    available_volume: f32,
    current_volume: f32,
    previous_volume: f32,
    gas_type: GasType,
    vessel_id: String,
}

impl Tank