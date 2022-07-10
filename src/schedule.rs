use chrono::{DateTime, Duration, FixedOffset};
use serde::Serialize;
use serde_with::{serde_as, DurationSeconds};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Title(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Room(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Track(pub String);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Day(pub i8);

#[serde_as]
#[derive(Clone, Serialize)]
pub struct Event {
    /// Title of the event.
    pub title: Title,
    /// Where the event takes place.
    pub room: Room,
    /// What track the event is part of.
    pub track: Track,
    /// The day the event is listed under (2AM is still the previous day)
    pub day: Day,
    /// The start of the event
    pub start: DateTime<FixedOffset>,
    /// How long the event is scheduled for.
    #[serde_as(as = "DurationSeconds<i64>")]
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
    /// The guid in the source data; cheap source of header IDs.
    pub unique_id: String,
}
