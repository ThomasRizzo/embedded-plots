use core::ops::Range;

use crate::range_conv::Scalable;
use itertools::{Itertools, MinMaxResult, MinMaxResult::MinMax};

use embedded_graphics::{draw_target::DrawTarget, geometry::Point, Drawable};

use embedded_graphics::primitives::Primitive;
use embedded_graphics::{primitives::Line, primitives::PrimitiveStyle};
use embedded_graphics::pixelcolor::PixelColor;

/// representation of the single point on the curve
pub struct PlotPoint {
    pub x: i32,
    pub y: i32,
}

/// curve object that contains data to be plotted
pub struct Curve<'a> {
    /// slice of points to be drawn
    points: &'a [PlotPoint],
    pub x_range: Range<i32>,
    pub y_range: Range<i32>,
}

impl<'a> Curve<'a> {
    /// create new curve data with manual ranges
    pub fn new(points: &'a [PlotPoint], x_range: Range<i32>, y_range: Range<i32>) -> Curve {
        Curve {
            points,
            x_range,
            y_range,
        }
    }

    /// create new curve data with ranges automatically deducted based on provided points
    pub fn from_data(points: &'a [PlotPoint]) -> Curve {
        let x_range = match points.iter().map(|p| (p.x)).minmax() {
            MinMaxResult::NoElements => 0..0,
            MinMaxResult::OneElement(v) => v..v,
            MinMax(min, max) => min..max,
        };

        let y_range = match points.iter().map(|p| (p.y)).minmax() {
            MinMaxResult::NoElements => 0..0,
            MinMaxResult::OneElement(v) => v..v,
            MinMax(min, max) => min..max,
        };

        Curve {
            points,
            x_range,
            y_range,
        }
    }

    /// create curve that can be drawed on specific display
    pub fn into_drawable_curve<C>(
        &self,
        top_left: &'a Point,
        bottom_right: &'a Point,
    ) -> DrawableCurve<C, impl Iterator<Item = Point> + Clone + '_>
    where
        C: PixelColor,
    {
        assert!(top_left.x < bottom_right.x);
        assert!(top_left.y < bottom_right.y);
        assert!(!self.x_range.is_empty());
        assert!(!self.y_range.is_empty());

        let it = self.points.iter().map(move |p| Point {
            x: p.x.scale_between_ranges(
                &self.x_range,
                &Range {
                    start: top_left.x,
                    end: bottom_right.x,
                },
            ),
            y: p.y.scale_between_ranges(
                &self.y_range,
                &Range {
                    start: bottom_right.y,
                    end: top_left.y,
                },
            ),
        });
        DrawableCurve {
            scaled_data: it,
            color: None,
            thickness: None,
        }
    }
}

/// Drawable curve object, constructed for specific display
pub struct DrawableCurve<C, I> {
    scaled_data: I,
    color: Option<C>,
    thickness: Option<usize>,
}

/// builder methods to modify curve decoration
impl<C, I> DrawableCurve<C, I>
where
    C: PixelColor,
    I: Iterator<Item = Point> + Clone,
{
    /// set curve color
    pub fn set_color(mut self, color: C) -> DrawableCurve<C, I> {
        self.color = Some(color);
        self
    }

    /// set curve line thickness
    pub fn set_thickness(mut self, thickness: usize) -> DrawableCurve<C, I> {
        self.thickness = Some(thickness);
        self
    }
}

impl<C, I> Drawable for DrawableCurve<C, I>
where
    C: PixelColor + Default,
    I: Iterator<Item = Point> + Clone,
{
    type Color = C;
    type Output = ();

    /// most important function - draw the curve on the display
    fn draw<D: DrawTarget<Color = C>>(
        &self,
        display: &mut D,
    ) -> Result<(), <D as DrawTarget>::Error> {
        let color = match &self.color {
            None => C::default(),
            Some(c) => *c,
        };
        let thickness = match &self.thickness {
            None => 2,
            Some(t) => *t,
        };
        let style = PrimitiveStyle::with_stroke(color, thickness as u32);
        self.scaled_data.clone().tuple_windows().try_for_each(
            |(prev, point)| -> Result<(), D::Error> {
                Line::new(prev, point).into_styled(style).draw(display)
            },
        )
    }
}
