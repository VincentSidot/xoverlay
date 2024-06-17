# XOverlay
[![Build Status][build-img]][build-img]
[![Documentation][doc-img]][doc-url]

[build-img]: https://img.shields.io/github/actions/workflow/status/vincentsidot/xoverlay/rust.yml?branch=master&style=for-the-badge
[build-url]: https://github.com/VincentSidot/xoverlay/actions/workflows/rust.yml
[doc-img]: https://img.shields.io/badge/docs.rs-xoverlay-4d76ae?style=for-the-badge
[doc-url]: https://vincentsidot.github.io/xoverlay/

XOverlay is a Rust project.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

Make sure you have Rust installed on your machine. If not, you can download it from [here](https://www.rust-lang.org/tools/install).

### Installing

Clone the repository to your local machine:

```sh
git clone <repository_url>
```

Navigate to the project directory:

```sh
cd xoverlay
```

Build the project:

```sh
cargo build
```

## Running the examples

You can run the example by using the following command:

```sh
cargo run --example hello <window_id>
cargo run --example hello -- $(xwininfo | grep "Window id:" | grep -o "0x[0-9a-f]*") # Just click on the wanted window
cargo run --example find <window_name>
```

## Built With

* [x11rb](https://crates.io/crates/x11rb) - The X11 library used


## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
