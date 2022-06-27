use core::{fmt::Write, ops::Range};
use heapless::String;

use embedded_graphics::{
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::Text,
    text::TextStyle,
};

use crate::range_conv::Scalable;
use embedded_graphics::mono_font::ascii::FONT_5X8;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::text::{Alignment, Baseline, TextStyleBuilder};

/// Used to provide alignment of an axis, it will be dsizerown exactly on the line marked by the points
pub enum Placement {
    X { x1: i32, x2: i32, y: i32 },
    Y { y1: i32, y2: i32, x: i32 },
}

/// Used to describe how densely ticks should be drawn
#[derive(Clone, Copy)]
pub enum Scale {
    /// Fixed scale means that ticks will be drawn between each increment of absolute distance provided.
    /// for example, on range 0..30 and Fixed(10), ticks will be drawn for 0, 10 and 20
    Fixed(usize),
    /// RangeFraction means that provided number of ticks ticks will be drawn on entire range
    /// for example, on range 0..60 and RangeFraction(3), ticks will be drawn for 0, 20 and 40
    RangeFraction(usize),
}

impl Default for Scale {
    fn default() -> Self {
        Scale::RangeFraction(5)
    }
}

/// Display-agnostic axis object, only contains scale range and title, can be converted to drawable axis for specific display
pub struct Axis<'a> {
    /// range that the scale will be drawn for
    range: Range<i32>,
    /// axis title displayed right next to it
    title: Option<&'a str>,
    /// Definition on how scale ticks should be drawn
    scale: Option<Scale>,
}

/// builder methods to modify axis decoration
impl<'a> Axis<'a> {
    /// create new axis data
    pub fn new(range: Range<i32>) -> Axis<'a> {
        Axis {
            range,
            title: None,
            scale: None,
        }
    }

    /// define how scale ticks should be drawn
    pub fn set_scale(mut self, scale: Scale) -> Axis<'a> {
        self.scale = Some(scale);
        self
    }

    /// set axis title
    pub fn set_title(mut self, title: &'a str) -> Axis<'a> {
        self.title = Some(title);
        self
    }

    /// turn axis data into drawable object suitable for specific display
    pub fn into_drawable_axis<C>(self, placement: Placement) -> DrawableAxis<'a, C>
    where
        C: PixelColor + Default,
        TextStyle: Clone + Default,
    {
        DrawableAxis {
            axis: self,
            placement,
            color: None,
            text_style: None,
            tick_size: None,
            thickness: None,
        }
    }
}

/// Drawable axis object, constructed for specific display
pub struct DrawableAxis<'a, C>
where
    C: PixelColor,
    TextStyle: Clone + Default,
{
    axis: Axis<'a>,
    placement: Placement,
    color: Option<C>,
    text_style: Option<MonoTextStyle<'a, C>>,
    tick_size: Option<usize>,
    thickness: Option<usize>,
}

impl<'a, C> DrawableAxis<'a, C>
where
    C: PixelColor + Default,
    TextStyle: Clone + Default,
{
    pub fn set_color(mut self, val: C) -> DrawableAxis<'a, C> {
        self.color = Some(val);
        self
    }
    pub fn set_text_style(mut self, val: MonoTextStyle<'a, C>) -> DrawableAxis<'a, C> {
        self.text_style = Some(val);
        self
    }

    /// set how wide tick should be drawn on the axis
    pub fn set_tick_size(mut self, val: usize) -> DrawableAxis<'a, C> {
        self.tick_size = Some(val);
        self
    }

    /// set thickness of the main line of the axis
    pub fn set_thickness(mut self, val: usize) -> DrawableAxis<'a, C> {
        self.thickness = Some(val);
        self
    }
}

