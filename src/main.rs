///// Imports
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;
use std::time::Instant;
use serde_json::Value;
use csv::Reader;
use serde_yaml;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Create folder 'converted'
	 fs::create_dir_all("converted")?;

	 // Receive file name from user
	 println!("Введите имя файла для конвертации (CSV/YAML):");
	 let mut input_filename = String::new();
	 io::stdin().read_line(&mut input_filename)?;
	 let input_path = Path::new(input_filename.trim());
	 

	 // Define type of file
	 let input_type = determine_input_type(input_path);

	 // Read and convert file CSV or YAML
	 let json_value: Value = match input_type.as_str() {
		"csv" => convert_csv_to_json(input_path)?,
		"yaml" => convert_yaml_to_json(input_path)?,
		_ => return Err("Неподдерживаемый на данный момент тип файла" .into()),
	 };

	 // Create name of output file
	 let output_filename = format!("converted/{}.json", Path::new(&input_filename).file_stem().unwrap().to_str().unwrap());

	 let start_time = Instant::now();

	 // Write JSON to file
	 fs::write(&output_filename, serde_json::to_string_pretty(&json_value)?)?;

	 let duration = start_time.elapsed();
	 println!("Файл успешно конвертирован и сохранен как {}", output_filename);
	 println!("Конвертация файла завершена за: {:.1?}", duration);

	 Ok(())
}

fn determine_input_type(path: &Path) -> String {
	path.extension()
	    .and_then(OsStr::to_str)
		 .map(|ext| ext.to_lowercase())
		 .map(|ext| match ext.as_str() {
			  "csv" => "csv",
			  "yaml" | "yml" => "yaml",
			  _ => "unknown",
		 })
		 .unwrap_or("unknown")
		 .to_string()
}


fn convert_csv_to_json(path: &Path) -> std::result::Result<Value, Box<dyn std::error::Error>> {
	let mut reader = Reader::from_path(path)?;
	let headers = reader.headers()?.clone();
	let records: std::result::Result<Vec<_>,_> = reader.records().collect();
	let records = records?;

	// Create JSON how array of objects because CSV has tables format //
	let json_array: Vec<serde_json::Map<String, Value>> = records
	    .iter()
		 .map(|record| {
			headers
				.iter()
				.zip(record.iter())
				.map(|(header, field)| (header.to_string(), Value::String(field.to_string())))
				.collect()
		 })

		 .collect();

		Ok(Value::Array(json_array.into_iter().map(Value::Object).collect()))
}

fn convert_yaml_to_json(path: &Path) -> std::result::Result<Value, Box<dyn std::error::Error>> {
	let mut file = File::open(path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;

	let yaml_value: Value = serde_yaml::from_str(&contents)?;
	Ok(yaml_value)
}