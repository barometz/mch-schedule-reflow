use chrono::{Duration, NaiveDateTime};

struct Title(String);
struct Room(String);
struct Track(String);

struct Event {
    /// Title of the event.
    title: Title,
    /// Where the event takes place.
    room: Room,
    /// What track the event is part of.
    track: Track,
    /// The start of the event (local TZ, probably, it's fine)
    start: NaiveDateTime,
    /// How long the event is scheduled for.
    duration: Duration,
    /// The abstract.
    brief: String,
    /// The event description.
    description: String,
    /// Names of the people speaking/guiding/working/shopping the event.
    people: Vec<String>,
    /// The kind of event (talk, workshop, ...?)
    event_type: String,
}