impl<'a, C> Drawable for DrawableAxis<'a, C>
where
    C: PixelColor + Default,
    TextStyle: Clone + Default,
{
    type Color = C;
    type Output = ();

    /// most important function - draw the axis on the display
    fn draw<D: DrawTarget<Color = C>>(&self, display: &mut D) -> Result<(), D::Error> {
        let color = self.color.unwrap_or_default();
        let thickness = self.thickness.unwrap_or(1);
        let tick_size = self.tick_size.unwrap_or(2);

        let character_style = MonoTextStyle::new(&FONT_5X8, color);

        let scale_marks = match self.axis.scale.unwrap_or_default() {
            Scale::Fixed(interval) => self.axis.range.clone().into_iter().step_by(interval),
            Scale::RangeFraction(fraction) => {
                let len = self.axis.range.len();
                self.axis.range.clone().into_iter().step_by(len / fraction)
            }
        };
        match self.placement {
            Placement::X { x1, x2, y } => {
                let title_text_style = TextStyleBuilder::new()
                    .alignment(Alignment::Center)
                    .baseline(Baseline::Top)
                    .build();
                let tick_text_style = TextStyleBuilder::new()
                    .alignment(Alignment::Left)
                    .baseline(Baseline::Top)
                    .build();
                Line {
                    start: Point { x: x1, y },
                    end: Point { x: x2, y },
                }
                .into_styled(PrimitiveStyle::with_stroke(color, thickness as u32))
                .draw(display)?;
                if let Some(title) = self.axis.title {
                    Text::with_text_style(
                        title,
                        Point {
                            x: x1 + (x2 - x1) / 2,
                            y: y + 10,
                        },
                        character_style,
                        title_text_style,
                    )
                    .draw(display)?;
                }
                for mark in scale_marks {
                    let x = mark.scale_between_ranges(&self.axis.range, &(x1..x2));
                    Line {
                        start: Point {
                            x,
                            y: y - tick_size as i32,
                        },
                        end: Point {
                            x,
                            y: y + tick_size as i32,
                        },
                    }
                    .into_styled(PrimitiveStyle::with_stroke(color, thickness as u32))
                    .draw(display)?;
                    let mut buf: String<8> = String::new();
                    write!(buf, "{}", mark).unwrap();
                    Text::with_text_style(
                        &buf,
                        Point { x: x + 2, y: y + 2 },
                        character_style,
                        tick_text_style,
                    )
                    .draw(display)?;
                }
            }
            Placement::Y { y1, y2, x } => {
                let title_text_style = TextStyleBuilder::new()
                    .alignment(Alignment::Right)
                    .baseline(Baseline::Middle)
                    .build();
                let tick_text_style = TextStyleBuilder::new()
                    .alignment(Alignment::Right)
                    .baseline(Baseline::Top)
                    .build();
                Line {
                    start: Point { x, y: y1 },
                    end: Point { x, y: y2 },
                }
                .into_styled(PrimitiveStyle::with_stroke(color, thickness as u32))
                .draw(display)?;

                let mut tick_text_left_pos_bound = i32::MAX;
                for mark in scale_marks {
                    let y = mark.scale_between_ranges(&self.axis.range, &(y2..y1));
                    Line {
                        start: Point {
                            x: x - tick_size as i32,
                            y,
                        },
                        end: Point {
                            x: x + tick_size as i32,
                            y,
                        },
                    }
                    .into_styled(PrimitiveStyle::with_stroke(color, thickness as u32))
                    .draw(display)?;
                    let mut buf: String<8> = String::new();
                    write!(buf, "{}", mark).unwrap();
                    let tick_val = Text::with_text_style(
                        &buf,
                        Point { x, y },
                        character_style,
                        tick_text_style,
                    );
                    if tick_val.bounding_box().top_left.x < tick_text_left_pos_bound {
                        tick_text_left_pos_bound = tick_val.bounding_box().top_left.x
                    };
                    tick_val.draw(display)?;
                }
                if let Some(title) = self.axis.title {
                    Text::with_text_style(
                        title,
                        Point {
                            x: tick_text_left_pos_bound - 1,
                            y: y1 + (y2 - y1) / 2,
                        },
                        character_style,
                        title_text_style,
                    )
                    .draw(display)?;
                }
            }
        }
        Ok(())
    }
}
