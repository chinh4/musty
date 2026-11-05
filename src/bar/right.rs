use amane::{Monitor, Parent, Rectangle, children, Row, Center, End};

// use crate::clock::Clock;

#[path = "./workspaces.rs"]
mod workspaces;

#[path = "./battery.rs"]
mod battery;

pub fn view(monitor: &Monitor) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .align_child(End, Center)
        .child(Row::new(children![
            battery::view(),
            workspaces::view(monitor)
        ]))
}
