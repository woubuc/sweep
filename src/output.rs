use std::io::{stdout, Write};
use yansi::{Color, Paint, Style};

pub const LABEL_WIDTH: usize = 12;

pub fn error<S: Into<String>>(message: S) {
    println("Error", Color::Red, &message.into());
}

pub fn println<S: Into<String>>(label: S, label_colour: Color, message: S) {
    print(label, label_colour, message);
    println!();
}

pub fn println_info<S: Into<String>>(message: S) {
    print_info(message);
    println!();
}

pub fn println_plain<S: Into<String>>(colour: Option<Color>, message: S) {
    print_plain(colour, message);
    println!();
}

pub fn print<S: Into<String>>(label: S, label_colour: Color, message: S) {
    let term_width = get_term_width();
    let message = shorten(message.into(), term_width - LABEL_WIDTH - 1);
    let label = label.into();

    if label.len() > LABEL_WIDTH {
        panic!(format!("Label {} too long", label));
    }

    print!(
        "{}{} {}{}\r",
        " ".repeat(LABEL_WIDTH - label.len()),
        Style::new(label_colour).bold().paint(label),
        message,
        " ".repeat(term_width - LABEL_WIDTH - message.len() - 1),
    );
    stdout().flush().unwrap();
}

pub fn print_info<S: Into<String>>(message: S) {
    let term_width = get_term_width();
    let message = shorten(message.into(), term_width - LABEL_WIDTH - 1);

    print!(
        "{} {}{}\r",
        " ".repeat(LABEL_WIDTH),
        message,
        " ".repeat(term_width - LABEL_WIDTH - message.len() - 1),
    );
    stdout().flush().unwrap();
}

pub fn print_plain<S: Into<String>>(colour: Option<Color>, message: S) {
    let term_width = get_term_width();
    let message = shorten(message.into(), term_width - LABEL_WIDTH - 1);
    let message_len = message.len();

    let message = match colour {
        Some(colour) => Style::new(colour).bold().paint(message),
        None => Paint::new(message),
    };

    print!("{}{}\r", message, " ".repeat(term_width - message_len),);
    stdout().flush().unwrap();
}

/// Shortens a message by omitting the middle part and replacing it with '...'
///
/// If the given message is shorter than the available width, the
/// original message will be returned
fn shorten(message: String, max_width: usize) -> String {
    let len = message.len();

    if len <= max_width {
        return message;
    }

    let break_index = max_width / 4;

    return [
        message.chars().take(break_index).collect(),
        "...".to_owned(),
        message
            .chars()
            .skip(len - max_width + break_index + 3)
            .collect(),
    ]
    .join("");
}

fn get_term_width() -> usize {
    if let Some((width, _)) = term_size::dimensions() {
        width
    } else {
        80
    }
}
