use gpui::{div, rgb, Context, IntoElement, ParentElement, Render, Styled, Window};
use crate::components::banner::banner;
use crate::components::table::Table;

pub struct MainPage {
    pub table: Table,
}

impl Render for MainPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(rgb(0xffffff))
            .size_full()
            .flex()
            .flex_col()
            .children([
                banner().into_any_element(),
                div()
                    .flex_1()
                    .size_full()
                    .overflow_hidden()
                    .child(self.table.view_virtualized(cx))
                    .into_any_element(),
            ])
    }
}