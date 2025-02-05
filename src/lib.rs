use iced::{font::Family, window::Position, Font, Size};

pub enum Fonts {
    Neon,
    Argon,
    Radon,
    Xenon,
    Krypton,
    HKGrotesk,
}

impl Fonts {
    fn to_str(&self) -> &'static str {
        match self {
            Fonts::Neon => "Monaspace Neon",
            Fonts::Argon => "Monaspace Argon",
            Fonts::Xenon => "Monaspace Xenon",
            Fonts::Radon => "Monaspace Radon Var",
            Fonts::Krypton => "Monaspace Krypton Var",
            Fonts::HKGrotesk => "Hanken Grotesk",
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
                Fonts::Xenon | Fonts::Argon | Fonts::Neon => iced::font::Weight::Medium,
                _ => iced::font::Weight::Normal,
            },
            stretch: iced::font::Stretch::Normal,
        }
    }
}

/// Application settings
pub fn settings() -> iced::Settings {
    iced::Settings {
        fonts: vec![
            include_bytes!("../fonts/MonaspaceXenon-Medium.otf").into(),
            include_bytes!("../fonts/MonaspaceXenon-MediumItalic.otf").into(),
            include_bytes!("../fonts/MonaspaceArgon-Medium.otf").into(),
            include_bytes!("../fonts/MonaspaceArgon-MediumItalic.otf").into(),
            include_bytes!("../fonts/MonaspaceNeon-Medium.otf").into(),
            include_bytes!("../fonts/MonaspaceNeon-MediumItalic.otf").into(),
            // TODO: Change to fixed weight variants
            include_bytes!("../fonts/MonaspaceRadonVarVF[wght,wdth,slnt].ttf").into(),
            include_bytes!("../fonts/MonaspaceKryptonVarVF[wght,wdth,slnt].ttf").into(),
            include_bytes!("../fonts/HankenGrotesk-VariableFont_wght.ttf").into(),
        ],
        default_font: Fonts::Neon.font(),
        default_text_size: iced::Pixels(16.0),
        antialiasing: true,
        ..Default::default()
    }
}

pub fn window_settings() -> iced::window::Settings {
    iced::window::Settings {
        position: Position::Centered,
        decorations: true,
        resizable: true,
        size: Size::new(800.0, 600.0),
        ..Default::default()
    }
}
