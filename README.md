# white_light

"White light is visible light that contains all the colors of the visible spectrum combined in roughly equal proportions"

A powerful, zero-dependency Rust library for adding colors and styles to terminal text.

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-blue)

## Features

- 16 standard terminal colors (regular and bright variants)
- 24-bit RGB true color support (16.7 million colors)
- Text styling (bold, italic, underline)
- Multiple intuitive API styles
- Zero dependencies
- Comprehensive test suite
- Cross-platform ANSI color support

## Installation

Add this to your `Cargo.toml`:

```
[dependencies]
white_light = "0.1.0"
```


## Usage

Status messages with custom colors:

```
print_status("INFO", Color::Blue, "Loading configuration...");
print_status("WARNING", Color::Yellow, "Disk space is running low");
```

Text styling:

```
println!("{}", Style::new()
    .fg_rgb(220, 20, 60)  
    .bg_rgb(25, 25, 25)   
    .bold()
    .italic()
    .paint("Styled RGB text"));

println!("{}", Style::new()
        .fg(Color::Magenta)
        .bg(Color::BrightBlack)
        .italic()
        .paint("Italic magenta text on gray background"));
```