use super::prelude::*;

mod monitor_scale;
pub use monitor_scale::*;

generate_wrapper_impl!(Monitor = "monitor");

impl<'a> Monitor<'a> {
    generate_wrapped_fn!(set_text_scale -> void = |scale: MonitorScale| => setTextScale(scale));

    generate_wrapped_fn!(
        get_text_scale -> MonitorScale = | | => getTextScale(Value::Null);
        [Value::Number(n)] => Ok(MonitorScale(n.as_f64().unwrap()))
    );

    generate_wrapped_fn!(write -> void = |text: impl ToString| => write(text.to_string()));

    generate_wrapped_fn!(scroll -> void = |y: usize| => scroll(y));

    generate_wrapped_fn!(
        get_cursor_pos -> (usize, usize) = | | => getCursorPos(Value::Null);
        [Value::Number(x), Value::Number(y)] => {
            Ok((x.as_u64().unwrap() as usize, y.as_u64().unwrap() as usize))
        }
    );

    generate_wrapped_fn!(set_cursor_pos -> void = |x: usize, y: usize| => setCursorPos(vec![x, y]));

    generate_wrapped_fn!(
        get_cursor_blink -> bool = | | => getCursorBlink(Value::Null);
        [Value::Bool(b)] => Ok(*b)
    );

    generate_wrapped_fn!(set_cursor_blink -> void = |blink: bool| => setCursorBlink(blink));

    generate_wrapped_fn!(
        get_size -> (usize, usize) = | | => getSize(Value::Null);
        [Value::Number(x), Value::Number(y)] => {
            Ok((x.as_u64().unwrap() as usize, y.as_u64().unwrap() as usize))
        }
    );

    generate_wrapped_fn!(clear -> void = | | => clear(Value::Null));

    generate_wrapped_fn!(clear_line -> void = | | => clearLine(Value::Null));

    generate_wrapped_fn!(
        get_text_color -> Color = | | => getTextColor(Value::Null);
        [Value::Number(n)] => Ok(n.as_u64().unwrap().try_into().unwrap())
    );

    generate_wrapped_fn!(set_text_color -> void = |color: Color| => setTextColor(color.into_u64()));

    generate_wrapped_fn!(
        get_background_color -> Color = | | => getBackgroundColor(Value::Null);
        [Value::Number(n)] => Ok(n.as_u64().unwrap().try_into().unwrap())
    );

    generate_wrapped_fn!(set_background_color -> void = |color: Color| => setBackgroundColor(color.into_u64()));

    generate_wrapped_fn!(
        is_color -> bool = | | => isColor(Value::Null);
        [Value::Bool(b)] => Ok(*b)
    );

    generate_wrapped_fn!(blit -> void = |text: impl ToString, text_color: impl ToString, background_color: impl ToString| => blit(vec![text.to_string(), text_color.to_string(), background_color.to_string()]));

    generate_wrapped_fn!(set_palette_color -> void = |color: Color, value: u32| => setPaletteColor(vec![Value::from(color), Value::from(value)]));

    generate_wrapped_fn!(set_palette_color_rgb -> void = |color: Color, r: f64, g: f64, b: f64| => setPaletteColor(vec![
        Value::from(color),
        Value::from(r),
        Value::from(g),
        Value::from(b)
    ]));

    generate_wrapped_fn!(
        get_palette_color -> (f64, f64, f64) = |color: Color| => getPaletteColor(color.into_u64());
        [Value::Number(r), Value::Number(g), Value::Number(b)] => Ok((
            r.as_f64().unwrap(),
            g.as_f64().unwrap(),
            b.as_f64().unwrap()
        ))
    );
}
