use amane::{Monitor, Parent, Rectangle, Start, Center};

#[path = "./workspaces.rs"]
mod workspaces;

pub fn view(monitor: &Monitor) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .align_child(Start, Center)
        .child(workspaces::view(monitor))
}
