use async_trait::async_trait;
use serde_json::Value;

use crate::peripheral::{IntoWrappedPeripheral, Peripheral, WrapPeripheralError};

use super::{generate_wrapper_impl, shared::color::Color};

mod monitor_scale;
pub use monitor_scale::*;

generate_wrapper_impl!(Monitor = "monitor");

impl<'a> Monitor<'a> {
    pub async fn set_text_scale(&self, scale: MonitorScale) {
        self.inner.call_method("setTextScale", scale).await;
    }

    pub async fn get_text_scale(&self) -> MonitorScale {
        match &self
            .inner
            .call_method("getTextScale", Value::Null)
            .await
            .unwrap()
            .unwrap()[..]
        {
            [Value::Number(n)] => MonitorScale(n.as_f64().unwrap()),
            _ => unreachable!(),
        }
    }

    pub async fn write(&self, text: &str) {
        self.inner.call_method("write", text).await;
    }

    pub async fn scroll(&self, y: usize) {
        self.inner.call_method("scroll", y).await;
    }

    pub async fn get_cursor_pos(&self) -> (usize, usize) {
        match &self
            .inner
            .call_method("getCursorPos", Value::Null)
            .await
            .unwrap()
            .unwrap()[..]
        {
            [Value::Number(x), Value::Number(y)] => {
                (x.as_u64().unwrap() as usize, y.as_u64().unwrap() as usize)
            }
            _ => unreachable!(),
        }
    }

    pub async fn set_cursor_pos(&self, x: usize, y: usize) {
        self.inner.call_method("setCursorPos", vec![x, y]).await;
    }

    pub async fn get_cursor_blink(&self) -> bool {
        match &self
            .inner
            .call_method("getCursorBlink", Value::Null)
            .await
            .unwrap()
            .unwrap()[..]
        {
            [Value::Bool(b)] => *b,
            _ => unreachable!(),
        }
    }

    pub async fn set_cursor_blink(&self, blink: bool) {
        self.inner.call_method("setCursorBlink", blink).await;
    }

    pub async fn get_size(&self) -> (usize, usize) {
        match &self
            .inner
            .call_method("getSize", Value::Null)
            .await
            .unwrap()
            .unwrap()[..]
        {
            [Value::Number(x), Value::Number(y)] => {
                (x.as_u64().unwrap() as usize, y.as_u64().unwrap() as usize)
            }
            _ => unreachable!(),
        }
    }

    pub async fn clear(&self) {
        self.inner.call_method("clear", Value::Null).await;
    }

    pub async fn clear_line(&self) {
        self.inner.call_method("clearLine", Value::Null).await;
    }

    pub async fn get_text_color(&self) -> Color {
        match &self
            .inner
            .call_method("getTextColor", Value::Null)
            .await
            .unwrap()
            .unwrap()[..]
        {
            [Value::Number(n)] => n.as_u64().unwrap().try_into().unwrap(),
            _ => unreachable!(),
        }
    }

    pub async fn set_text_color(&self, color: Color) {
        self.inner.call_method("setTextColor", color).await;
    }

    pub async fn get_background_color(&self) -> Color {
        match &self
            .inner
            .call_method("getBackgroundColor", Value::Null)
            .await
            .unwrap()
            .unwrap()[..]
        {
            [Value::Number(n)] => n.as_u64().unwrap().try_into().unwrap(),
            _ => unreachable!(),
        }
    }

    pub async fn set_background_color(&self, color: Color) {
        self.inner.call_method("setBackgroundColor", color).await;
    }

    pub async fn is_color(&self) -> bool {
        match &self
            .inner
            .call_method("isColor", Value::Null)
            .await
            .unwrap()
            .unwrap()[..]
        {
            [Value::Bool(b)] => *b,
            _ => unreachable!(),
        }
    }

    pub async fn blit(&self, text: &str, text_color: &str, background_color: &str) {
        self.inner
            .call_method("blit", vec![text, text_color, background_color])
            .await;
    }

    pub async fn set_palette_color(&self, color: Color, value: u32) {
        self.inner
            .call_method(
                "setPaletteColor",
                vec![Value::from(color), Value::from(value)],
            )
            .await;
    }

    pub async fn set_palette_color_rgb(&self, color: Color, r: f64, g: f64, b: f64) {
        self.inner
            .call_method(
                "setPaletteColor",
                vec![
                    Value::from(color),
                    Value::from(r),
                    Value::from(g),
                    Value::from(b),
                ],
            )
            .await;
    }

    pub async fn get_palette_color(&self, color: Color) -> (f64, f64, f64) {
        match &self
            .inner
            .call_method("getPaletteColor", color)
            .await
            .unwrap()
            .unwrap()[..]
        {
            [Value::Number(r), Value::Number(g), Value::Number(b)] => (
                r.as_f64().unwrap(),
                g.as_f64().unwrap(),
                b.as_f64().unwrap(),
            ),
            _ => unreachable!(),
        }
    }
}
