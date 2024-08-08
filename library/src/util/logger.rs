use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::{Mutex, Once};

pub struct Logger;

static mut LOG_FILE: Option<File> = None;
static INIT: Once = Once::new();
static FILE_LOCK: Mutex<()> = Mutex::new(());

impl Logger {
    // Initialize the logger with a log file path
    pub fn init<P: AsRef<Path>>(path: P) {
        INIT.call_once(|| {
            let file = OpenOptions::new()
                .write(true)
                .create(true) // Create the file if it does not exist
                .truncate(true) // Truncate the file to zero length
                .open(path)
                .expect("Failed to open log file");

            unsafe {
                LOG_FILE = Some(file); // Store the opened log file
            }
        });
    }

    // Log a message that implements fmt::Display
    pub fn log<T: fmt::Display>(message: T) {
        // Log to console
        println!("{}", message);

        // Log to file
        if let Some(ref mut file) = unsafe { LOG_FILE.as_ref() } {
            let _lock = FILE_LOCK.lock().unwrap(); // Lock the mutex for thread safety
            writeln!(file, "{}", message).expect("Failed to write to log file"); // Write the message to the file
        }
    }

    // Log a formatted message using fmt::Arguments
    pub fn log_fmt(args: fmt::Arguments) {
        // Log to console
        println!("{}", args);

        // Log to file
        if let Some(ref mut file) = unsafe { LOG_FILE.as_ref() } {
            let _lock = FILE_LOCK.lock().unwrap(); // Lock the mutex for thread safety
            writeln!(file, "{}", args).expect("Failed to write to log file"); // Write the formatted message to the file
        }
    }
}