mod file;
mod json;
mod os_file;
use std::path::Path;
use std::{fs, io::Write};
use walkdir::WalkDir;

fn main() {
    let now = std::time::Instant::now();
    let mut file_count = 0;
    //let mut os_entry_vec = Vec::new();
    //let mut os_entry_key_vec = Vec::new();

    fs::create_dir_all("./out/os").expect("Failed to create directory ./out/os");
    file::create_blank_file_and_overwrite("./out/os/main.json")
        .expect("Failed to create ./out/os/main.json");
    let mut main_json_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("./out/os/main.json")
        .unwrap();
    main_json_file
        .write("[".as_bytes())
        .expect("Failed to write to ./out/os/main.json");

    file::create_blank_file_and_overwrite("./out/os/index.json")
        .expect("Failed to create ./out/os/index.json");
    let mut index_json_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("./out/os/index.json")
        .unwrap();
    index_json_file
        .write("[".as_bytes())
        .expect("Failed to write to ./out/os/index.json");

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
            let out_json = serde_json::to_string(&entry).expect("Failed to convert struct to JSON");

            let output = ["./out/os/", &entry.key.replace(';', "/"), ".json"].concat();
            file::write_string_to_file(&output, &out_json).expect("Failed to write JSON");

            main_json_file
                .write([out_json.to_string(), ','.to_string()].concat().as_bytes())
                .expect("Failed to write to ./out/os/main.json");
            index_json_file
                .write(
                    [
                        entry
                            .clone()
                            .key
                            .replace(';', "/")
                            .replace(' ', "-")
                            .to_string(),
                        ','.to_string(),
                    ]
                    .concat()
                    .as_bytes(),
                )
                .expect("Failed to write to ./out/os/index.json");

            file_count += 1;
        }
    }

    main_json_file
        .write("]\n".as_bytes())
        .expect("Failed to write to ./out/os/main.json");
    index_json_file
        .write("]\n".as_bytes())
        .expect("Failed to write to ./out/os/index.json");

    let elapsed = now.elapsed();
    println!("Processed {} files in {:.2?}", file_count, elapsed);
}
