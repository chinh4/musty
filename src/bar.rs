mod left;
mod center;
mod right;

use amane::{Full, Layer, LayerWindow, Row, children, Margin, Monitor, Parent, Rectangle, Vertical, Palette, Service, Zone, Align, Start, Center, End};

pub fn view(monitor: &Monitor) -> LayerWindow {
    let palette = Palette::read();

    LayerWindow::new()
        .width(Full)
        .height(32.0)
        .space(Zone::Reserve)
        .layer(Layer::Top)
        .anchor_vertical(Vertical::Bottom)
        .margin(Margin {
                top: 0,
                bottom: 25,
                left: 25,
                right: 25
            }
        )
        .child(
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill(palette.background())
                .border(1.0, palette.foreground())
                .padding(5.0)
                .child(Row::new(children![
                    section(left::view(monitor), Start),
                    section(center::view(monitor), Center),
                    section(right::view(monitor), End)
                ])),
        )
}

fn section(content: Rectangle, align: impl Into<Align>) -> Rectangle {
    let palette = Palette::read();

    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .fill(palette.background())
        .align_child(align, Center)
        .child(content)
}