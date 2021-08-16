use duct::cmd;
use std::{ffi::OsString, fmt, io};

macro_rules! display_cmd {
    ($name:expr $(, $arg:expr)* $(,)?) => {
        $crate::cmd::DisplayCmd::new($name)$(.arg($arg))*
    };
}

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

    pub(crate) fn prepend(self, name: impl Into<OsString>) -> Self {
        let args = std::iter::once(self.name)
            .chain(self.args.into_iter())
            .collect();

        Self {
            name: name.into(),
            args,
        }
    }

    pub(crate) fn run(&self) -> io::Result<()> {
        cmd(&self.name, &self.args).run().map(|_| ())
    }
}

impl fmt::Display for DisplayCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.to_string_lossy().fmt(f)?;
        self.args
            .iter()
            .map(|arg| write!(f, " {}", arg.to_string_lossy()))
            .collect()
    }
}
