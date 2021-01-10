use iced::{Container, Element, Length, Sandbox, Settings, Svg};

pub fn main() -> iced::Result {
    Tiger::run(Settings::default())
}

struct Tiger;

impl Sandbox for Tiger {
    type Message = ();

    fn new() -> Self {
        Tiger
    }

    fn title(&self) -> String {
        String::from("SVG - Iced")
    }

    fn update(&mut self, _message: ()) {}

    fn view(&mut self) -> Element<()> {
        let svg = Svg::from_path(format!(
            "{}/resources/tiger.svg",
            env!("CARGO_MANIFEST_DIR")
        ))
        .width(Length::Fill)
        .height(Length::Fill);

        Container::new(svg)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}

/// This test only verifies that the example does not panic in the first 5 seconds of its execution.
#[cfg(test)]
mod test{
    use rusty_fork::rusty_fork_test;
    rusty_fork_test!{
        #![rusty_fork(timeout_ms = 5000)]

        #[test]
        #[should_panic(expected = "child process exceeded 5000 ms timeout")]
        fn test_name() {
            crate::main().unwrap()
        }
    }
}
