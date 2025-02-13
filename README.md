# AsciiDoc Zettlekasten for Desktop

This is a note taking and collecting application for desktop that leverages
AsciiDoc for markup. It leverages [`iced`](https://iced.rs) (an Elm-like library)
to handle the GUI.

The goal is to streamline the learning workflows by providing an easy way to
use a Zettlekasten system and a markup language conducive to book writing.

## Setup

Make sure you have the 2021 edition of Rust installed. Then in the project's
root dir, run `cargo run`.

```bash
git clone https://github.com/stormlightlabs/ascdoc.git
cd ascdoc
cargo build
cargo run
```

## Book

There are a lot of posts I've seen about iced being [difficult](https://www.reddit.com/r/rust/comments/1445lqd/iced_is_hard_to_learn/)
to learn. In fact, the [disclaimer](https://docs.rs/iced/0.13.1/iced/#disclaimer)
even mentions that the book is incomplete but asserts that the library adheres to
Rust paradigms and the Elm architecture. As a new Rust developer, I appreciate
the candidness and challenge but also want to make the learning curve easier for
others. Plus, what better way to learn than to teach? All that to say, I'm
storing my notes and providing some explanations in [book](docs/src/SUMMARY.md).

## Future

### Integrations

- Hypothes.is
- OpenAI
- Anthropic
- Gemini

## License

This project is licensed under the Mozilla Public License - see [LICENSE](LICENSE) for details.
