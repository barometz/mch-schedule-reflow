use chrono::{DateTime, Duration, FixedOffset};
use serde::Serialize;
use serde_with::{serde_as, DurationSeconds};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Title(pub String);
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Room(pub String);
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Track(pub String);

// TODO: need day field in Event, because a 1AM event is on the schedule for yesterday

#[serde_as]
#[derive(Serialize)]
pub struct Event {
    /// Title of the event.
    pub title: Title,
    /// Where the event takes place.
    pub room: Room,
    /// What track the event is part of.
    pub track: Track,
    /// The start of the event
    pub start: DateTime<FixedOffset>,
    /// How long the event is scheduled for.
    #[serde_as(as = "DurationSeconds<String>")]
    pub duration: Duration,
    /// The abstract.
    pub brief: String,
    /// The event description.
    pub description: String,
    /// Names of the people speaking/guiding/working/shopping the event.
    pub people: Vec<String>,
    /// The kind of event (talk, workshop, ...?)
    pub event_type: String,
    /// A link to the event description on the website.
    pub url: String,
}
