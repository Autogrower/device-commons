use crate::reader;
use csv::Reader;
pub use reader::DataReader;
use reader::{DataReturn, MeasurementEntry};
#[cfg(not(test))]
use std::fs::OpenOptions;
use std::{fs::File, path::PathBuf};

pub struct CSVReader {
    reader: Reader<std::fs::File>,
}

#[cfg(not(test))]
fn create_csv(csv: PathBuf) -> File {
    OpenOptions::new()
        .write(true)
        .append(true)
        .read(true)
        .create(true)
        .open(csv)
        .unwrap()
}

impl CSVReader {
    pub fn new(csv: PathBuf) -> CSVReader {
        let fd = create_csv(csv);
        let rdr: Reader<std::fs::File> = csv::Reader::from_reader(fd);

        CSVReader { reader: rdr }
    }
}

impl DataReader for CSVReader {
    fn read_all_measurements(&mut self) -> DataReturn {
        let mut data: Vec<MeasurementEntry> = vec![];

        for result in self.reader.deserialize() {
            let entry: MeasurementEntry = match result {
                Ok(entry) => entry,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };

            data.push(entry);
        }

        Ok(data)
    }

    fn read_latest(&mut self) -> DataReturn {
        let mut entries: Vec<MeasurementEntry> = self.read_all_measurements()?;

        entries.sort_by_key(|e| 2147483648 - e.timestamp);

        let entry: Option<&MeasurementEntry> = entries.first();
        let mut data: Vec<MeasurementEntry> = vec![];

        if let Some(entry) = entry {
            data.push(*entry);
        };

        Ok(data)
    }
}

#[cfg(test)]
fn create_csv(_csv: PathBuf) -> File {
    use std::io::{Seek, Write};

    let mut fd = tempfile::tempfile().unwrap();

    let test_data = tests::mock::measurement_entry();

    fd.write_fmt(format_args!(
        "{}\n{}\n{}\n{}",
        "timestamp,light_intensity,acidity,temperature,electrical_conductivity",
        test_data[0],
        test_data[1],
        test_data[2],
    ))
    .unwrap();

    fd.rewind().unwrap();

    fd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_csv_reader() -> Result<(), Box<dyn std::error::Error>> {
        CSVReader::new("test_file".into());

        Ok(())
    }

    #[test]
    fn test_csv_read_all() -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = CSVReader::new("test_file".into());

        let res = rdr.read_all_measurements().unwrap();

        assert_eq!(res, mock::measurement_entry());

        Ok(())
    }

    #[test]
    fn test_read_latest() -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = CSVReader::new("test_file".into());

        let res = rdr.read_latest().unwrap();

        assert_eq!(res[0], mock::measurement_entry()[1]);

        Ok(())
    }

    pub mod mock {
        use crate::reader::MeasurementEntry;

        pub fn measurement_entry() -> Vec<MeasurementEntry> {
            vec![
                MeasurementEntry {
                    timestamp: 2,
                    light_intensity: 1,
                    acidity: 1,
                    temperature: 1,
                    electrical_conductivity: 1,
                },
                MeasurementEntry {
                    timestamp: 43,
                    light_intensity: 1,
                    acidity: 1,
                    temperature: 1,
                    electrical_conductivity: 1,
                },
                MeasurementEntry {
                    timestamp: 23,
                    light_intensity: 1,
                    acidity: 1,
                    temperature: 1,
                    electrical_conductivity: 1,
                },
            ]
        }
    }
}
