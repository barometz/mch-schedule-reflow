mod parse;
mod render;
mod schedule;

use std::{
    fs::File,
    io::{Seek, Write}
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

fn convert_file(json_file: &mut File) -> anyhow::Result<()> {
    let events = parse::file(json_file).and_then(|j| parse::events(&j))?;
    let mut intermediate = tempfile::NamedTempFile::new()?;
    render::render(&events, &mut intermediate)?;
    let mut pandoc = pandoc::new();
    pandoc.add_input(intermediate.path());
    pandoc.set_output(pandoc::OutputKind::File("schedule.epub".into()));
    pandoc.set_input_format(
        pandoc::InputFormat::Markdown,
        vec![
            pandoc::MarkdownExtension::HeaderAttributes,
            pandoc::MarkdownExtension::SimpleTables,
        ],
    );
    pandoc.set_output_format(pandoc::OutputFormat::Epub3, vec![]);
    pandoc.set_toc();
    pandoc.add_option(pandoc::PandocOption::Standalone);
    match pandoc.execute() {
        Ok(_) => (),
        Err(e) => {
            eprintln!(
                "Failed to write pandoc output. Intermediate file is at {}. Error: {}",
                intermediate.path().display(),
                e
            );
            intermediate.keep()?;
            Err(e)?;
        }
    }
    Ok(())
}

pub fn convert() -> anyhow::Result<()> {
    download(SCHEDULE_URL).and_then(|mut f| convert_file(&mut f))?;
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
    fn convert_local() {
        super::convert_file(&mut File::open("test/mch-sched.json").unwrap()).unwrap();
    }

    #[test]
    fn convert() {
        super::convert().unwrap();
    }
}
