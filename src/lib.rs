use std::{fs::File, io::Write};

use curl::easy::Easy;
use tempfile;

fn fetch_json_to(mut file: File) -> Result<(), curl::Error> {
    let mut request = Easy::new();
    request.url("https://program.mch2022.org/mch2021-2020/schedule/export/schedule.json")?;
    request.write_function(move |data| {
        // unpacking this to match write_function's error handling just obscures
        // write_all's error, so never mind that.
        file.write_all(data).unwrap();
        Ok(data.len())
    })?;
    request.perform()
}

fn fetch_json() -> File {
    let file = tempfile::tempfile().unwrap();
    fetch_json_to(file.try_clone().unwrap()).unwrap();
    return file;
}

pub fn convert() {
    let schedule_json = fetch_json();
}

#[cfg(test)]
mod tests {
    use std::{fs::File, os::linux::fs::MetadataExt};
    #[test]
    fn fetch_to() {
        let file = File::create("fetch_to.json").unwrap();
        super::fetch_json_to(file.try_clone().unwrap()).unwrap();
        assert!(file.metadata().unwrap().st_size() > 1024);
        std::fs::remove_file("fetch_to.json").unwrap();
    }
}
