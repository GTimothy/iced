use iced::{slider, Column, Element, ProgressBar, Sandbox, Settings, Slider};

pub fn main() -> iced::Result {
    Progress::run(Settings::default())
}

#[derive(Default)]
struct Progress {
    value: f32,
    progress_bar_slider: slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SliderChanged(f32),
}

impl Sandbox for Progress {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("A simple Progressbar")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SliderChanged(x) => self.value = x,
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .push(ProgressBar::new(0.0..=100.0, self.value))
            .push(
                Slider::new(
                    &mut self.progress_bar_slider,
                    0.0..=100.0,
                    self.value,
                    Message::SliderChanged,
                )
                .step(0.01),
            )
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
