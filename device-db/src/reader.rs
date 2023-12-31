use std::fmt;

#[derive(Debug, serde::Deserialize, Clone, Copy, PartialEq)]
pub struct MeasurementEntry {
    pub timestamp: u128,
    pub light_intensity: u32,
    pub acidity: u32,
    pub temperature: u32,
    pub electrical_conductivity: u32,
}

impl fmt::Display for MeasurementEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{}",
            self.timestamp,
            self.light_intensity,
            self.acidity,
            self.temperature,
            self.electrical_conductivity
        )
    }
}

/// This type is the return type of every data loading function of [DataReader].
pub type DataReturn = Result<Vec<MeasurementEntry>, Box<dyn std::error::Error>>;

pub trait DataReader {
    // fn new() -> Result<Self, Box<dyn std::error::Error>>
    // where
    //     Self: Sized;

    /// Reads all stored measurements
    fn read_all_measurements(&mut self) -> DataReturn;

    /// Reads only the most recent measurement
    fn read_latest(&mut self) -> DataReturn;

    // /// Reads all entries since a given timestamp and returns them
    // fn read_since(date: u128) -> Vec<MeasurementEntry>;

    // /// Reads all measurements that have been added since the last reading
    // fn read_new() -> Vec<MeasurementEntry>;
}
