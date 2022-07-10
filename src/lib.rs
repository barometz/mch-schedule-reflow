mod parse;
mod render;
mod schedule;

use std::{
    fs::File,
    io::{Seek, Write},
};

use curl::easy::Easy;

const SCHEDULE_URL: &str = "https://program.mch2022.org/mch2021-2020/schedule/export/schedule.json";

fn download_to(url: &str, mut output: File) -> anyhow::Result<()> {
    let mut request = Easy::new();
    request.url(url)?;
    request.write_function(move |data| {
        // unpacking this to match write_function's error handling just obscures
        // write_all's error, so never mind that.
        output.write_all(data).unwrap();
        Ok(data.len())
    })?;
    Ok(request.perform()?)
}

fn download(url: &str) -> anyhow::Result<File> {
    let mut file = tempfile::tempfile()?;
    download_to(url, file.try_clone().unwrap())?;
    file.rewind()?;
    Ok(file)
}

pub fn convert(output: &std::path::Path) -> anyhow::Result<()> {
    let json_file = download(SCHEDULE_URL).and_then(|mut f| parse::file(&mut f))?;
    let events = parse::events(&json_file)?;
    let mut output_file = File::create(output)?;
    render::render(&events, &mut output_file)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs::File, os::linux::fs::MetadataExt};

    #[test]
    fn fetch_to() {
        let file = File::create("fetch_to.json").unwrap();
        super::download_to(super::SCHEDULE_URL, file.try_clone().unwrap()).unwrap();
        assert!(file.metadata().unwrap().st_size() > 1024);
        std::fs::remove_file("fetch_to.json").unwrap();
    }

    #[test]
    fn convert() {
        let file = tempfile::NamedTempFile::new().unwrap();
        super::convert(file.path()).unwrap();
    }
}
