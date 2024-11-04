use std::ops::RangeInclusive;

use eframe::egui;
use egui_plot::{AxisHints, GridMark, Line, Plot, PlotPoint, PlotPoints};

pub struct CustomPlotUi<'a> {
    x_label: String,
    y_label: String,
    points: PlotPoints,
    label_formatter: Box<dyn Fn(&str, &PlotPoint) -> String + 'a>,
    title: String,
}

impl<'a> egui::Widget for CustomPlotUi<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.label(&self.title);
        ui.horizontal(|ui| {
            let x_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| {
                let val = mark.value;
                format!("{}. {}", val, &self.x_label)
            };
            let y_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| {
                let percent = mark.value;
                format!("{percent:.0}%")
            };
            let x_axes = vec![AxisHints::new_x()
                .label(&self.x_label)
                .formatter(x_formatter)];
            let y_axes = vec![AxisHints::new_x()
                .label(&self.y_label)
                .formatter(y_formatter)];

            let line = Line::new(self.points).name(&self.title);

            Plot::new(&self.title)
                .width(400.)
                .height(200.)
                .custom_x_axes(x_axes)
                .custom_y_axes(y_axes)
                .label_formatter(self.label_formatter)
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
        })
        .response
    }
}

impl<'a> CustomPlotUi<'a> {
    pub fn new(
        x_label: String,
        y_label: String,
        points: PlotPoints,
        label_formetter: Box<dyn Fn(&str, &PlotPoint) -> String + 'a>,
        title: String,
    ) -> Self {
        Self {
            x_label,
            y_label,
            points,
            label_formatter: label_formetter,
            title,
        }
    }
}
