mod file;
mod json;
mod os_file;

fn main() {
    let now = std::time::Instant::now();
    let mut file_count: u32 = 0;
    file_count += os_file::create_firmware();
    let elapsed = now.elapsed();
    println!("Processed {} files in {:.2?}", file_count, elapsed);
}
