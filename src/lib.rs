use iced::{executor, widget::text, Application, Command, Element, Settings, Theme};
use note::NoteView;
mod note;

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
     // 执行器的关联类型定义为默认即可，
    type Executor = executor::Default;
    type Flags = ();
    // 外部定义的Message需要在此处给Application这个trati指明
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
pub fn run() -> iced::Result{
    NoteView::run(Settings::default())
}