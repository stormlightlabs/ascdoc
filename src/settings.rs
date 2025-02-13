/// Settings module
///
use iced::{font::Family, window::Position, Font, Size};

// TODO: Change VF fonts to fixed weight variants
pub enum LocalFont {
    Neon,
    _Argon,
    _Radon,
    _Krypton,
    _Xenon,
    HKGrotesk,
}

impl LocalFont {
    fn to_str(&self) -> &'static str {
        match self {
            LocalFont::Neon => "Monaspace Neon",
            LocalFont::_Argon => "Monaspace Argon",
            LocalFont::_Xenon => "Monaspace Xenon",
            LocalFont::_Radon => "Monaspace Radon Var",
            LocalFont::_Krypton => "Monaspace Krypton Var",
            LocalFont::HKGrotesk => "Hanken Grotesk",
        }
    }

    fn family(&self) -> Family {
        Family::Name(&self.to_str())
    }

    pub fn font(&self) -> Font {
        Font {
            family: self.family(),
            style: iced::font::Style::Normal,
            weight: match self {
                LocalFont::_Xenon
                | LocalFont::_Argon
                | LocalFont::Neon
                | LocalFont::HKGrotesk => iced::font::Weight::Medium,
                _ => iced::font::Weight::Normal,
            },
            stretch: iced::font::Stretch::Normal,
        }
    }
}

/// Application settings
pub fn application(font_selection: Option<LocalFont>) -> iced::Settings {
    let font = match font_selection {
        None => LocalFont::HKGrotesk.font(),
        Some(f) => f.font(),
    };

    iced::Settings {
        fonts: vec![
            include_bytes!("../fonts/MonaspaceXenon-Medium.otf").into(),
            include_bytes!("../fonts/MonaspaceXenon-MediumItalic.otf").into(),
            include_bytes!("../fonts/MonaspaceArgon-Medium.otf").into(),
            include_bytes!("../fonts/MonaspaceArgon-MediumItalic.otf").into(),
            include_bytes!("../fonts/MonaspaceNeon-Medium.otf").into(),
            include_bytes!("../fonts/MonaspaceNeon-MediumItalic.otf").into(),
            include_bytes!("../fonts/MonaspaceRadonVarVF[wght,wdth,slnt].ttf").into(),
            include_bytes!("../fonts/MonaspaceKryptonVarVF[wght,wdth,slnt].ttf").into(),
            include_bytes!("../fonts/HankenGrotesk-VariableFont_wght.ttf").into(),
        ],
        default_font: font,
        default_text_size: iced::Pixels(16.0),
        antialiasing: true,
        ..Default::default()
    }
}

pub fn window() -> iced::window::Settings {
    iced::window::Settings {
        position: Position::Centered,
        decorations: true,
        resizable: true,
        size: Size::new(800.0, 600.0),
        ..Default::default()
    }
}
