# Chapter 1

## What is Iced?

Iced is a cross-platform GUI library for Rust, inspired by Elm.

## GUI Elements

A GUI application is generally composed of three fundamental parts: widgets,
interactions, and state. - A widget is a distinct visual element of a a user interface. - An interaction is an action that can be triggered by some widgets. - The state represents the attributes or information of a user interface.

### Elm Architecture

The Elm Architecture, used primarily as a way to build web applications, adds a
more precise abstraction to describe for its fundamental parts: - An application's state is called a `Model`. - Interactions are called `Messages`. - Updates are state mutations, dictated by the `Messages`. - A View is a function of state (`Model`) that returns a widget.

## Containers with Expanded Size

```rust
let container = Container::new(
        row![
            Text::new("Toolbar Item 1"),
            Text::new("Toolbar Item 2"),
            Text::new("Toolbar Item 3"),
            // Debugging
            Text::new(self.cwd())
        ]
        .spacing(10),
    )
    .style(container::bordered_box)
    .width(Length::Fill)
    .padding(10);
```

## Fonts

Loading Fonts:

```rust
iced::Settings {
    fonts: vec![
        include_bytes!("/path/to/font.ttf").into(),
        include_bytes!("path/to/other_font.tf").into(),
    ],
    default_font: Fonts::Neon.font(),
    default_text_size: iced::Pixels(16.0),
    antialiasing: true,
    ..Default::default()
}
```

In the above example, the `font` method on the `Fonts` enum (specific to how I've
structured my project) returns the `iced::Font` struct,
which is a wrapper around the index of the font in the `fonts` vector. This
implements the `Default` trait. Below is the default value:

```rust
Font {
    family: Family::SansSerif,
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
}
```

### Font Weight Bug

Variable weight fonts don't work properly. Weights such as light, normal, and bold
work as expected but the weight variant medium goes to a fallback.

See [this](https://github.com/iced-rs/iced/issues/2613) and [this](https://github.com/iced-rs/iced/issues/2060).
My solution to this was to instead store the medium weight otf files instead of
the variable weight ttf files for the Monaspace fonts.

## Tasks

From the doc comments:

> Concurrent actions performed by the iced runtime
>
> "`Task` must be returned to the runtime to take effect; normally in your
> `update` or `new` functions."

## Widgets

### Text

### TextEditor

### Button

### Row

### Column
