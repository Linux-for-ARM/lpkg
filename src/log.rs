//! Logging (print messages to `stderr`/`stdout`, write messages to log file)

pub fn set_colors(is_color: bool) {
    colored::control::set_override(is_color);
}

#[macro_export]
macro_rules! msg {
    ($($arg:tt)*) => {{
        use $crate::consts::LOG_FILE_PATH;
        use $crate::fs::write_append;

        use colored::Colorize;

        println!("{} {}", "==>".bold().blue(), format_args!($($arg)*));
        let _ = write_append(LOG_FILE_PATH, format!("{}\n", format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! msg2 {
    ($($arg:tt)*) => {{
        use $crate::consts::LOG_FILE_PATH;
        use $crate::fs::write_append;

        use colored::Colorize;

        println!("{} {}", " ->".bold().magenta(), format_args!($($arg)*));
        let _ = write_append(LOG_FILE_PATH, format!("    {}\n", format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! ok_msg {
    ($($arg:tt)*) => {{
        use $crate::consts::LOG_FILE_PATH;
        use $crate::fs::write_append;

        use colored::Colorize;

        println!("{}{}{} {}", "[".bold(), "✓".bold().green(), "]".bold(), format_args!($($arg)*));
        let _ = write_append(LOG_FILE_PATH, format!("OK: {}\n", format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! err_msg {
    ($($arg:tt)*) => {{
        use $crate::consts::LOG_FILE_PATH;
        use $crate::fs::write_append;

        use colored::Colorize;

        println!("{}{}{} {}", "[".bold(), "E".bold().red(), "]".bold(), format_args!($($arg)*));
        let _ = write_append(LOG_FILE_PATH, format!("ERROR: {}\n", format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! dbg_msg {
    ($($arg:tt)*) => {{
        use $crate::consts::LOG_FILE_PATH;
        use $crate::fs::write_append;

        use std::env::var;
        use colored::Colorize;

        let is_print = var("LPKG_DBG").unwrap_or("n".to_string());

        if &is_print == "y" || &is_print == "Y" {
            println!("{}{}{} {}", "[".bold().dimmed(), "DEBUG".bold().dimmed(), "]".dimmed().bold(), format_args!($($arg)*));
        }

        let _ = write_append(LOG_FILE_PATH, format!("DEBUG: {}\n", format_args!($($arg)*)));
    }};
}
