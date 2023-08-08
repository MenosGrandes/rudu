use std::{
    fs::{self, DirEntry},
    io,
    path::Path, env,
};

use crate::cl_parser::parser::CliOption;

pub enum DuError {
    E1,
    E2,
}
pub trait DuCommand {
    fn run(self, options: CliOption) -> Result<(), DuError>;
}

pub struct DuDefault {
    depth: u8,
}
impl Default for DuDefault {
    fn default() -> Self {
        Self { depth: 0 }
    }
}
impl DuDefault {
    fn visit(&self, dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self.visit(&path, cb)?;
                } else {
                    cb(&entry);
                }
            }
        }
        Ok(())
    }
    pub fn run(self, options: CliOption) -> Result<(), DuError> {
        let _ = self.visit(options.file.as_path(), &|file| {
            println!("{:?} {:?}", file.metadata().unwrap().len(), file.path().strip_prefix(env::current_dir().unwrap()));
        });

        Ok(())
    }
}
