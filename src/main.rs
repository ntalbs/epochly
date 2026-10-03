use rustyline::Editor;
use rustyline::history::FileHistory;
use std::env;
use std::path::PathBuf;
use time::format_description::FormatItem;
use time::macros::format_description;
use time::{OffsetDateTime, PrimitiveDateTime};

static FORMATS: &[&[FormatItem<'static>]] = &[
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"),
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second][offset_hour]"),
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond][offset_hour]"),
    format_description!("[year]/[month]/[day] [hour]:[minute]:[second]"),
    format_description!("[year]/[month]/[day] [hour]:[minute]:[second].[subsecond]"),
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z"),
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]Z"),
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour]"),
    format_description!(
        "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour]:[offset_minute]"
    ),
];

fn parse_any(s: &str) -> Option<OffsetDateTime> {
    for fmt in FORMATS {
        if let Ok(dt) = OffsetDateTime::parse(s, fmt) {
            return Some(dt);
        }
        if let Ok(dt) = PrimitiveDateTime::parse(s, fmt) {
            return Some(dt.assume_utc());
        }
    }
    None
}

fn epoch_millis(dt: OffsetDateTime) -> i128 {
    dt.unix_timestamp_nanos() / 1_000_000
}

const HISTORY_FILE: &str = ".epochly_history";

fn history_path() -> PathBuf {
    let home_dir = env::var("HOME").map(PathBuf::from).ok().unwrap();
    home_dir.join(HISTORY_FILE)
}

fn main() {
    println!("Enter date-time string to get Epoch millis.");

    let mut editor = Editor::<(), FileHistory>::new().unwrap();
    if editor.load_history(&history_path()).is_err() {
        // No history file yet - this is fine
    }

    while let Ok(input) = editor.readline("> ") {
        let input = input.trim();
        editor.add_history_entry(input).unwrap();

        if input.is_empty() {
            continue;
        }

        match parse_any(input) {
            Some(dt) => println!("  {} ({})", epoch_millis(dt), dt.offset()),
            None => eprintln!("Unidentified date-time format. {}", input),
        }
    }

    editor.save_history(&history_path()).unwrap();
}
