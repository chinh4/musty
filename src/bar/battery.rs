use amane::{Parent, Rectangle, Battery, Service, Text, Palette, Center};

pub fn view() -> Rectangle {
  let battery = Battery::read();
  let palette = Palette::read();

  let percent = format!("{}", battery.percent());

  Rectangle::new()
      .width(50.0)
      .height(Parent)
      .align_child(Center, Center)
      .child(Text::new(percent).color(palette.foreground()))
}