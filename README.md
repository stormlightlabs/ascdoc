# AsciiDoc Zettlekasten for Desktop

This is a note taking and collecting application for desktop that
leverages AsciiDoc for markup.

## Getting Started/Pre-Requisites

- Install the Tauri CLI
- Install the Trunk CLI
- Add the wasm32 target

```bash
cargo install tauri-cli --version "^2.0.0" --locked
cargo install trunk --locked
rustup target add wasm32-unknown-unknown
```

Standalone Tailwind [CLI](https://tailwindcss.com/blog/standalone-cli):

```bash
npm install -g tailwindcss @tailwindcss/cli

npx @tailwindcss/cli -i styles.css -o styles.min.css --minify
```

Formatter for Leptos `view!` macro:

```bash
cargo install leptosfmt
```

## Manuals

- [Trunk](https://trunkrs.dev/guide/)
- [Leptos](https://book.leptos.dev/)
- [Tauri](https://v2.tauri.app/concept/)

## Notes

- This user on the Hacker News post about the v2 RC's permission issues:
[link](https://news.ycombinator.com/item?id=41145167)

## Future

### Integrations

- Hypothes.is
- OpenAI
- Anthropic
- Gemini

## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.
