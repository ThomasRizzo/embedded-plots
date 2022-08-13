use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

use embedded_plots::{
    axis::Scale,
    curve::{Curve, PlotPoint},
    single_plot::SinglePlot,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(480, 272));

    //min(x), max(x), min(y) and max(y) must be equal for each curve in order to display the plot correctly
    let data = vec![
        PlotPoint { x: 0, y: 0 },
        PlotPoint { x: 1, y: 2 },
        PlotPoint { x: 2, y: 2 },
        PlotPoint { x: 3, y: 0 },
    ];

    let data2 = vec![
        PlotPoint { x: 0, y: 0 },
        PlotPoint { x: 1, y: 2 },
        PlotPoint { x: 2, y: 1 },
        PlotPoint { x: 3, y: 1 },
    ];

    let curve = Curve::from_data(data.as_slice());
    let curve2 = Curve::from_data(data2.as_slice());
    let curve_list = [(curve, RgbColor::YELLOW), (curve2, RgbColor::BLUE)];

    let plot = SinglePlot::new(&curve_list, Scale::RangeFraction(3), Scale::RangeFraction(2))
        .into_drawable(Point { x: 50, y: 10 }, Point { x: 430, y: 250 })
        .set_color(RgbColor::YELLOW)
        .set_text_color(RgbColor::WHITE);

    plot.draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().build();

    Window::new("Basic plot", &output_settings).show_static(&display);

    Ok(())
}
