use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use embedded_plots::{
    axis::Scale,
    curve::{Curve, PlotPoint},
    single_plot::SinglePlot,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 48));
    let data = vec![
        PlotPoint { x: 0, y: 0 },
        PlotPoint { x: 1, y: 2 },
        PlotPoint { x: 2, y: 2 },
        PlotPoint { x: 3, y: 0 },
    ];

    let curve = Curve::from_data(data.as_slice());
    let plot = SinglePlot::new(&curve, Scale::RangeFraction(3), Scale::RangeFraction(2))
        .into_drawable(Point { x: 18, y: 2 }, Point { x: 120, y: 30 })
        .set_color(BinaryColor::On);

    plot.draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    Window::new("Basic plot", &output_settings).show_static(&display);

    Ok(())
}
