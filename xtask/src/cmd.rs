use duct::cmd;
use std::{ffi::OsString, fmt, io};

/// Creates a [`DisplayCmd`] instance.
///
/// Arguments must implement `Into<`[`OsString`]`>`.
macro_rules! display_cmd {
    ($name:expr $(, $arg:expr)* $(,)?) => {
        $crate::cmd::DisplayCmd::new($name)$(.arg($arg))*
    };
}

/// A representation of a shell command that implements
/// [`Display`][fmt::Display].
#[derive(Clone)]
pub(crate) struct DisplayCmd {
    name: OsString,
    args: Vec<OsString>,
}

impl DisplayCmd {
    pub(crate) fn new(name: impl Into<OsString>) -> Self {
        Self {
            name: name.into(),
            args: Vec::new(),
        }
    }

    pub(crate) fn arg(mut self, arg: impl Into<OsString>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub(crate) fn args<I, A>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<OsString>,
    {
        self.args.extend(args.into_iter().map(|arg| arg.into()));
        self
    }

    /// Prepend `name` to this command.
    ///
    /// Essentially, this funtion sets the command name to `name`,
    /// makes the old command name into the first argument, and pushes
    /// every other argument forward by one.
    pub(crate) fn prepend(self, name: impl Into<OsString>) -> Self {
        let args = std::iter::once(self.name)
            .chain(self.args.into_iter())
            .collect();

        Self {
            name: name.into(),
            args,
        }
    }

    /// Runs this command.
    ///
    /// Returns [`io::Error`] if an error occurs when trying to run
    /// the command or if the command runs, but exits with a non-zero
    /// exit code.
    pub(crate) fn run(&self) -> io::Result<()> {
        cmd(&self.name, &self.args).run().map(|_| ())
    }
}

impl fmt::Display for DisplayCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.to_string_lossy().fmt(f)?;
        self.args
            .iter()
            .map(|arg| {
                let arg = arg.to_string_lossy();
                // Quote any arguments that contain spaces
                if arg.contains(' ') {
                    write!(f, " '{}'", arg)
                } else {
                    write!(f, " {}", arg)
                }
            })
            .collect()
    }
}
