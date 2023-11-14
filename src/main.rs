use iced::widget::{ container,  text};
use iced::{ Element, Length,  Application, Theme, Command, Font, Settings};

pub const FIRA: Font = Font {
    family: iced::font::Family::Name("Fira Code"),
    weight: iced::font::Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    monospaced: true
};



#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded(Result<(), iced::font::Error>),
    Loaded(Result<(), String>), 
}


async fn load() -> Result<(), String> {
    Ok(())
}

enum LoadingState {
    Loading,
    Loaded
}

struct Example {
    loading_state: LoadingState
}


impl Application for Example {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Example, Command<Message>) {
        let font_data = include_bytes!("../fonts/FiraCode-Regular.ttf");
        let font = iced::font::load(font_data.as_slice());


        let example = Example {
            loading_state: LoadingState::Loading,
        };


        let command = Command::batch(vec![
            font.map(Message::FontLoaded),
            Command::perform(load(), Message::Loaded),
        ]);

        (example, command)
    }


    fn title(&self) -> String {
        String::from("Font Testing - Iced")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::FontLoaded(_) => (),
            Message::Loaded(font) => {
                self.loading_state = LoadingState::Loaded;
                match font {
                    Ok(_) => println!("Font loaded!"),
                    Err(e) => println!("{:?}", e),
                }
            },
        };
        Command::none()
    }


    fn view(&self) -> Element<Message> {
        match self.loading_state {
            LoadingState::Loaded => {
                container(
                    text("-> ++ == -- <- <= >= != =>")
                    .shaping(text::Shaping::Advanced)
                    .font(FIRA)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(40)
                .into()

            },
            LoadingState::Loading => {
                container(text("Loading"))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
            }
        }

    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }


}

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

