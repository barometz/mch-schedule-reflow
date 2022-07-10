use crate::schedule;

use std::io::Write;

mod templates {
    pub const EVENTS: &str = r#"
# Events

{{#each this}}
## {{title}}
__________  ____ 
People      {{join people ", "}}
Time        {{friendly_time start}}
Duration    {{friendly_duration duration}}
Date        {{friendly_date start}}
Room        {{room}}
__________  ____

{{/each}}
"#;
}

mod helpers {
    use chrono::{DateTime, NaiveTime};
    use handlebars::{
        Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext,
    };

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
        out.write(&unrendered.format("%H:%M (%z)").to_string())?;
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

pub fn render(events: &[schedule::Event], output: &mut dyn Write) -> anyhow::Result<()> {
    let mut handlebars = handlebars::Handlebars::new();
    handlebars.register_helper("join", Box::new(helpers::join));
    handlebars.register_helper("friendly_date", Box::new(helpers::friendly_date));
    handlebars.register_helper("friendly_time", Box::new(helpers::friendly_time));
    handlebars.register_helper("friendly_duration", Box::new(helpers::friendly_duration));
    handlebars.register_template_string("events", templates::EVENTS)?;

    handlebars.render_to_write("events", &events, output as &mut dyn Write)?;

    Ok(())
}
