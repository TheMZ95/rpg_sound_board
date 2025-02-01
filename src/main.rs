mod sounds;

use clap::Parser;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, event, execute};
use lazy_static::lazy_static;
use soloud::Soloud;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// the path to the sound mapping file
    #[arg(short, long, required = true)]
    sound_mapping_file: String,

    /// enable verbose mode
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

lazy_static! {
    static ref RUNNING: AtomicBool = AtomicBool::new(true);
}

fn main() {
    let args = Args::parse();

    sounds::config::handle_sound_mappings(args.sound_mapping_file);
    if args.verbose {
        // enable verbose mode
    }

    ctrlc::set_handler(move || {
        println!("\nSIGINT received! Shutting down gracefully...");
        // Only the signal handler is allowed to set the RUNNING variable
        RUNNING.store(false, Ordering::SeqCst);
    })
    .expect("Error setting SIGINT handler");

    //sounds::player::play_sound();
    enable_raw_mode().expect("Failed to enable raw mode");
    let mut stdout = std::io::stdout();
    execute!(stdout, cursor::MoveToNextLine(1)).expect("Failed to move cursor");

    let mut sl = Soloud::default().unwrap();

    while is_running() {
        // Simulate some work
        if event::poll(std::time::Duration::from_secs(1)).expect("Error polling events") {
            if let Event::Key(key_event) = event::read().expect("Error reading events") {
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Esc, _) => {
                        println!("ESC key pressed, exiting...");
                        break;
                    }
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        RUNNING.store(false, Ordering::SeqCst);
                        println!("Exiting...");
                    }
                    (KeyCode::Char('f'), _) => {
                        sounds::player::play_sound_once(&sl, "Feuer 01.mp3");
                    }
                    (KeyCode::Char(c), _) => {
                        println!("Key pressed: '{}'", c);
                    }
                    (KeyCode::Enter, _) => {
                        println!("Enter key pressed");
                    }
                    (KeyCode::Backspace, _) => {
                        println!("Backspace pressed");
                    }
                    (_, _) => {
                        println!("Other key pressed");
                    }
                }
            }
        }
    }

    disable_raw_mode().expect("Failed to disable raw mode");
}
pub fn is_running() -> bool {
    RUNNING.load(Ordering::SeqCst)
}
