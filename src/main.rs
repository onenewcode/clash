use iced::{executor, widget::text, Application, Command, Element, Settings, Theme};


pub struct LocalNative {
    state: State,
}
#[allow(clippy::large_enum_variant)]
pub enum State {
    Loading,
    Loaded(Data),
}
pub struct Data {
}
#[derive(Debug)]
pub enum Message {
    Loading(()),
}
impl Application for LocalNative {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self{state:State::Loading},Command::none())
    }

    fn title(&self) -> String {
        "ln-iced".to_owned()
    }

    fn update( &mut self,message: Self::Message,) -> Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        text("fds").into()
    }
}

pub fn settings() -> iced::Settings<()> {
    Settings {
        ..Default::default()
    }
}
fn main() -> iced::Result {
	LocalNative::run(Settings::default())
}


