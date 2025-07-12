use std::env;
use iced::widget::{svg, svg::Svg};


pub fn home<'a>(size: f32) -> Svg<'a> {
    svg(svg::Handle::from_path(format!(
        "{}/assets/icons/home_24dp_1F1F1F_FILL1_wght400_GRAD0_opsz24.svg",
        env!("CARGO_MANIFEST_DIR")
    )))
    .height(size)
}

pub fn profile<'a>(size: f32) -> Svg<'a> {
    svg(svg::Handle::from_path(format!(
        "{}/assets/icons/account_circle_24dp_1F1F1F_FILL1_wght400_GRAD0_opsz24.svg",
        env!("CARGO_MANIFEST_DIR")
    )))
    .height(size)
}
