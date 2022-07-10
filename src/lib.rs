mod schedule;

use std::{
    fs::File,
    io::{Read, Seek, Write},
};

use chrono::{Duration, NaiveDateTime, NaiveTime};
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

fn parse(file: &mut File) -> anyhow::Result<json::JsonValue> {
    let mut body = String::new();
    file.read_to_string(&mut body)?;
    Ok(json::parse(&body)?)
}

fn parse_datetime(input: &str) -> anyhow::Result<NaiveDateTime> {
    Ok(chrono::DateTime::parse_from_rfc3339(input)?.naive_local())
}

fn parse_duration_hhmm(input: &str) -> anyhow::Result<Duration> {
    Ok(NaiveTime::parse_from_str(input, "%H:%M")? - NaiveTime::from_hms(0, 0, 0))
}

fn parse_people(input: &json::JsonValue) -> Vec<String> {
    input.members().map(|j| j.to_string()).collect()
}

fn extract_event(input: &json::JsonValue) -> anyhow::Result<schedule::Event> {
    Ok(schedule::Event {
        title: schedule::Title(input["title"].to_string()),
        room: schedule::Room(input["room"].to_string()),
        track: schedule::Track(input["track"].to_string()),
        start: parse_datetime(&input["date"].to_string())?,
        duration: parse_duration_hhmm(&input["duration"].to_string())?,
        brief: input["abstract"].to_string(),
        description: input["description"].to_string(),
        people: parse_people(&input["persons"]),
        event_type: input["type"].to_string(),
    })
}

fn extract_events(input: &json::JsonValue) -> anyhow::Result<Vec<schedule::Event>> {
    unimplemented!()
}

pub fn convert() -> anyhow::Result<()> {
    let json_file = download(SCHEDULE_URL).and_then(|mut f| parse(&mut f))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::schedule;
    use std::{fs::File, os::linux::fs::MetadataExt};

    #[test]
    fn fetch_to() {
        let file = File::create("fetch_to.json").unwrap();
        super::download_to(super::SCHEDULE_URL, file.try_clone().unwrap()).unwrap();
        assert!(file.metadata().unwrap().st_size() > 1024);
        std::fs::remove_file("fetch_to.json").unwrap();
    }

    #[test]
    fn extract_event() {
        let event_json = json::parse(r#"
        {
            "id": 109,
            "guid": "8021acd3-9860-5c31-bdcc-b1bdd25e4c87",
            "logo": "",
            "date": "2022-07-22T17:00:00+02:00",
            "start": "17:00",
            "duration": "00:50",
            "room": "Abacus  \ud83e\uddee",
            "slug": "mch2021-2020-109--may-contain-hackers-2022-opening",
            "url": "https://program.mch2022.org/mch2021-2020/talk/JBNXAX/",
            "title": "\u26a0\ufe0f May Contain Hackers 2022 Opening",
            "subtitle": "",
            "track": "MCH2022 Curated content",
            "type": "Talk",
            "language": "en",
            "abstract": "\u26a0\ufe0f Warning! This talk may contain hackers. There may be hackers in the room. There may be hackers surrounding the room. There may be hackers recording this. There may be hackers listening in. There may be hackers that exfiltrate data. There may be hackers wearing shirts. There may be hackers carrying spying devices. OH NO! There are hackers EVERYWHERE! What can we do now, except having a party?",
            "description": "This talk serves as an introduction to the camp. It tells how the camp works, what new features are being released, how to participate and what to be aware of.\r\n\r\nDuring this talk there will be some audio-trickery in the Abacus stage which can not be relayed to the recording or via the stream. As we cannot film audience reactions, know that it will be more epic than the final battle scene of LOTR.\r\n\r\nIn all seriousness: there are absolutely stunning new additions to the camp.\r\n\r\nI'm have to write at least 5",
            "recording_license": "",
            "do_not_record": false,
            "persons": [
                {
                    "id": 112,
                    "code": "SQMXG7",
                    "public_name": "Elger \"Stitch\" Jonker",
                    "biography": "Stitch is co-organizer for the MCH2022 hacker camp. Has also helped set up SHA2017 and does all kinds of things, for example helped build the design generator, work on the permit and spend office days organizing all kinds of stuff. In the past Stitch helped set up hackerspaces Hack42 and Awesome Space in the Netherlands.",
                    "answers": []
                }
            ],
            "links": [],
            "attachments": [],
            "answers": []
        }
"#).unwrap();
        let event = super::extract_event(&event_json).unwrap();
        assert_eq!(
            event.title,
            schedule::Title("\u{26a0}\u{fe0f} May Contain Hackers 2022 Opening".to_string())
        );
        assert_eq!(event.duration, chrono::Duration::minutes(50));
        assert_eq!(
            event.start,
            chrono::NaiveDateTime::parse_from_str("2022-07-22T17:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
        );
    }

    #[test]
    fn parse() {
        let mut file = File::open("test/mch-sched.json").unwrap();
        super::parse(&mut file).unwrap();
    }

    #[test]
    fn convert() {
        super::convert();
    }
}
