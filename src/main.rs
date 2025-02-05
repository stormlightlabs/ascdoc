use iced::widget::text_editor::Action;
use iced::widget::{button, container};
use iced::{widget, Length};
use iced::{
    widget::{row, text_editor, Column, Container, Text},
    Task, Theme,
};
use std::io::ErrorKind::{NotFound, Other};
use std::path::PathBuf;
use std::{io, path::Path};
use tokio::fs;
use tracing::Level;
use tracing_subscriber;

pub fn main() -> iced::Result {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    iced::application("MAD", AscDoc::update, AscDoc::view)
        .settings(ascdoc_ui::settings())
        .window(ascdoc_ui::window_settings())
        .theme(AscDoc::theme)
        .run_with(AscDoc::new)
}

/// Application State
#[derive(Debug)]
struct Model {
    cwd: PathBuf,
    ln: usize,
    col: usize,
    word_wrap: bool,
    buffer: text_editor::Content,
    // file: Option<PathBuf>,
    loading: bool,
}

impl Model {
    fn new() -> Self {
        Self::default()
    }

    fn position(&self) -> (usize, usize) {
        (self.ln, self.col)
    }
}

impl Default for Model {
    fn default() -> Self {
        Model {
            cwd: Path::new("~/Documents/AscDocWorkspace").to_path_buf(),
            ln: 1,
            col: 1,
            word_wrap: false,
            buffer: text_editor::Content::new(),
            loading: true,
            // file: None,
        }
    }
}

/// Application Interactions
#[derive(Debug, Clone)]
enum Message {
    CursorMoved(text_editor::Action),
    ToggleWrapped,
    // SetWorkingDir,
    // NewBuffer,
    // OpenFile,
    // SaveFile,
    Setup(Result<String, Error>),
    // FileOpened(Result<(PathBuf, Arc<String>), Error>),
    // FileSaved(Result<PathBuf, Error>),
}

/// Application Errors
#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

#[derive(Debug, Default)]
struct AscDoc {
    state: Model,
}

#[derive(Debug, Clone)]
struct Pos {
    ln: usize,
    col: usize,
}

impl AscDoc {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                state: Model::new(),
            },
            Task::batch([
                Task::perform(set_cwd(), Message::Setup),
                widget::focus_next(),
            ]),
        )
    }

    fn update_position(&mut self, pos: Pos) -> Task<Message> {
        self.state.ln = pos.ln;
        self.state.col = pos.col;

        Task::none()
    }

    fn toggle_wrapped(&mut self) -> Task<Message> {
        self.state.word_wrap = !self.state.word_wrap;

        Task::none()
    }

    fn cwd(&self) -> String {
        let home = std::env::var("HOME").unwrap();
        let cwd = self.state.cwd.to_str().unwrap();

        match cwd.split_once(home.as_str()) {
            Some(dirs) => {
                let ws_dir = dirs.1;
                format!("~{ws_dir}")
            }
            None => "Not found!".to_string(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNightStorm
    }

    fn handle_editor_action(&mut self, act: Action) -> Task<Message> {
        let cursor = self.state.buffer.cursor_position();
        match act {
            _ => self.update_position(Pos {
                ln: cursor.0,
                col: cursor.1,
            }),
        }
    }
}

impl AscDoc {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Setup(res) => match res {
                Ok(val) => {
                    tracing::debug!("working directory set to {val}");
                    self.state.cwd = Path::new(&val).to_path_buf();
                    self.state.loading = false;

                    Task::none()
                }
                Err(_) => Task::none(),
            },
            Message::CursorMoved(act) => self.handle_editor_action(act),
            Message::ToggleWrapped => self.toggle_wrapped(),
            // _ => Task::none(),
        }
    }

    fn view(&self) -> Container<Message> {
        tracing::debug!("current position {:?}", self.state.position());
        let toolbar = Container::new(
            row![
                button("Wrap").on_press(Message::ToggleWrapped),
                button("Toolbar Item 1"),
                button("Toolbar Item 2"),
                // Debugging
                Text::new(self.cwd())
            ]
            .spacing(10),
        )
        .style(container::bordered_box)
        .width(Length::Fill)
        .padding(10);

        let sidebar = Container::new(
            Column::new()
                .push(Text::new("Sidebar Item 1"))
                .push(Text::new("Sidebar Item 2"))
                .spacing(10),
        )
        .style(container::bordered_box)
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .padding(10);

        let main_content = Container::new(
            Column::new()
                .push(
                    text_editor(&self.state.buffer)
                        .placeholder("Type something here...")
                        .on_action(Message::CursorMoved)
                        .height(Length::Fill),
                )
                .spacing(10),
        )
        .style(container::bordered_box)
        .height(Length::Fill)
        .width(Length::FillPortion(3))
        .padding(10);

        Container::new(
            Column::new()
                .push(toolbar)
                .push(row![sidebar, main_content].spacing(20))
                .spacing(20),
        )
        .padding(20)
        .into()
    }
}

/// Check the default path for the existence of a directory and create it
/// with a README.md (TODO: or adoc file)
///
/// TODO: Remove hardcoded dir
async fn set_cwd() -> Result<String, Error> {
    tracing::debug!("setting current working directory");

    let home = std::env::var("HOME").unwrap();
    let path_str = format!("{}/Documents/AscDocWorkspace", home);
    let path = Path::new(&path_str);

    match fs::metadata(&path).await {
        Ok(_) => Ok(path_str.to_string()),
        Err(e) => match e.kind() {
            NotFound => {
                if let Ok(_res) = fs::create_dir_all(path).await {
                    tracing::debug!("created ws dir at {path_str}");

                    Ok(path_str.to_string())
                } else {
                    Err(Error::IoError(NotFound))
                }
            }
            _ => Err(Error::IoError(Other)),
        },
    }
}
