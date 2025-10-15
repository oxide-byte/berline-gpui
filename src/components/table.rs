use gpui::{div, rgb, IntoElement, ParentElement, RenderOnce, Styled, px, uniform_list, Context};
use crate::service::file_import::{import_data, MeetupUrl};

pub struct Table {
    pub rows: Vec<TableRow>,
}

#[derive(Clone, Debug)]
pub struct TableRow {
    pub id: String,
    pub domain: String,
    pub title: String,
    pub url: String,
    pub description: String,
}

impl Into<TableRow> for MeetupUrl {
    fn into(self) -> TableRow {
        TableRow {
            id: self.uri_uuid.to_string(),
            domain: self.host,
            title: self.title,
            url: self.url,
            description: self.auto_descr,
        }
    }
}

impl Table {
    pub fn initialize() -> Self {
        let rows: Vec<TableRow> = import_data().into_iter().map(Into::into).collect();
        Table { rows }
    }

    pub fn view(&self) -> impl IntoElement {
        div().flex().flex_col().children(
            self.rows
                .iter()
                .enumerate()
                .map(|(ix, row)| TableRowView::new(ix, row.clone())),
        )
    }

    pub fn view_virtualized<T: 'static>(&self, cx: &mut Context<T>) -> impl IntoElement {
        use std::ops::Range;
        let total = self.rows.len();
        let rows = self.rows.clone();

        // Header row for visibility and structure, similar to the sample
        let header = div()
            .flex()
            .flex_row()
            .w_full()
            .overflow_hidden()
            .border_b_1()
            .border_color(rgb(0xE0E0E0))
            .text_color(rgb(0x555555))
            .bg(rgb(0xF0F0F0))
            .py_1()
            .px_2()
            .children([
                div().whitespace_nowrap().flex_shrink_0().truncate().px_1().w(px(120.)).child("ID"),
                div().whitespace_nowrap().flex_shrink_0().truncate().px_1().w(px(160.)).child("DOMAIN"),
                div().whitespace_nowrap().flex_shrink_0().truncate().px_1().w(px(320.)).child("TITLE"),
                div().whitespace_nowrap().flex_shrink_0().truncate().px_1().w(px(320.)).child("URL"),
                div().whitespace_nowrap().flex_shrink_0().truncate().px_1().w(px(360.)).child("DESCRIPTION"),
            ]);

        // When there are no rows, render a simple placeholder so the table area is visible
        if total == 0 {
            return div()
                .flex()
                .flex_col()
                .child(header)
                .child(div().p_2().text_color(rgb(0x777777)).child("No data available"));
        }

        div()
            .flex()
            .flex_col()
            .size_full()
            .flex_1()
            .child(header)
            .child(
                div()
                    .flex_1()
                    .relative()
                    .size_full()
                    .min_h(px(200.))
                    .overflow_hidden()
                    .child(
                        uniform_list(
                            "rows",
                            total,
                            cx.processor(move |_this, range: Range<usize>, _window, _cx| {
                                let mut items = Vec::with_capacity(range.end.saturating_sub(range.start));
                                for ix in range {
                                    if let Some(row) = rows.get(ix) {
                                        items.push(TableRowView::new(ix, row.clone()));
                                    }
                                }
                                items
                            }),
                        )
                        .size_full(),
                    ),
            )
    }
}

#[derive(IntoElement)]
struct TableRowView {
    ix: usize,
    row: TableRow,
}

impl TableRowView {
    fn new(ix: usize, row: TableRow) -> Self {
        Self { ix, row }
    }
}

impl RenderOnce for TableRowView {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        let bg = if self.ix % 2 == 0 { rgb(0xFFFFFF) } else { rgb(0xFAFAFA) };
        div()
            .flex()
            .flex_row()
            .border_b_1()
            .border_color(rgb(0xE0E0E0))
            .bg(bg)
            .py_0p5()
            .px_2()
            .gap_2()
            .children([
                div().whitespace_nowrap().truncate().w(px(120.)).child(self.row.id),
                div().whitespace_nowrap().truncate().w(px(160.)).child(self.row.domain),
                div().whitespace_nowrap().truncate().w(px(320.)).child(self.row.title),
                div().whitespace_nowrap().truncate().w(px(320.)).child(self.row.url),
                div().whitespace_nowrap().truncate().w(px(360.)).child(self.row.description),
            ])
    }
}