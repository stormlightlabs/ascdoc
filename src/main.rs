use iced::widget::container::Style;
use iced::widget::{button, container};
use iced::{widget, Background, Length};
use iced::{
    widget::{row, text_editor, Column, Container, Text},
    Task, Theme,
};
use settings::LocalFont;
use std::fs::OpenOptions;
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

enum FileType {
    PlainText,
    AsciiDoc,
    Markdown,
}

impl FileType {
    fn as_str(&self) -> &str {
        match self {
            Self::AsciiDoc => "adoc",
            Self::Markdown => "md",
            _ => "txt",
        }
    }
}

/// Application State
struct Model {
    cwd: PathBuf,
    file: Option<PathBuf>,
    ln: usize,
    col: usize,
    buffer: text_editor::Content,
    word_wrap: bool,
    loading: bool,
    modified: bool,
    extension: FileType,
    toast: Option<Toast>,
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
            extension: FileType::PlainText,
            toast: None,
        }
    }
}

#[derive(Debug, Clone)]
enum ToastLevel {
    Success,
    Fail,
}

#[derive(Debug, Clone)]
enum Toast {
    Fail(String),
    Success(String),
}

impl Toast {
    fn to_string(&self) -> &String {
        match self {
            Self::Success(m) => m,
            Self::Fail(m) => m,
        }
    }
}

/// Application Interactions
#[derive(Debug, Clone)]
enum Message {
    CursorMoved(text_editor::Action),
    SetupCompleted(Result<bool, Error>),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    ToggleWrapped,
    SaveFile,
    ShowToast(Result<Toast, Error>),
    HideToast(Result<(), Error>),
    // OpenDir - Perform Action
    // SetWorkingDir, - Update State
    // CreateFile
}

/// Application Errors
#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

#[derive(Default)]
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
        Theme::CatppuccinMocha
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
                    self.state.loading = false;

                    // When the workspace directory is created, we create
                    // an empty initial file or open a README.md file.
                    let ws_dir = self.cwd();
                    let fpath = format!("{}/README.md", ws_dir);

                    tracing::debug!("set working/ws dir to {}", ws_dir);

                    Task::perform(open_file(fpath), Message::FileOpened)
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
            Message::FileOpened(result) => match result {
                Err(why) => {
                    tracing::error!("failed to set buffer because {:?}", why);

                    Task::none()
                }
                Ok((path_buf, contents)) => {
                    let path = path_buf.clone();

                    self.state.buffer = text_editor::Content::with_text(&contents);
                    self.state.extension = match path.extension() {
                        Some(ext) => match ext.to_str().unwrap() {
                            "adoc" => FileType::AsciiDoc,
                            "md" => FileType::Markdown,
                            _ => FileType::PlainText,
                        },
                        None => FileType::PlainText,
                    };
                    self.state.file = Some(path_buf);

                    Task::none()
                }
            },
            Message::SaveFile => {
                let path = self.state.file.as_mut().unwrap().as_path();
                let text = self.state.buffer.text();
                let disp = path.display();

                let mut file = OpenOptions::new().write(true).open(path).unwrap();

                match file.write(text.as_bytes()) {
                    Ok(b) => {
                        tracing::debug!("wrote {} bytes to {}", b, disp);

                        self.state.modified = false;

                        Task::perform(
                            create_toast(
                                format!("saved file {}", disp),
                                ToastLevel::Success,
                            ),
                            Message::ShowToast,
                        )
                    }
                    Err(why) => {
                        tracing::error!(
                            "failed to write to {} because {}",
                            disp,
                            why.kind()
                        );

                        Task::perform(
                            create_toast(
                                format!("save failed: {}", why.kind()),
                                ToastLevel::Fail,
                            ),
                            Message::ShowToast,
                        )
                    }
                }
            }
            Message::ShowToast(res) => match res {
                Ok(toast) => {
                    tracing::debug!("message: {}", toast.to_string());

                    self.state.toast = Some(toast);
                    Task::perform(pause(3), Message::HideToast)
                }
                Err(err) => {
                    tracing::error!("sww {:?}", err);

                    Task::none()
                }
            },
            Message::HideToast(_res) => {
                self.state.toast = None;
                Task::none()
            }
        }
    }

    fn view(&self) -> Container<Message> {
        tracing::debug!("current position {:?}", self.state.position());

        let toast_message: Text = match &self.state.toast {
            Some(toast) => match toast {
                Toast::Success(m) => {
                    Text::new(m).color(iced::Color::parse("#22c55e").unwrap())
                }
                Toast::Fail(m) => {
                    Text::new(m).color(iced::Color::parse("#e11d48").unwrap())
                }
            },
            None => Text::new(""),
        };

        let status = if self.state.modified { "Unsaved" } else { "" };
        let status_bar = Container::new(
            row![
                Text::new(status),
                Column::new()
                    .align_x(iced::Alignment::Center)
                    .width(Length::Fill)
                    .push(toast_message),
                Container::new(Text::new(self.state.render_pos()))
                    .style(|t| Style {
                        // TODO: This can be a helper
                        background: Some(Background::Color(
                            iced::Color::parse("#9ca3af").unwrap()
                        )),
                        ..container::rounded_box(t)
                    })
                    .padding(iced::Padding::from([2, 3])),
            ]
            .spacing(10),
        )
        .style(container::bordered_box)
        .width(Length::Fill)
        .padding(10);

        let save_file: Option<Message> = if self.state.modified {
            Some(Message::SaveFile)
        } else {
            None
        };

        let toolbar = Container::new(
            row![
                button("Wrap").on_press(Message::ToggleWrapped),
                button("Save").on_press_maybe(save_file),
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
                        .font(LocalFont::Neon.font())
                        .placeholder("Type something here...")
                        .highlight(
                            self.state.extension.as_str(),
                            iced::highlighter::Theme::Base16Mocha,
                        )
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
                .push(status_bar)
                .spacing(20),
        )
        .padding(20)
        .into()
    }
}

async fn pause(sec: u64) -> Result<(), Error> {
    tokio::time::sleep(tokio::time::Duration::from_secs(sec)).await;

    Ok(())
}

async fn create_toast(message: String, level: ToastLevel) -> Result<Toast, Error> {
    let toast = match level {
        ToastLevel::Success => Toast::Success(message),
        _ => Toast::Fail(message),
    };

    Ok(toast)
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

/// Creates or reads a README.md file in the root of the workspace directory
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
                    match file.write(content.as_bytes()) {
                        Err(why) => {
                            tracing::error!("couldn't write to {}: {}", disp, why);
                            Err(Error::IoError(why.kind()))
                        }
                        Ok(b) => {
                            tracing::debug!("wrote {} bytes to {}", b, disp);
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
