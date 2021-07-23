use embedded_graphics::{
    mono_font::{
        ascii::{FONT_5X8, FONT_5X7},
        MonoTextStyleBuilder,
    },
    pixelcolor::Rgb565,
    prelude::*,
};

use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

use embedded_graphics::pixelcolor::RgbColor;
use embedded_plots::axis::{Axis, Placement, Scale};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 272));

    let text_style_white = MonoTextStyleBuilder::new()
        .font(&FONT_5X8)
        .text_color(RgbColor::WHITE)
        .build();

    let text_style_yellow_compact = MonoTextStyleBuilder::new()
        .font(&FONT_5X7)
        .text_color(RgbColor::YELLOW)
        .build();

    Axis::new(0..100)
        .set_title("X Fixed 0-100(10)")
        .set_scale(Scale::Fixed(10))
        .into_drawable_axis(Placement::X {
            x1: 40,
            x2: 230,
            y: 10,
        })
        .set_color(RgbColor::WHITE)
        .set_text_style(text_style_white)
        .set_thickness(2)
        .set_tick_size(2)
        .draw(&mut display)?;

    Axis::new(0..200)
        .set_title("X Fixed 0-200(100)")
        .set_scale(Scale::Fixed(100))
        .into_drawable_axis(Placement::X {
            x1: 240,
            x2: 470,
            y: 10,
        })
        .set_color(RgbColor::YELLOW)
        .set_text_style(text_style_yellow_compact)
        .set_tick_size(2)
        .draw(&mut display)?;

    Axis::new(0..100)
        .set_title("X Frac 0-100(7)")
        .set_scale(Scale::RangeFraction(7))
        .into_drawable_axis(Placement::X {
            x1: 50,
            x2: 220,
            y: 30,
        })
        .set_color(RgbColor::BLUE)
        .set_text_style(text_style_white)
        .set_tick_size(3)
        .draw(&mut display)?;

    Axis::new(0..200)
        .set_title("X Frac 0-200(4)")
        .set_scale(Scale::RangeFraction(4))
        .into_drawable_axis(Placement::X {
            x1: 250,
            x2: 460,
            y: 40,
        })
        .set_color(RgbColor::RED)
        .set_text_style(text_style_yellow_compact)
        .set_tick_size(7)
        .draw(&mut display)?;

    Axis::new(0..100)
        .set_title("Y Fixed 0-100(10)")
        .set_scale(Scale::Fixed(10))
        .into_drawable_axis(Placement::Y {
            y1: 70,
            y2: 230,
            x: 130,
        })
        .set_color(RgbColor::WHITE)
        .set_text_style(text_style_white)
        .set_tick_size(2)
        .draw(&mut display)?;

    Axis::new(0..200)
        .set_title("Y Fixed 0-200(100)")
        .set_scale(Scale::Fixed(100))
        .into_drawable_axis(Placement::Y {
            y1: 70,
            y2: 210,
            x: 260,
        })
        .set_color(RgbColor::YELLOW)
        .set_text_style(text_style_yellow_compact)
        .set_tick_size(1)
        .draw(&mut display)?;

    Axis::new(0..100)
        .set_title("Y Frac 0-100(7)")
        .set_scale(Scale::RangeFraction(7))
        .into_drawable_axis(Placement::Y {
            y1: 60,
            y2: 180,
            x: 370,
        })
        .set_color(RgbColor::BLUE)
        .set_text_style(text_style_white)
        .set_tick_size(3)
        .draw(&mut display)?;

    Axis::new(0..200)
        .set_title("Y Frac 0-200(4)")
        .set_scale(Scale::RangeFraction(4))
        .into_drawable_axis(Placement::Y {
            y1: 90,
            y2: 220,
            x: 470,
        })
        .set_color(RgbColor::RED)
        .set_text_style(text_style_yellow_compact)
        .set_tick_size(7)
        .draw(&mut display)?;

    Axis::new(123..2137)
        .set_title("X")
        .set_scale(Scale::Fixed(150))
        .into_drawable_axis(Placement::X {
            x1: 30,
            x2: 470,
            y: 250,
        })
        .set_color(RgbColor::YELLOW)
        .set_text_style(text_style_white)
        .set_tick_size(2)
        .draw(&mut display)?;

    Axis::new(0..2137)
        .set_title("Y")
        .set_scale(Scale::RangeFraction(15))
        .into_drawable_axis(Placement::Y {
            y1: 10,
            y2: 250,
            x: 30,
        })
        .set_color(RgbColor::WHITE)
        .set_text_style(text_style_white)
        .set_tick_size(2)
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().build();
    Window::new("Free axis", &output_settings).show_static(&display);

    Ok(())
}
