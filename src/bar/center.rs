use amane::{Monitor, Parent, Rectangle, Center};

#[path = "./workspaces.rs"]
mod workspaces;

pub fn view(monitor: &Monitor) -> Rectangle {
    
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .align_child(Center, Center)
        .child(workspaces::view(monitor))
}
