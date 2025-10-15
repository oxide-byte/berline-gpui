use gpui::{div, img, rgb, AbsoluteLength, InteractiveElement, IntoElement, ParentElement, Pixels, Styled};
use std::path::PathBuf;

pub fn banner() -> impl IntoElement {
    div()
        .p_1_12()
        .flex()
        .flex_row()
        .child(crab_image())
        .child(title())
}

fn crab_image() -> impl IntoElement {
    let crab_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("images/browsing_ferris.png");
    div()
        .child(
            img(crab_path)
                .size(AbsoluteLength::Pixels(Pixels::from(100.0)))
                .h(AbsoluteLength::Pixels(Pixels::from(100.0)))
                .id("crab_image")
        )
}

fn title() -> impl IntoElement {
    div()
        .pt_3()
        .pl_3()
        .flex()
        .flex_col()
        .items_center()
        .child(
            div()
               .text_2xl()
               .text_color(rgb(0x000000))
               .child("Winter of Rust Desktop Frameworks")
        )
        .child(
            div()
                .text_xl()
                .text_color(rgb(0x000000))
                .child("GPUI (ZED) Version")
        )
}