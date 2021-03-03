use core::ops::Range;
use core::fmt::Write;
use heapless::{consts::*, String};

use embedded_graphics::{
    prelude::*,
    style::{TextStyle, PrimitiveStyle},
    primitives::Line,
    fonts::Text,
};
use crate::range_conv::Scalable;


pub enum Placement {
    X {
        x1: i32,
        x2: i32,
        y: i32,
    },
    Y {
        y1: i32,
        y2: i32,
        x: i32,
    },
}

pub enum Scale {
    Fixed(usize),
    RangeFraction(usize),
}

impl Default for Scale {
    fn default() -> Self {
        Scale::RangeFraction(5)
    }
}

pub struct Axis<'a> {
    range: Range<i32>,
    title: Option<&'a str>,
    scale: Option<Scale>,
}

impl<'a> Axis<'a>
{
    pub fn new(range: Range<i32>) -> Axis<'a> {
        Axis { range, title: None, scale: None }
    }

    pub fn set_scale(mut self, scale: Scale) -> Axis<'a> {
        self.scale = Some(scale);
        self
    }

    pub fn set_title(mut self, title: &'a str) -> Axis<'a> {
        self.title = Some(title);
        self
    }

    pub fn into_drawable_axis<C, F>(self, placement: Placement) -> DrawableAxis<'a, C, F>
        where
            C: PixelColor + Default,
            F: Font,
            TextStyle<C, F>: Clone + Default,
    {
        DrawableAxis{
            axis: self,
            placement,
            color: None,
            text_style: None,
            tick_size: None,
            thickness: None,
        }
    }
}

pub struct DrawableAxis<'a, C, F>
    where
        C: PixelColor,
        F: Font,
        TextStyle<C, F>: Clone + Default,
{
    axis: Axis<'a>,
    placement: Placement,
    color: Option<C>,
    text_style: Option<TextStyle<C, F>>,
    tick_size: Option<usize>,
    thickness: Option<usize>,
}

impl<'a, C, F> DrawableAxis<'a, C, F>
    where
        C: PixelColor + Default,
        F: Font,
        TextStyle<C, F>: Clone + Default,
{
    pub fn set_color(mut self, val: C) -> DrawableAxis<'a, C, F> {
        self.color = Some(val);
        self
    }
    pub fn set_text_style(mut self, val: TextStyle<C, F>) -> DrawableAxis<'a, C, F> {
        self.text_style = Some(val);
        self
    }
    pub fn set_tick_size(mut self, val: usize) -> DrawableAxis<'a, C, F> {
        self.tick_size = Some(val);
        self
    }
    pub fn set_thickness(mut self, val: usize) -> DrawableAxis<'a, C, F> {
        self.thickness = Some(val);
        self
    }
}


impl<'a, C, F> Drawable<C> for DrawableAxis<'a, C, F>
    where
        C: PixelColor + Default,
        F: Font + Copy,
        TextStyle<C, F>: Clone + Default,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        let color = self.color.unwrap_or_default();
        let text_style = self.text_style.unwrap_or_default();
        let thickness = self.thickness.unwrap_or(1);
        let tick_size = self.tick_size.unwrap_or(2);


        let scale_marks = match self.axis.scale.unwrap_or_default() {
            Scale::Fixed(interval) => {
                self.axis.range.clone().into_iter().step_by(interval)
            }
            Scale::RangeFraction(fraction) => {
                let len = self.axis.range.len();
                self.axis.range.clone().into_iter().step_by(len / fraction)
            }
        };
        match self.placement {
            Placement::X { x1, x2, y } => {
                Line { start: Point { x: x1, y }, end: Point { x: x2, y } }
                    .into_styled(PrimitiveStyle::with_stroke(color, thickness as u32))
                    .draw(display)?;
                if let Some(title) = self.axis.title {
                    let title = Text::new(title, Point { x: x1, y: y + 10 })
                        .into_styled(text_style);
                    let title = title.translate(Point { x: (x2 - x1) / 2 - title.size().width as i32 / 2, y: 0 });
                    title.draw(display)?;
                }

                for mark in scale_marks {
                    let x = mark.scale_between_ranges(&self.axis.range, &(x1..x2));
                    Line { start: Point { x, y: y - tick_size as i32 }, end: Point { x, y: y + tick_size as i32 } }
                        .into_styled(PrimitiveStyle::with_stroke(color, thickness as u32))
                        .draw(display)?;
                    let mut buf: String::<U8> = String::new();
                    write!(buf, "{}", mark).unwrap();
                    Text::new(&buf, Point { x: x + 2, y: y + 2 }).into_styled(text_style).draw(display)?;
                }
            }
            Placement::Y { y1, y2, x } => {
                Line { start: Point { x, y: y1 }, end: Point { x, y: y2 } }
                    .into_styled(PrimitiveStyle::with_stroke(color, thickness as u32))
                    .draw(display)?;

                let mut max_tick_text_width = 0;
                for mark in scale_marks {
                    let y = mark.scale_between_ranges(&self.axis.range, &(y2..y1));
                    Line { start: Point { x: x - tick_size as i32, y }, end: Point { x: x + tick_size as i32, y } }
                        .into_styled(PrimitiveStyle::with_stroke(color, thickness as u32))
                        .draw(display)?;
                    let mut buf: String::<U8> = String::new();
                    write!(buf, "{}", mark).unwrap();
                    let tick_val = Text::new(&buf, Point { x, y }).into_styled(text_style);
                    let tick_val = tick_val.translate(Point { x: -(tick_val.size().width as i32) - 2, y: 2 });
                    if tick_val.size().width > max_tick_text_width { max_tick_text_width = tick_val.size().width }
                    tick_val.draw(display)?;
                }
                if let Some(title) = self.axis.title {
                    let title = Text::new(title, Point { x, y: y1 })
                        .into_styled(text_style);
                    let title = title.translate(Point { x: -(title.size().width as i32) - max_tick_text_width as i32 - tick_size as i32 - 2, y: (y2 - y1) / 2 });
                    title.draw(display)?;
                }
            }
        }
        Ok(())
    }
}