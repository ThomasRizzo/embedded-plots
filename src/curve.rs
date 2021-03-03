use core::ops::{Range};

use crate::range_conv::Scalable;
use itertools::{Itertools, MinMaxResult::MinMax, MinMaxResult};

use embedded_graphics::drawable::{Drawable};
use embedded_graphics::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{PixelColor};
use embedded_graphics::primitives::{Line, Primitive};
use embedded_graphics::style::PrimitiveStyle;


pub struct PlotPoint {
    pub x: i32,
    pub y: i32,
}

pub struct Curve<'a> {
    points: &'a [PlotPoint],
    pub x_range: Range<i32>,
    pub y_range: Range<i32>,
}

impl<'a> Curve<'a> {
    pub fn new(points: &'a [PlotPoint], x_range: Range<i32>, y_range: Range<i32>) -> Curve {
        Curve { points, x_range, y_range }
    }

    pub fn from_data(points: &'a [PlotPoint]) -> Curve {
        let x_range = match points
            .iter()
            .map(|p| (p.x))
            .minmax() {
            MinMaxResult::NoElements => 0..0,
            MinMaxResult::OneElement(v) => v..v,
            MinMax(min, max) => min..max,
        };

        let y_range = match points.iter().map(|p| (p.y)).minmax() {
            MinMaxResult::NoElements => 0..0,
            MinMaxResult::OneElement(v) => v..v,
            MinMax(min, max) => min..max,
        };

        Curve { points, x_range, y_range }
    }

    pub fn into_drawable_curve<C>(&self,
                                  top_left: &'a Point,
                                  bottom_right: &'a Point,
    ) -> DrawableCurve<C, impl Iterator<Item=Point> + '_>
        where C: PixelColor
    {
        assert!(top_left.x < bottom_right.x);
        assert!(top_left.y < bottom_right.y);
        assert!(!self.x_range.is_empty());
        assert!(!self.y_range.is_empty());

        let it = self.points.iter()
            .map(move |p| Point {
                x: p.x.scale_between_ranges(
                    &self.x_range,
                    &Range { start: top_left.x, end: bottom_right.x },
                ),
                y: p.y.scale_between_ranges(
                    &self.y_range,
                    &Range { start: bottom_right.y, end: top_left.y },
                ),
            });
        DrawableCurve {
            scaled_data: it,
            color: None,
            thickness: None,
        }
    }
}

pub struct DrawableCurve<C, I>
{
    scaled_data: I,
    color: Option<C>,
    thickness: Option<usize>,
}

impl<C, I> DrawableCurve<C, I>
    where
        C: PixelColor,
        I: Iterator<Item=Point>,
{
    pub fn set_color(mut self, color: C) -> DrawableCurve<C, I> {
        self.color = Some(color);
        self
    }
    pub fn set_thickness(mut self, thickness: usize) -> DrawableCurve<C,I> {
        self.thickness = Some(thickness);
        self
    }
}

impl<C, I> Drawable<C> for DrawableCurve<C, I>
    where C: PixelColor + Default,
          I: Iterator<Item=Point>,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        let color = match self.color {
            None => C::default(),
            Some(c) => c,
        };
        let thickness = match self.thickness {
            None => 2,
            Some(t) => t,
        };
        let style = PrimitiveStyle::with_stroke(color, thickness as u32);
        let mut iter = self.scaled_data.into_iter();
        let mut prev = iter.next().unwrap();
        for point in iter {
            Line::new(prev, point)
                .into_styled(style)
                .draw(display)?;
            prev = point;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
