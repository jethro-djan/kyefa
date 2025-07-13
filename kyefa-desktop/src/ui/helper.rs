use std::env;
use iced::widget::{svg, svg::Svg};
use iced::Color;


pub fn home<'a>(w: f32, h: f32, color: Option<Color>) -> Svg<'a> {
    let mut svg_widget = svg(svg::Handle::from_path(format!(
        "{}/assets/icons/home_24dp_1F1F1F_FILL1_wght400_GRAD0_opsz24.svg",
        env!("CARGO_MANIFEST_DIR")
    )))
    .height(h)
    .width(w);

    if let Some(color) = color {
        svg_widget = svg_widget.style(move |_theme, _status| svg::Style {
            color: Some(color),
        });
    }
    
    svg_widget
}

pub fn profile<'a>(w: f32, h: f32, color: Option<Color>) -> Svg<'a> {
    let mut svg_widget = svg(svg::Handle::from_path(format!(
        "{}/assets/icons/account_circle_24dp_1F1F1F_FILL1_wght400_GRAD0_opsz24.svg",
        env!("CARGO_MANIFEST_DIR")
    )))
    .height(h)
    .width(w);

    if let Some(color) = color {
        svg_widget = svg_widget.style(move |_theme, _status| svg::Style {
            color: Some(color),
        });
    }
    
    svg_widget
}

pub fn student<'a>(w: f32, h: f32, color: Option<Color>) -> Svg<'a> {
    let mut svg_widget = svg(svg::Handle::from_path(format!(
        "{}/assets/icons/school_24dp_1F1F1F_FILL1_wght400_GRAD0_opsz24.svg",
        env!("CARGO_MANIFEST_DIR")
    )))
    .height(h)
    .width(w);

    if let Some(color) = color {
        svg_widget = svg_widget.style(move |_theme, _status| svg::Style {
            color: Some(color),
        });
    }
    
    svg_widget
}

pub fn period<'a>(w: f32, h: f32, color: Option<Color>) -> Svg<'a> {
    let mut svg_widget = svg(svg::Handle::from_path(format!(
        "{}/assets/icons/view_week_24dp_1F1F1F_FILL1_wght400_GRAD0_opsz24.svg",
        env!("CARGO_MANIFEST_DIR")
    )))
    .height(h)
    .width(w);

    if let Some(color) = color {
        svg_widget = svg_widget.style(move |_theme, _status| svg::Style {
            color: Some(color),
        });
    }
    
    svg_widget
}

pub fn payment<'a>(w: f32, h: f32, color: Option<Color>) -> Svg<'a> {
    let mut svg_widget = svg(svg::Handle::from_path(format!(
        "{}/assets/icons/payments_24dp_1F1F1F_FILL1_wght400_GRAD0_opsz24.svg",
        env!("CARGO_MANIFEST_DIR")
    )))
    .height(h)
    .width(w);

    if let Some(color) = color {
        svg_widget = svg_widget.style(move |_theme, _status| svg::Style {
            color: Some(color),
        });
    }
    
    svg_widget
}

pub fn access<'a>(w: f32, h: f32, color: Option<Color>) -> Svg<'a> {
    let mut svg_widget = svg(svg::Handle::from_path(format!(
        "{}/assets/icons/door_open_24dp_1F1F1F_FILL1_wght400_GRAD0_opsz24.svg",
        env!("CARGO_MANIFEST_DIR")
    )))
    .height(h)
    .width(w);

    if let Some(color) = color {
        svg_widget = svg_widget.style(move |_theme, _status| svg::Style {
            color: Some(color),
        });
    }
    
    svg_widget
}

pub fn report<'a>(w: f32, h: f32, color: Option<Color>) -> Svg<'a> {
    let mut svg_widget = svg(svg::Handle::from_path(format!(
        "{}/assets/icons/bar_chart_24dp_1F1F1F_FILL1_wght400_GRAD0_opsz24.svg",
        env!("CARGO_MANIFEST_DIR")
    )))
    .height(h)
    .width(w);
    
    if let Some(color) = color {
        svg_widget = svg_widget.style(move |_theme, _status| svg::Style {
            color: Some(color),
        });
    }
    
    svg_widget
}
