use crate::schedule;

use std::{collections::HashMap, io::Write};

mod templates {
    pub const EVENTS: &str = r#"
# Events {#events}

{{#each this}}
## {{title}} {#{{unique_id}}}
__________  ____ 
People      {{join people ", "}}
Time        {{friendly_time start}}
Duration    {{friendly_duration duration}}
Date        {{friendly_date start}} ([Day {{day}}](#day-{{day}}))
Room        [{{room}}](#room-{{id room}})
__________  ____

{{/each}}
"#;

    pub const ROOM_DAY_EVENTS: &str = r#"
# Rooms {#rooms}

{{#each this}}
## {{@key}} {#room-{{id @key}}}

{{#each this}}
### Day {{@key}} ({{friendly_date this.0.start}}) {#room-{{id @../key}}-day-{{@key}}}

{{#each this}}
- {{friendly_time start}}: [{{title}}](#{{unique_id}})
{{/each}} <!-- events -->

{{/each}} <!-- days -->

{{/each}} <!-- rooms -->
"#;

    pub const DAY_ROOM_EVENTS: &str = r#"
# Days {#days}

{{#each this}}
## Day {{@key}} {#day-{{@key}}}

{{#each this}}
### {{@key}} {#day-{{@../key}}-room-{{id @key}}}

{{#each this}}
- {{friendly_time start}}: [{{title}}](#{{unique_id}})
{{/each}} <!-- events -->

{{/each}} <!-- rooms -->

{{/each}} <!-- days -->

"#;
}

mod helpers {
    use chrono::{DateTime, NaiveTime};
    use handlebars::{
        Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext,
    };

    pub fn id(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let source = h.param(0).unwrap().render();
        out.write(&source.replace(|c: char| !c.is_ascii_alphanumeric(), ""))?;
        Ok(())
    }

    pub fn join(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let elems = h.param(0).unwrap();
        let separator = h.param(1).unwrap();
        // it seems like there should be a better way than this
        out.write(
            &elems
                .value()
                .as_array()
                .unwrap()
                .iter()
                .map(|j| j.render())
                .collect::<Vec<String>>()
                .join(&separator.render()),
        )?;

        Ok(())
    }

    pub fn friendly_date(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap();
        // There's probably a more work-with-serde way to do this, but This Is Fine
        let unrendered = DateTime::parse_from_rfc3339(&param.value().render()).unwrap();
        out.write(&unrendered.format("%A, %B %-d").to_string())?;
        Ok(())
    }

    pub fn friendly_time(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap();
        // There's probably a more work-with-serde way to do this, but This Is Fine
        let unrendered = DateTime::parse_from_rfc3339(&param.value().render()).unwrap();
        out.write(&unrendered.format("%H:%M").to_string())?;
        Ok(())
    }

    pub fn friendly_duration(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap();
        let seconds: u32 = param.value().render().parse()?;
        out.write(
            &NaiveTime::from_num_seconds_from_midnight(seconds, 0)
                .format("%H:%M")
                .to_string(),
        )?;
        Ok(())
    }
}

type DayEvent = HashMap<schedule::Day, Vec<schedule::Event>>;
type RoomEvent = HashMap<schedule::Room, Vec<schedule::Event>>;
type RoomDayEvent = HashMap<schedule::Room, DayEvent>;
type DayRoomEvent = HashMap<schedule::Day, RoomEvent>;

fn make_room_day_event(events: &[schedule::Event]) -> RoomDayEvent {
    let mut room_day_events = RoomDayEvent::new();
    for event in events {
        let day_events = room_day_events
            .entry(event.room.clone())
            .or_insert(DayEvent::new());
        day_events
            .entry(event.day)
            .or_insert(Vec::<schedule::Event>::new())
            .push(event.clone());
    }
    room_day_events
}

fn make_day_room_event(events: &[schedule::Event]) -> DayRoomEvent {
    let mut day_room_events = DayRoomEvent::new();
    for event in events {
        let room_events = day_room_events.entry(event.day).or_insert(RoomEvent::new());
        room_events
            .entry(event.room.clone())
            .or_insert(Vec::<schedule::Event>::new())
            .push(event.clone());
    }
    day_room_events
}

pub fn render(events: &[schedule::Event], output: &mut dyn Write) -> anyhow::Result<()> {
    let mut handlebars = handlebars::Handlebars::new();
    handlebars.register_helper("id", Box::new(helpers::id));
    handlebars.register_helper("join", Box::new(helpers::join));
    handlebars.register_helper("friendly_date", Box::new(helpers::friendly_date));
    handlebars.register_helper("friendly_time", Box::new(helpers::friendly_time));
    handlebars.register_helper("friendly_duration", Box::new(helpers::friendly_duration));

    handlebars.render_template_to_write(
        templates::DAY_ROOM_EVENTS,
        &make_day_room_event(events),
        output as &mut dyn Write,
    )?;
    handlebars.render_template_to_write(
        templates::ROOM_DAY_EVENTS,
        &make_room_day_event(&events),
        output as &mut dyn Write,
    )?;
    handlebars.render_template_to_write(templates::EVENTS, &events, output as &mut dyn Write)?;

    Ok(())
}
