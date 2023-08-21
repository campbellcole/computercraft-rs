use async_trait::async_trait;
use serde_json::Value;

use crate::{
    error::Result,
    peripheral::{generate_wrapper_impl, IntoWrappedPeripheral, Peripheral},
};

use super::shared::color::Color;

mod monitor_scale;
pub use monitor_scale::*;

generate_wrapper_impl!(Monitor = "monitor");

impl<'a> Monitor<'a> {
    pub async fn set_text_scale(&self, scale: MonitorScale) -> Result<()> {
        self.inner.call_method("setTextScale", scale).await?;

        Ok(())
    }

    pub async fn get_text_scale(&self) -> Result<MonitorScale> {
        match &self.inner.call_method("getTextScale", Value::Null).await?[..] {
            [Value::Number(n)] => Ok(MonitorScale(n.as_f64().unwrap())),
            _ => unreachable!(),
        }
    }

    pub async fn write(&self, text: &str) -> Result<()> {
        self.inner.call_method("write", text).await?;

        Ok(())
    }

    pub async fn scroll(&self, y: usize) -> Result<()> {
        self.inner.call_method("scroll", y).await?;

        Ok(())
    }

    pub async fn get_cursor_pos(&self) -> Result<(usize, usize)> {
        match &self.inner.call_method("getCursorPos", Value::Null).await?[..] {
            [Value::Number(x), Value::Number(y)] => {
                Ok((x.as_u64().unwrap() as usize, y.as_u64().unwrap() as usize))
            }
            _ => unreachable!(),
        }
    }

    pub async fn set_cursor_pos(&self, x: usize, y: usize) -> Result<()> {
        self.inner.call_method("setCursorPos", vec![x, y]).await?;

        Ok(())
    }

    pub async fn get_cursor_blink(&self) -> Result<bool> {
        match &self
            .inner
            .call_method("getCursorBlink", Value::Null)
            .await?[..]
        {
            [Value::Bool(b)] => Ok(*b),
            _ => unreachable!(),
        }
    }

    pub async fn set_cursor_blink(&self, blink: bool) -> Result<()> {
        self.inner.call_method("setCursorBlink", blink).await?;

        Ok(())
    }

    pub async fn get_size(&self) -> Result<(usize, usize)> {
        match &self.inner.call_method("getSize", Value::Null).await?[..] {
            [Value::Number(x), Value::Number(y)] => {
                Ok((x.as_u64().unwrap() as usize, y.as_u64().unwrap() as usize))
            }
            _ => unreachable!(),
        }
    }

    pub async fn clear(&self) -> Result<()> {
        self.inner.call_method("clear", Value::Null).await?;

        Ok(())
    }

    pub async fn clear_line(&self) -> Result<()> {
        self.inner.call_method("clearLine", Value::Null).await?;

        Ok(())
    }

    pub async fn get_text_color(&self) -> Result<Color> {
        match &self.inner.call_method("getTextColor", Value::Null).await?[..] {
            [Value::Number(n)] => Ok(n.as_u64().unwrap().try_into().unwrap()),
            _ => unreachable!(),
        }
    }

    pub async fn set_text_color(&self, color: Color) -> Result<()> {
        self.inner.call_method("setTextColor", color).await?;

        Ok(())
    }

    pub async fn get_background_color(&self) -> Result<Color> {
        match &self
            .inner
            .call_method("getBackgroundColor", Value::Null)
            .await?[..]
        {
            [Value::Number(n)] => Ok(n.as_u64().unwrap().try_into().unwrap()),
            _ => unreachable!(),
        }
    }

    pub async fn set_background_color(&self, color: Color) -> Result<()> {
        self.inner.call_method("setBackgroundColor", color).await?;

        Ok(())
    }

    pub async fn is_color(&self) -> Result<bool> {
        match &self.inner.call_method("isColor", Value::Null).await?[..] {
            [Value::Bool(b)] => Ok(*b),
            _ => unreachable!(),
        }
    }

    pub async fn blit(&self, text: &str, text_color: &str, background_color: &str) -> Result<()> {
        self.inner
            .call_method("blit", vec![text, text_color, background_color])
            .await?;

        Ok(())
    }

    pub async fn set_palette_color(&self, color: Color, value: u32) -> Result<()> {
        self.inner
            .call_method(
                "setPaletteColor",
                vec![Value::from(color), Value::from(value)],
            )
            .await?;

        Ok(())
    }

    pub async fn set_palette_color_rgb(&self, color: Color, r: f64, g: f64, b: f64) -> Result<()> {
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
            .await?;

        Ok(())
    }

    pub async fn get_palette_color(&self, color: Color) -> Result<(f64, f64, f64)> {
        match &self.inner.call_method("getPaletteColor", color).await?[..] {
            [Value::Number(r), Value::Number(g), Value::Number(b)] => Ok((
                r.as_f64().unwrap(),
                g.as_f64().unwrap(),
                b.as_f64().unwrap(),
            )),
            _ => unreachable!(),
        }
    }
}
