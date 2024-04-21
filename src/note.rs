use iced::{theme::{QRCode, Rule}, widget::{button, qr_code, Button, Column, Row, Text}, Element, Sandbox};


// 我们定义一个NoteView的结构体，给该结构体实现相关方法

pub struct NoteView {
    // 标签的数量不是固定的，因此我们需要Vec来保存标签
    // Note中的标签使用String来保存，通过逗号隔离每个标签
    // 我们需要自己处理，将标签转换成tags
    tags: Vec<Tag>,// Tag我们将会下面定义
    // iced 中的button在构建的时候，需要我们提供一个&mut State
    // 我们将State放到NoteView里面保存
    // 我们只需要两个State就够了，因为Tag按钮的State我们将单独放到每个Tag结构体中去
    open_url: button::State,
    open_qrcode: button::State,
    delete: button::State,
}
// 我们只需要保存标签的名字和对应按钮的状态即可

pub struct Tag {
    name: String,
    open_tag: button::State, 
}
// 最后，就像是我们当初实现Application的时候一样
// 如果我们想要处理消息，我们就需要定义一个Message来保存我们可能会用到的结构体
#[derive(Debug, Clone)]
pub enum Message {
    OpenUrl,
    Delete,
    QRCode,
    Search(String),
}
impl Sandbox for NoteView {
    type Message=Message;
    fn new() -> Self {
        NoteView { tags: vec![], open_url: button::State::new(), open_qrcode: button::State::new(), delete: button::State::new() }.into()
    }
    fn view(&self) -> Element<'_, Self::Message> {
        
        let delete = Button::new(Text::new("delete")).on_press(Message::Delete);
        let qrcode_button = Button::new(Text::new("qr")).on_press(Message::QRCode);
        // 我们构建一个Row在同一行放入用于展示的数据
        let row = Row::new()
            .push(Text::new("rowid {}"))
            .push(qrcode_button);
        // 接着我们通过fold，将上述的row传入tags中，创建不同的tag按钮放入同一行
       
        // 我们需要展示的是多行，因此在创建号row之后，需要一个column来储存不同的行
        let mut column = Column::new().push(row);
     
        column = column.push(
            // 最后一行我们需要放入一个delete按钮，这个按钮大概在左右7比1的位置
            Row::new()
            .push(delete)
          
        );
        // 最后不要忘记调用into
        column.into()
    }
    // update方法除了&mut self参数之外，还需要一个Message参数用于判断需要处理哪一个消息
    fn update(&mut self, msg: Message) {
        match msg {
            // 我们定义一个open方法，传入参数即可
            Message::OpenUrl => {
                println!("open url")
            },
            Message::Delete => {
                // 我们无法在当前的层面上处理这个操作，只需要留白，等待上层结合的时候处理就行
                println!("delete");
            }
            Message::QRCode => {
                println!("QRCode")
            },
            Message::Search(tag) => {
                // 同delete，需要上层处理
                println!("search tag: {}", tag);
            }
        }
    }
    
    fn theme(&self) -> iced::Theme {
        iced::Theme::default()
    }
    
    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::default()
    }
    
    fn scale_factor(&self) -> f64 {
        1.0
    }
    
    fn title(&self) -> String {
        String::from("Grocery List App")
    }  
}
