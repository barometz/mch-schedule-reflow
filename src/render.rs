use crate::schedule;

use std::{fs::File, io::Write};

use serde::Serialize;

mod templates {
    pub const EVENT: &str = r#"
__________  ____ 
Title       {{title}}
Date        {{weekday}}, {{month}} {{day_of_month}}
Room        {{room}}
__________  ____
"#;
}

/// All event info processed and pulled apart for rendering.
#[derive(Serialize)]
struct Event {
    title: schedule::Title,
    room: schedule::Room,
    weekday: String,
    month: String,
    day_of_month: String,
}

impl From<&schedule::Event> for Event {
    fn from(event: &schedule::Event) -> Self {
        Self {
            title: event.title.clone(),
            room: event.room.clone(),
            weekday: event.start.format("%A").to_string(),
            month: event.start.format("%B").to_string(),
            day_of_month: event.start.format("%-d").to_string(),
        }
    }
}

pub fn render(events: &[schedule::Event], output: &mut File) -> anyhow::Result<()> {
    output.write_all("# Events\n".as_bytes())?;
    let mut handlebars = handlebars::Handlebars::new();
    handlebars.register_template_string("event", templates::EVENT)?;

    for event in events {
        output.write_fmt(format_args!("\n## {}\n", event.title.0))?;
        // The clone seems like it shouldn't be necessary here
        handlebars.render_to_write("event", &Event::from(event), output.try_clone()?)?;
    }

    Ok(())
}
