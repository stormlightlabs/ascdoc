use iced::widget::{button, container};
use iced::{widget, Length};
use iced::{
    widget::{row, text_editor, Column, Container, Text},
    Task, Theme,
};
use std::io::ErrorKind::NotFound;
use std::io::Write;
use std::path::PathBuf;
use std::{io, path::Path};
use tokio::fs::{self, File};
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
    file: Option<PathBuf>,
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

fn default_ws_directory() -> String {
    let home = std::env::var("HOME").unwrap();
    format!("{}/Documents/AscDocWorkspace", home)
}

impl Default for Model {
    fn default() -> Self {
        Model {
            cwd: PathBuf::from(default_ws_directory()),
            ln: 1,
            col: 1,
            word_wrap: false,
            buffer: text_editor::Content::new(),
            loading: true,
            file: None,
        }
    }
}

/// Application Interactions
#[derive(Debug, Clone)]
enum Message {
    CursorMoved(text_editor::Action),
    Setup(Result<bool, Error>),
    CreateFirstNote(Result<(), Error>),
    OpenFile(Result<PathBuf, Error>),
    SetBuffer(Result<(), Error>),
    ToggleWrapped,
    // SetWorkingDir(String),
    // SaveFile,
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

impl AscDoc {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                state: Model::new(),
            },
            Task::batch([
                Task::perform(setup_ws(), Message::Setup),
                widget::focus_next(),
            ]),
        )
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNightStorm
    }

    fn cwd(&self) -> String {
        let cwd = self.state.cwd.to_str();
        cwd.unwrap().to_string()
    }

    fn create_first_note(&mut self, created: bool) -> Task<Message> {
        tracing::debug!("set working/ws dir to {}", self.cwd());

        self.state.loading = false;

        if created {
            Task::perform(create_first_note(self.cwd()), Message::CreateFirstNote)
        } else {
            Task::none()
        }
    }

    fn set_buffer(&mut self, fpath: Option<PathBuf>) -> Task<Message> {
        if let Some(filepath) = fpath {
            self.state.file = Some(filepath)
        }
        Task::perform(set_buffer(), Message::SetBuffer)
    }
}

impl AscDoc {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Setup(res) => match res {
                Err(_) => Task::none(),
                Ok(created) => self.create_first_note(created),
            },
            Message::CursorMoved(_act) => {
                let cursor = self.state.buffer.cursor_position();
                self.state.ln = cursor.0;
                self.state.col = cursor.1;
                Task::none()
            }
            Message::CreateFirstNote(_res) => match self.state.loading {
                true => Task::none(),
                false => Task::perform(open_file(), Message::OpenFile),
            },
            Message::ToggleWrapped => {
                self.state.word_wrap = !self.state.word_wrap;
                Task::none()
            }
            Message::OpenFile(res) => match res {
                Err(_) => Task::none(),
                Ok(filepath) => self.set_buffer(Some(filepath)),
            },
            Message::SetBuffer(_res) => Task::none(),
        }
    }

    fn view(&self) -> Container<Message> {
        tracing::debug!("current position {:?}", self.state.position());
        let toolbar = Container::new(
            row![
                button("Wrap").on_press(Message::ToggleWrapped),
                button("Toolbar Item 1"),
                button("Toolbar Item 2"),
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
async fn setup_ws() -> Result<bool, Error> {
    tracing::debug!("setting current working directory");

    let home = std::env::var("HOME").unwrap();
    let p = format!("{}/Documents/AscDocWorkspace", home);
    let path = Path::new(&p);
    let disp = path.display();

    match fs::metadata(&path).await {
        Err(err) => match err.kind() {
            NotFound => match fs::create_dir_all(path).await {
                Err(why) => {
                    tracing::error!("couldn't create {} because {}", disp, why);
                    Err(Error::IoError(why.kind()))
                }
                Ok(_) => {
                    tracing::debug!("created workspace dir at {}", disp);
                    Ok(true)
                }
            },
            why => {
                tracing::error!("couldn't get metadata for {} because {}", disp, err);
                Err(Error::IoError(why))
            }
        },
        Ok(_) => {
            tracing::debug!("{} already present", disp);
            Ok(false)
        }
    }
}

/// Creates a README.adoc file in the root of the workspace directory. This should
/// only occur when the directory is created.
async fn create_first_note(ws_dir: String) -> Result<(), Error> {
    let p = format!("{}/{}/README.adoc", std::env::var("HOME").unwrap(), ws_dir);
    let path = Path::new(&p);
    let disp = path.display();
    let content: &str = "= Welcome to AscDoc";

    let mut file = match File::create(&path).await {
        Err(why) => {
            tracing::error!("couldn't create {}: {}", disp, why);
            return Err(Error::IoError(why.kind()));
        }
        Ok(file) => file.into_std().await,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => {
            tracing::error!("couldn't write to {}: {}", disp, why);
            Err(Error::IoError(why.kind()))
        }
        Ok(_) => {
            tracing::debug!("successfully {} wrote to {}", content, disp);
            Ok(())
        }
    }
}

async fn open_file() -> Result<PathBuf, Error> {
    Ok(PathBuf::new())
}
async fn set_buffer() -> Result<(), Error> {
    Ok(())
}
