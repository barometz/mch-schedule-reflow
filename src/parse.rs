use crate::schedule;

use chrono::{Duration, NaiveDateTime, NaiveTime};

use std::{fs::File, io::Read};

fn parse_datetime(input: &str) -> anyhow::Result<NaiveDateTime> {
    Ok(chrono::DateTime::parse_from_rfc3339(input)?.naive_local())
}

fn parse_duration_hhmm(input: &str) -> anyhow::Result<Duration> {
    Ok(NaiveTime::parse_from_str(input, "%H:%M")? - NaiveTime::from_hms(0, 0, 0))
}

fn parse_people(input: &json::JsonValue) -> Vec<String> {
    input
        .members()
        .map(|j| j["public_name"].to_string())
        .collect()
}

fn parse_event(input: &json::JsonValue) -> anyhow::Result<schedule::Event> {
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
        url: input["url"].to_string(),
    })
}

pub fn file(file: &mut File) -> anyhow::Result<json::JsonValue> {
    let mut body = String::new();
    file.read_to_string(&mut body)?;
    Ok(json::parse(&body)?)
}

pub fn events(input: &json::JsonValue) -> anyhow::Result<Vec<schedule::Event>> {
    let mut all_events = Vec::<schedule::Event>::new();
    let conference = &input["schedule"]["conference"];

    for day in conference["days"].members() {
        for (_room, events) in day["rooms"].entries() {
            for event in events.members() {
                all_events.push(parse_event(event)?);
            }
        }
    }

    Ok(all_events)
}

#[cfg(test)]
mod tests {
    use super::schedule;
    use std::fs::File;

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
        let event = super::parse_event(&event_json).unwrap();
        assert_eq!(
            event.title,
            schedule::Title(String::from("⚠️ May Contain Hackers 2022 Opening"))
        );
        assert_eq!(event.room, schedule::Room(String::from("Abacus  🧮")));
        assert_eq!(
            event.track,
            schedule::Track(String::from("MCH2022 Curated content"))
        );
        assert_eq!(event.duration, chrono::Duration::minutes(50));
        assert_eq!(
            event.start,
            chrono::NaiveDateTime::parse_from_str("2022-07-22T17:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap()
        );
        assert!(event
            .brief
            .starts_with("⚠️ Warning! This talk may contain hackers."));
        assert!(event
            .description
            .starts_with("This talk serves as an introduction to the camp."));
        assert_eq!(event.people, vec![String::from("Elger \"Stitch\" Jonker")]);
        assert_eq!(event.event_type, String::from("Talk"));
        assert_eq!(
            event.url,
            String::from("https://program.mch2022.org/mch2021-2020/talk/JBNXAX/")
        );
    }

    #[test]
    fn parse() {
        let mut file = File::open("test/mch-sched.json").unwrap();
        super::file(&mut file).unwrap();
    }
}
