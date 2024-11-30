use pyo3::prelude::*;

#[pymodule]
mod _lib {
    use pyo3::prelude::*;
    use csv::ReaderBuilder;
    use std::fs::File;

    #[pyfunction]
    fn read_csv(file_path: &str, delimiter: char, has_headers: bool) -> PyResult<(Option<Vec<String>>, Vec<Vec<String>>)> {
        let file = File::open(file_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to open file: {}", e)))?;

        let mut rdr = ReaderBuilder::new()
            .delimiter(delimiter as u8) // Convert char to u8
            .has_headers(has_headers)  // Ensure the first row is treated as headers
            .from_reader(file);

        let mut headers = None;

        if has_headers {
            let csv_headers = rdr.headers()
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to read headers: {}", e)))?

                .iter()
                .map(String::from)
                .collect::<Vec<String>>();

            headers = Some(csv_headers);
        }

        let mut records = Vec::new();

        // Read the records
        for result in rdr.records() {
            let record = result
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to read record: {}", e)))?;
            records.push(record.iter().map(String::from).collect());
        }

        // Return a tuple (headers, records)
        Ok((headers, records))
    }

}