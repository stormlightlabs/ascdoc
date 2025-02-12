use iced::widget::{button, container};
use iced::{widget, Length};
use iced::{
    widget::{row, text_editor, Column, Container, Text},
    Task, Theme,
};
use std::io::ErrorKind::NotFound;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::{io, path::Path};
use tokio::fs::{self, File};
use tracing::Level;
use tracing_subscriber;
use widgets::spacer;

mod settings;
mod widgets;

pub fn main() -> iced::Result {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    iced::application("MAD", AscDoc::update, AscDoc::view)
        .settings(settings::application(None))
        .window(settings::window())
        .theme(AscDoc::theme)
        .run_with(AscDoc::new)
}

/// Application State
#[derive(Debug)]
struct Model {
    cwd: PathBuf,
    file: Option<PathBuf>,
    ln: usize,
    col: usize,
    buffer: text_editor::Content,
    word_wrap: bool,
    loading: bool,
    modified: bool,
}

impl Model {
    fn new() -> Self {
        Self::default()
    }

    fn position(&self) -> (usize, usize) {
        (self.ln, self.col)
    }

    fn render_pos(&self) -> String {
        format!("{}:{}", self.ln + 1, self.col + 1)
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
            file: None,
            ln: 0,
            col: 0,
            word_wrap: false,
            loading: true,
            modified: false,
            buffer: text_editor::Content::new(),
        }
    }
}

/// Application Interactions
#[derive(Debug, Clone)]
enum Message {
    CursorMoved(text_editor::Action),
    SetupCompleted(Result<bool, Error>),
    BufferSet(Result<(PathBuf, Arc<String>), Error>),
    ToggleWrapped,
    // OpenDir - Perform Action
    // SetWorkingDir, - Update State
    // SaveFile, - Perform Action
    // FileSaved(Result<PathBuf, Error>), - Update State
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
                Task::perform(setup_ws(), Message::SetupCompleted),
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
}

impl AscDoc {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetupCompleted(res) => match res {
                Err(_) => Task::none(),
                Ok(_) => {
                    let ws_dir = self.cwd();
                    let fpath = format!("{}/README.adoc", ws_dir);

                    tracing::debug!("set working/ws dir to {}", ws_dir);

                    self.state.loading = false;

                    // When the workspace directory is created, we create an
                    // empty initial file or open a README.adoc file.
                    Task::perform(open_file(fpath), Message::BufferSet)
                }
            },
            Message::CursorMoved(action) => {
                tracing::debug!("action: {:?}", action);

                let cursor = self.state.buffer.cursor_position();

                self.state.modified = self.state.modified || action.is_edit();
                self.state.ln = cursor.0;
                self.state.col = cursor.1;

                self.state.buffer.perform(action);

                Task::none()
            }
            Message::ToggleWrapped => {
                self.state.word_wrap = !self.state.word_wrap;
                Task::none()
            }
            Message::BufferSet(result) => match result {
                Err(why) => {
                    tracing::error!("failed to set buffer because {:?}", why);

                    Task::none()
                }
                Ok((path, contents)) => {
                    self.state.buffer = text_editor::Content::with_text(&contents);
                    self.state.file = Some(path);

                    Task::none()
                }
            },
        }
    }

    fn view(&self) -> Container<Message> {
        tracing::debug!("current position {:?}", self.state.position());
        let toolbar = Container::new(
            row![
                button("Wrap").on_press(Message::ToggleWrapped),
                button("Save"),
                button("Open"),
                spacer(),
                button("Learn")
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

        let status = if self.state.modified { "Unsaved" } else { "" };
        let status_bar = Container::new(
            row![
                Text::new(status),
                Column::new().width(Length::Fill),
                Text::new(self.state.render_pos())
            ]
            .spacing(10),
        )
        .style(container::bordered_box)
        .width(Length::Fill)
        .padding(10);

        Container::new(
            Column::new()
                .push(toolbar)
                .push(row![sidebar, main_content].spacing(20))
                .push(status_bar)
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

/// Creates or reads a README.adoc file in the root of the workspace directory
async fn open_file(filepath_str: String) -> Result<(PathBuf, Arc<String>), Error> {
    tracing::debug!("opening file {}", filepath_str);

    let filepath = Path::new(&filepath_str);
    let disp = filepath.display();
    let mut contents: String = String::new();
    match std::fs::File::open(&filepath) {
        Ok(mut bytes) => match bytes.read_to_string(&mut contents) {
            Ok(size) => {
                tracing::debug!("read {} bytes", size);
                Ok((PathBuf::from(filepath), Arc::new(contents)))
            }
            Err(why) => Err(Error::IoError(why.kind())),
        },
        Err(why) => match why.kind() {
            NotFound => match File::create(&filepath).await {
                Err(why) => {
                    tracing::error!("couldn't create {}: {}", disp, why);
                    Err(Error::IoError(why.kind()))
                }
                Ok(f) => {
                    let content: &str = "= Welcome to AscDoc";
                    let mut file = f.into_std().await;
                    match file.write_all(content.as_bytes()) {
                        Err(why) => {
                            tracing::error!("couldn't write to {}: {}", disp, why);
                            Err(Error::IoError(why.kind()))
                        }
                        Ok(_) => {
                            tracing::debug!(
                                "successfully {} wrote to {}",
                                content,
                                disp
                            );
                            Ok((PathBuf::from(filepath), Arc::new(content.to_string())))
                        }
                    }
                }
            },
            kind => {
                tracing::error!("failed to read file {} because {}", disp, kind);
                Err(Error::IoError(kind))
            }
        },
    }
}
