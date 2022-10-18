use std::fmt;
use std::io::Write;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

pub trait Log {
    fn error(&mut self) -> LogStream;
    fn red(&mut self) -> LogStream;
}

impl Log for StandardStream {
    fn error(&mut self) -> LogStream {
        let mut color = ColorSpec::new();
        color.set_fg(Some(Color::Red)).set_bold(true);
        let _ = self.set_color(&color);
        let _ = write!(self, "error:");
        let _ = self.reset();
        let _ = write!(self, " ");
        LogStream(self)
    }

    fn red(&mut self) -> LogStream {
        let mut color = ColorSpec::new();
        color.set_fg(Some(Color::Red));
        let _ = self.set_color(&color);
        LogStream(self)
    }
}

pub struct LogStream<'a>(&'a mut StandardStream);

impl<'a> LogStream<'a> {
    pub fn write_fmt(&mut self, args: fmt::Arguments) {
        let _ = self.0.write_fmt(args);
    }
}

impl<'a> Drop for LogStream<'a> {
    fn drop(&mut self) {
        let _ = self.0.reset();
    }
}
