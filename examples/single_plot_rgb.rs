use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

use embedded_plots::{
    axis::Scale,
    curve::{Curve, PlotPoint},
    single_plot::SinglePlot,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 272));
    let data = vec![
        PlotPoint { x: 0, y: 0 },
        PlotPoint { x: 1, y: 2 },
        PlotPoint { x: 2, y: 2 },
        PlotPoint { x: 3, y: 0 },
    ];

    let curve = Curve::from_data(data.as_slice());
    let plot = SinglePlot::new(&curve, Scale::RangeFraction(3), Scale::RangeFraction(2))
        .into_drawable(Point { x: 50, y: 10 }, Point { x: 430, y: 250 })
        .set_color(RgbColor::YELLOW)
        .set_text_color(RgbColor::WHITE);

    plot.draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().build();

    Window::new("Basic plot", &output_settings).show_static(&display);

    Ok(())
}
