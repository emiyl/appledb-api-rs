mod file;
mod json;
mod os_file;
use walkdir::WalkDir;

fn main() {
    let now = std::time::Instant::now();
    let mut file_count = 0;
    //let mut os_entry_vec = Vec::new();
    let mut os_entry_key_vec = Vec::new();

    for entry in WalkDir::new("./appledb/osFiles")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|entry| {
            if entry.path().extension().map_or(false, |ext| ext == "json") {
                Some(entry)
            } else {
                None
            }
        })
    {
        let path = entry.path().to_str().unwrap();
        let entry_vec = os_file::get_os_entry_vec_from_path(path);

        for entry in entry_vec {
            //os_entry_vec.push(entry.clone());
            os_entry_key_vec.push(entry.clone().key.replace(';', "/"));

            let out_json = serde_json::to_string(&entry).expect("Failed to convert struct to JSON");
            let output = ["./out/os/", &entry.key.replace(';', "/"), ".json"].concat();
            file::write_string_to_file(&output, out_json).expect("Failed to write JSON");

            file_count += 1;
        }
    }

    /*let main_json_string =
        serde_json::to_string(&os_entry_vec).expect("Failed to convert struct to JSON");
    file::write_string_to_file("./out/os/main.json", main_json_string)
    .expect("Failed to write JSON");*/
    let key_json_string =
        serde_json::to_string(&os_entry_key_vec).expect("Failed to convert struct to JSON");
    file::write_string_to_file("./out/os/index.json", key_json_string)
        .expect("Failed to write JSON");

    let elapsed = now.elapsed();
    println!("Processed {} files in {:.2?}", file_count, elapsed);
}
