use amane::{Center, Monitor, Parent, Palette, Rectangle, Row, Service, Text, Widget, Workspace, Workspaces};

pub fn view(monitor: &Monitor) -> Row {
    let workspaces = Workspaces::read();

    let mut buttons: Vec<Box<dyn Widget>> = Vec::new();

    for workspace in workspaces.list() {
        // only this monitor's workspaces
        if workspace.output() != Some(monitor.name.as_str()) {
            continue;
        }

        buttons.push(Box::new(button(workspace)));
    }

    Row::new(buttons).gap(2.0)
}

fn button(workspace: &Workspace) -> Rectangle {
    let palette = Palette::read();


    let fill = if workspace.focused() {
        palette.accent()
    } else if workspace.urgent() {
        palette.colors()[10]
    } else {
        palette.colors()[3]
    };

    let id = workspace.id();

    Rectangle::new()
        .width(25.0)
        .height(Parent)
        .fill(fill)
        .align_child(Center, Center)
        .on_click(move |_| Workspaces::focus(id))
        .child(Text::new(workspace.index().to_string()).color(palette.on_accent()))
}
