mod file;
mod json;
mod os_file;
use std::{
    fs,
    io::{Read, Seek, Write},
    os::unix::fs::FileExt,
};
use walkdir::WalkDir;

fn main() {
    let now = std::time::Instant::now();
    let mut file_count = 0;

    fs::create_dir_all("./out/os").expect("Failed to create directory ./out/os");

    let main_json_path = "./out/os/main.json".to_string();
    let index_json_path = "./out/os/index.json".to_string();
    file::create_blank_file_and_overwrite(&main_json_path)
        .expect("Failed to create ./out/os/main.json");
    file::write_string_to_file(&main_json_path, &"[".to_string())
        .expect("Failed to write to ./out/os/main.json");
    file::create_blank_file_and_overwrite(&index_json_path)
        .expect("Failed to create ./out/os/index.json");
    file::write_string_to_file(&index_json_path, &"[".to_string())
        .expect("Failed to write to ./out/os/main.json");

    let mut main_json_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("./out/os/main.json")
        .unwrap();
    let mut index_json_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("./out/os/index.json")
        .unwrap();

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

            let output = ["./out/os/", &entry.key.replace(';', "/"), ".json"].concat();
            file::write_string_to_file(&output, &out_json).expect("Failed to write JSON");
            file_count += 1;

            let os_str = &entry.osStr;
            let os_str_main_json_file_path = ["./out/os/", os_str.as_str(), "/main.json"].concat();
            let os_str_index_json_file_path =
                ["./out/os/", os_str.as_str(), "/index.json"].concat();
            if !os_str_vec.contains(os_str) {
                os_str_vec.push(os_str.clone());
                if file::path_exists(&os_str_main_json_file_path) {
                    fs::remove_file(&os_str_main_json_file_path)
                        .expect("Failed to delete osStr main.json file")
                }
                if file::path_exists(&os_str_index_json_file_path) {
                    fs::remove_file(&os_str_index_json_file_path)
                        .expect("Failed to delete osStr main.json file")
                }
            }

            if !file::path_exists(&os_str_main_json_file_path) {
                file::write_string_to_file(&os_str_main_json_file_path, &"[".to_string())
                    .expect("Failed to write to osStr main.json file");
            }
            if !file::path_exists(&os_str_index_json_file_path) {
                file::write_string_to_file(&os_str_index_json_file_path, &"[".to_string())
                    .expect("Failed to write to osStr index.json file");
            }

            let mut os_str_main_json_file = fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(os_str_main_json_file_path)
                .unwrap();

            let mut os_str_index_json_file = fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(os_str_index_json_file_path)
                .unwrap();

            let main_json_file_contents = out_json + ",";
            let main_json_file_buf = main_json_file_contents.as_bytes();

            let mut index_json_file_contents = entry.clone().key.replace(';', "/");
            index_json_file_contents =
                ["\"", index_json_file_contents.as_str(), "\"", ","].concat();
            let index_json_file_buf = index_json_file_contents.as_bytes();

            main_json_file
                .write_all(main_json_file_buf)
                .expect("Failed to write to ./out/os/main.json");
            os_str_main_json_file
                .write_all(main_json_file_buf)
                .expect("Failed to write to osStr main.json file");
            index_json_file
                .write_all(index_json_file_buf)
                .expect("Failed to write to ./out/os/index.json");
            os_str_index_json_file
                .write_all(index_json_file_buf)
                .expect("Failed to write to ./out/os/index.json");
        }
    }

    for os_str in os_str_vec {
        let os_str_main_json_file_path = ["./out/os/", os_str.as_str(), "/main.json"].concat();
        let os_str_index_json_file_path = ["./out/os/", os_str.as_str(), "/index.json"].concat();

        let os_str_main_json_file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(os_str_main_json_file_path)
            .unwrap();
        let os_str_index_json_file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(os_str_index_json_file_path)
            .unwrap();

        let os_str_main_json_len = os_str_main_json_file.metadata().unwrap().len();
        let os_str_index_json_len = os_str_index_json_file.metadata().unwrap().len();

        os_str_main_json_file
            .write_at("]\n".as_bytes(), os_str_main_json_len - 1)
            .expect("Failed to write to osStr main.json");
        os_str_index_json_file
            .write_at("]\n".as_bytes(), os_str_index_json_len - 1)
            .expect("Failed to write to osStr index.json");

        file_count += 1;
    }

    let main_json_position = main_json_file
        .stream_position()
        .expect("Failed to get main.json stream_position");
    let index_json_position = index_json_file
        .stream_position()
        .expect("Failed to get index.json stream_position");

    main_json_file
        .write_at("]\n".as_bytes(), main_json_position - 1)
        .expect("Failed to write to ./out/os/main.json");
    index_json_file
        .write_at("]\n".as_bytes(), index_json_position - 1)
        .expect("Failed to write to ./out/os/index.json");

    file_count += 2;

    let elapsed = now.elapsed();
    println!("Processed {} files in {:.2?}", file_count, elapsed);
}
