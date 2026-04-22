# ratatui-reactive &emsp; [![Action Badge]][actions] [![Version Badge]][crates.io] [![License Badge]][license] [![Docs Badge]][docs]

[Version Badge]: https://img.shields.io/crates/v/ratatui-reactive.svg
[crates.io]: https://crates.io/crates/ratatui-reactive
[Action Badge]: https://github.com/JayceFayne/ratatui-reactive/workflows/Rust/badge.svg
[actions]: https://github.com/JayceFayne/ratatui-reactive/actions
[License Badge]: https://img.shields.io/crates/l/ratatui-reactive.svg
[license]: https://github.com/JayceFayne/ratatui-reactive/blob/master/LICENSE.md
[Docs Badge]: https://docs.rs/ratatui-reactive/badge.svg
[docs]: https://docs.rs/ratatui-reactive

A minimalistic framework for building TUI applications using fine-grained reactivity.

## Usage

Examples of how to use the library can be found [here](./examples).

## Why?

I built this library around my personal preference for creating UIs with reactive systems. [ratatui-reactive](https://github.com/jaycefayne/ratatui-reactive) is designed around a reactive graph, rather than the message-driven architectures (like Elm) commonly used in many Rust UI libraries.

## Contributing

If you find any errors in ratatui-reactive or just want to add a new feature feel free to [submit a PR](https://github.com/jaycefayne/ratatui-reactive/pulls).

## Credits

- [The Ratatui Developers](https://github.com/orgs/ratatui/people) for their very simple and flexible [ratatui](https://github.com/ratatui/ratatui) library for cooking up terminal user interfaces.
- [The Sycamore Developers](https://github.com/orgs/sycamore-rs/people) for their elegant implementation of [reactive primitives](https://github.com/sycamore-rs/sycamore/tree/main/packages/sycamore-reactive).
