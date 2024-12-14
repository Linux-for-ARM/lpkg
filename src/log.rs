//! Logging (print messages to `stderr`/`stdout`, write messages to log file)

pub fn set_colors(is_color: bool) {
    colored::control::set_override(is_color);
}

#[macro_export]
macro_rules! msg {
    ($($arg:tt)*) => {{
        use ::lpkg::consts::LOG_FILE_PATH;
        use ::lpkg::error::*;
        use ::lpkg::fs::write_append;

        use colored::Colorize;

        println!("{} {}", "==>".bold().blue(), format_args!($($arg)*));
        let _ = write_append(LOG_FILE_PATH, format!("{}", format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! msg2 {
    ($($arg:tt)*) => {{
        use ::lpkg::consts::LOG_FILE_PATH;
        use ::lpkg::error::*;
        use ::lpkg::fs::write_append;

        use colored::Colorize;

        println!("{} {}", " ->".bold().magenta(), format_args!($($arg)*));
        let _ = write_append(LOG_FILE_PATH, format!("    {}", format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! ok_msg {
    ($($arg:tt)*) => {{
        use ::lpkg::consts::LOG_FILE_PATH;
        use ::lpkg::error::*;
        use ::lpkg::fs::write_append;

        use colored::Colorize;

        println!("{}{}{} {}", "[".bold(), "✓".bold().green(), "]".bold(), format_args!($($arg)*));
        let _ = write_append(LOG_FILE_PATH, format!("OK: {}", format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! err_msg {
    ($($arg:tt)*) => {{
        use ::lpkg::consts::LOG_FILE_PATH;
        use ::lpkg::error::*;
        use ::lpkg::fs::write_append;

        use colored::Colorize;

        println!("{}{}{} {}", "[".bold(), "E".bold().red(), "]".bold(), format_args!($($arg)*));
        let _ = write_append(LOG_FILE_PATH, format!("ERROR: {}", format_args!($($arg)*)));
    }};
}
