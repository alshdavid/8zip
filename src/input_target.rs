use std::{
    io::{IsTerminal, Read},
    path::PathBuf,
};

pub enum InputTarget {
    Path(PathBuf),
    Stdin(Box<dyn Read>),
}

impl InputTarget {
    pub fn detect(input: Option<PathBuf>) -> anyhow::Result<Self> {
        Ok(match input {
            Some(path) if path.to_str() == Some("-") => Self::Stdin(Box::new(std::io::stdin())),
            Some(path) => Self::Path(path),
            None => {
                if !std::io::stdin().is_terminal() {
                    Self::Stdin(Box::new(std::io::stdin()))
                } else {
                    anyhow::bail!("No input file specified and stdin is not piped");
                }
            }
        })
    }
}
