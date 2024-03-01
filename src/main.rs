mod file;
mod json;
mod os_file;
use std::{
    fs,
    io::{Seek, Write},
    os::unix::fs::FileExt,
};
use walkdir::WalkDir;

fn main() {
    let now = std::time::Instant::now();
    let mut file_count = 0;

    fs::create_dir_all("./out/os").expect("Failed to create directory ./out/os");

    file::create_blank_file_and_overwrite("./out/os/main.json")
        .expect("Failed to create ./out/os/main.json");
    file::create_blank_file_and_overwrite("./out/os/index.json")
        .expect("Failed to create ./out/os/index.json");

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

    main_json_file
        .write_all("[".as_bytes())
        .expect("Failed to write to ./out/os/main.json");
    index_json_file
        .write_all("[".as_bytes())
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
                .write_all([out_json.to_string(), ','.to_string()].concat().as_bytes())
                .expect("Failed to write to ./out/os/main.json");
            index_json_file
                .write_all(
                    [
                        '"'.to_string(),
                        entry
                            .clone()
                            .key
                            .replace(';', "/")
                            .replace(' ', "-")
                            .to_string(),
                        '"'.to_string(),
                        ','.to_string(),
                    ]
                    .concat()
                    .as_bytes(),
                )
                .expect("Failed to write to ./out/os/index.json");

            file_count += 1;
        }
    }

    let main_json_position = index_json_file
        .stream_position()
        .expect("Failed to get main_json stream_position");
    main_json_file
        .write_at("]\n".as_bytes(), main_json_position - 1)
        .expect("Failed to write to ./out/os/main.json");
    let index_json_position = index_json_file
        .stream_position()
        .expect("Failed to get index_json stream_position");
    index_json_file
        .write_at("]\n".as_bytes(), index_json_position - 1)
        .expect("Failed to write to ./out/os/index.json");
    file_count += 2;

    let elapsed = now.elapsed();
    println!("Processed {} files in {:.2?}", file_count, elapsed);
}
