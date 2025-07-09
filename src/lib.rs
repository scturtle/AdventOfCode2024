use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;

pub fn get_test_input() -> std::io::Result<String> {
    read_to_string("test.txt")
}

pub fn get_input(day: i8) -> anyhow::Result<String> {
    dotenvy::dotenv().expect(".env");
    let session = std::env::var("session").expect("env session");
    let filename = format!("{day}.txt");
    let path = Path::new(&filename);
    if path.exists() {
        return Ok(read_to_string(path)?);
    }
    let url = format!("https://adventofcode.com/2024/day/{day}");
    let s = ureq::get(&format!("{url}/input"))
        .header("referer", &url)
        .header("cookie", &format!("session={session}"))
        .call()?
        .body_mut()
        .read_to_string()?;
    File::create(path)?.write_all(s.as_bytes())?;
    Ok(s)
}
