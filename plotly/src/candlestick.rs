//! Candlestick plot

use crate::common::color::NamedColor;
use crate::common::{Calendar, Dim, Direction, HoverInfo, Label, Line, PlotType};
use crate::private;
use crate::Trace;
use serde::Serialize;

#[derive(Serialize, Debug, Default)]
pub struct Candlestick<T, O>
where
    T: Serialize + Default,
    O: Serialize + Default,
{
    r#type: PlotType,
    x: Vec<T>,
    open: Vec<O>,
    high: Vec<O>,
    low: Vec<O>,
    close: Vec<O>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xaxis")]
    x_axis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxis")]
    y_axis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<Line>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "whiskerwidth")]
    whisker_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    increasing: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decreasing: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
}

impl<T, O> Candlestick<T, O>
where
    T: Serialize + Default,
    O: Serialize + Default,
{
    pub fn new(
        x: Vec<T>,
        open: Vec<O>,
        high: Vec<O>,
        low: Vec<O>,
        close: Vec<O>,
    ) -> Box<Candlestick<T, O>> {
        let iline = Line::new().width(1.0).color(NamedColor::Green);
        let dline = Line::new().width(1.0).color(NamedColor::Red);
        Box::new(Candlestick {
            r#type: PlotType::Candlestick,
            x,
            open,
            high,
            low,
            close,
            increasing: Some(Direction::Increasing { line: iline }),
            decreasing: Some(Direction::Decreasing { line: dline }),
            ..Default::default()
        })
    }

    pub fn name(mut self, name: &str) -> Box<Candlestick<T, O>> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }

    pub fn visible(mut self, visible: bool) -> Box<Candlestick<T, O>> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<Candlestick<T, O>> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<Candlestick<T, O>> {
        self.legend_group = Some(legend_group.to_owned());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<Candlestick<T, O>> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn text(mut self, text: &str) -> Box<Candlestick<T, O>> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        Box::new(self)
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Candlestick<T, O>> {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }

    pub fn hover_text(mut self, hover_text: &str) -> Box<Candlestick<T, O>> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        Box::new(self)
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Candlestick<T, O>> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Candlestick<T, O>> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn x_axis(mut self, axis: &str) -> Box<Candlestick<T, O>> {
        self.x_axis = Some(axis.to_owned());
        Box::new(self)
    }

    pub fn y_axis(mut self, axis: &str) -> Box<Candlestick<T, O>> {
        self.y_axis = Some(axis.to_owned());
        Box::new(self)
    }

    pub fn line(mut self, line: Line) -> Box<Candlestick<T, O>> {
        self.line = Some(line);
        Box::new(self)
    }

    pub fn whisker_width(mut self, whisker_width: f64) -> Box<Candlestick<T, O>> {
        self.whisker_width = Some(whisker_width);
        Box::new(self)
    }

    pub fn increasing(mut self, increasing: Direction) -> Box<Candlestick<T, O>> {
        self.increasing = Some(increasing);
        Box::new(self)
    }

    pub fn decreasing(mut self, decreasing: Direction) -> Box<Candlestick<T, O>> {
        self.decreasing = Some(decreasing);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<Candlestick<T, O>> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Candlestick<T, O>> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }
}

impl<X, Y> Trace for Candlestick<X, Y>
where
    X: Serialize + Default,
    Y: Serialize + Default,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
