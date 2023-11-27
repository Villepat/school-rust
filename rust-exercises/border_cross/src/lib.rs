// Define the Car struct with a lifetime annotation 'a
#[allow(dead_code)]
pub struct Car<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
}

// Define the Truck struct with a lifetime annotation 'a
#[allow(dead_code)]
pub struct Truck<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
    pub load_tons: u32,
}

// Define the Vehicle trait
pub trait Vehicle {
    fn model(&self) -> &str;
    fn year(&self) -> u32;
}

// Implement Vehicle trait for Truck
impl<'a> Vehicle for Truck<'a> {
    fn model(&self) -> &str {
        self.model
    }
    fn year(&self) -> u32 {
        self.year
    }
}

// Implement Vehicle trait for Car
impl<'a> Vehicle for Car<'a> {
    fn model(&self) -> &str {
        self.model
    }
    fn year(&self) -> u32 {
        self.year
    }
}

// Function to return all models from a list of Vehicles
pub fn all_models(list: Vec<&dyn Vehicle>) -> Vec<&str> {
    let mut models = Vec::new();
    for vehicle in list {
        models.push(vehicle.model());
    }
    models
}
