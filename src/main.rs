mod file;
mod json;
mod os_file;
use std::{fs, fs::File, io::Write, os::unix::fs::FileExt};
use walkdir::WalkDir;

fn create_firmware() -> u32 {
    let mut file_count: u32 = 0;
    fs::create_dir_all("./out/firmware").expect("Failed to create directory ./out/firmware");

    let main_index_json_path_array = ["./out/firmware/main.json", "./out/firmware/index.json"];
    let mut main_index_json_file_vec: Vec<File> = Vec::new();

    for path in main_index_json_path_array {
        file::write_string_to_file(path, &"[".to_string())
            .expect("Failed to write to main/index json file");
        main_index_json_file_vec.push(
            fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .unwrap(),
        )
    }
    let mut os_str_vec: Vec<String> = Vec::new();

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

            let output = ["./out/firmware/", &entry.key.replace(';', "/"), ".json"].concat();
            file::write_string_to_file(&output, &out_json).expect("Failed to write JSON");
            file_count += 1;

            let os_str = &entry.osStr;
            let os_str_main_index_json_file_path = [
                ["./out/firmware/", os_str, "/main.json"].concat(),
                ["./out/firmware/", os_str, "/index.json"].concat(),
            ];
            if !os_str_vec.contains(os_str) {
                os_str_vec.push(os_str.clone());
                for path in os_str_main_index_json_file_path.iter() {
                    if file::path_exists(path) {
                        fs::remove_file(path).expect("Failed to delete osStr main/index json file")
                    }
                }
            }

            for path in os_str_main_index_json_file_path.iter() {
                if !file::path_exists(path) {
                    file::write_string_to_file(path, &"[".to_string())
                        .expect("Failed to write to osStr main/index json file");
                }
            }

            let mut os_str_main_index_json_file_vec: Vec<File> = Vec::new();
            for path in os_str_main_index_json_file_path.iter() {
                os_str_main_index_json_file_vec.push(
                    fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(path)
                        .unwrap(),
                );
            }

            let main_index_json_file_buf = vec![
                [out_json, ",".to_string()].concat(),
                [
                    "\"".to_string(),
                    entry.clone().key.replace(';', "/"),
                    "\",".to_string(),
                ]
                .concat(),
            ];

            for (i, mut file) in os_str_main_index_json_file_vec.iter().enumerate() {
                file.write_all(main_index_json_file_buf[i].as_bytes())
                    .expect("Failed to write to osStr main/index json file");
            }
            for (i, mut file) in main_index_json_file_vec.iter().enumerate() {
                file.write_all(main_index_json_file_buf[i].as_bytes())
                    .expect("Failed to write to main/index json file");
            }
        }
    }

    for os_str in os_str_vec {
        let os_str_main_index_json_file_path_array = [
            ["./out/firmware/", os_str.as_str(), "/main.json"].concat(),
            ["./out/firmware/", os_str.as_str(), "/index.json"].concat(),
        ];

        for path in os_str_main_index_json_file_path_array {
            let file = fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .unwrap();
            let len = file.metadata().unwrap().len();
            let pos = if len > 1 { len - 1 } else { len };
            file.write_at("]\n".as_bytes(), pos)
                .expect("Failed to write to osStr main.json");
        }

        file_count += 1;
    }

    for file in main_index_json_file_vec {
        let len = file.metadata().unwrap().len();
        let pos = if len > 1 { len - 1 } else { len };
        file.write_at("]\n".as_bytes(), pos)
            .expect("Failed to write to main/index json file");
        file_count += 1;
    }

    file_count
}

fn main() {
    let now = std::time::Instant::now();
    let mut file_count: u32 = 0;
    file_count += create_firmware();
    let elapsed = now.elapsed();
    println!("Processed {} files in {:.2?}", file_count, elapsed);
}
