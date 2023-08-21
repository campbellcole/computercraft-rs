use super::prelude::*;

generate_wrapper_impl!(Printer = "printer");

impl<'a> Printer<'a> {
    generate_wrapped_fn!(write -> void = |text: impl ToString| => write(text.to_string()));

    generate_wrapped_fn!(
        get_cursor_pos -> (usize, usize) = | | => getCursorPos(Value::Null);
        [Value::Number(x), Value::Number(y)] => {
            Ok((x.as_u64().unwrap() as usize, y.as_u64().unwrap() as usize))
        }
    );

    generate_wrapped_fn!(set_cursor_pos -> void = |x: usize, y: usize| => setCursorPos(vec![x, y]));

    generate_wrapped_fn!(
        get_page_size -> (usize, usize) = | | => getPageSize(Value::Null);
        [Value::Number(x), Value::Number(y)] => {
            Ok((x.as_u64().unwrap() as usize, y.as_u64().unwrap() as usize))
        }
    );

    generate_wrapped_fn!(
        new_page -> bool = | | => newPage(Value::Null);
        [Value::Bool(b)] => Ok(*b)
    );

    generate_wrapped_fn!(
        end_page -> bool = | | => endPage(Value::Null);
        [Value::Bool(b)] => Ok(*b)
    );

    generate_wrapped_fn!(set_page_title -> void = |title: impl ToString| => setPageTitle(title.to_string()));

    generate_wrapped_fn!(unset_page_title -> void = | | => setPageTitle(Value::Null));

    generate_wrapped_fn!(
        get_ink_level -> i32 = | | => getInkLevel(Value::Null);
        [Value::Number(n)] => Ok(n.as_i64().unwrap() as i32)
    );
}
