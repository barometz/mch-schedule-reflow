use chrono::{Duration, NaiveDateTime};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Title(pub String);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Room(pub String);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Track(pub String);

pub struct Event {
    /// Title of the event.
    pub title: Title,
    /// Where the event takes place.
    pub room: Room,
    /// What track the event is part of.
    pub track: Track,
    /// The start of the event (local TZ, probably, it's fine)
    pub start: NaiveDateTime,
    /// How long the event is scheduled for.
    pub duration: Duration,
    /// The abstract.
    pub brief: String,
    /// The event description.
    pub description: String,
    /// Names of the people speaking/guiding/working/shopping the event.
    pub people: Vec<String>,
    /// The kind of event (talk, workshop, ...?)
    pub event_type: String,
}
