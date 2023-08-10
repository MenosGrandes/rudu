use std::{
    collections::VecDeque,
    env,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
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
struct DirSize {
    pub size: u128,
}
impl DuDefault {
    fn visit(
        &self,
        dir: &Path,
        cb: &dyn Fn(&DirEntry),
        depth: &mut u8,
        max_depth: u8,
        dirs: &mut VecDeque<PathBuf>,
    ) -> io::Result<()> {
        let width = 8;
        let precision = 8;
        let mut file_stack: Vec<PathBuf> = vec![dir.to_path_buf()];
        /*
         *MG sum is not calculated ok.
         * */
        while !file_stack.is_empty() {
            let current_dir = file_stack.pop();
            //println!("{:<width$.precision$?}", current_dir.clone().unwrap().as_path().strip_prefix(dir).unwrap().display());
            let cd = current_dir.clone().unwrap();
            //println!("{:<width$.precision$} | 2222 ", cd.as_path().display());
            if let Ok(files) = fs::read_dir(cd) {
                let mut sum = 0;
                for file in files {
                    if let Ok(file) = file {
                        let path = file.path();
                        if path.is_dir() {
                            file_stack.push(path);
                        } else {
                            cb(&file);
                            sum+=file.metadata().unwrap().len();
                        }
                    }
                }
            println!("{:<width$.precision$?} | {}",sum, current_dir.clone().unwrap().as_path().strip_prefix(dir).unwrap().display());
            }
        }
        Ok(())
    }
    pub fn run(self, options: CliOption) -> Result<(), DuError> {
        let width = 8;
        let precision = 8;
        let mut deq: VecDeque<PathBuf> = VecDeque::new();
        let _ = self.visit(
            options.file.as_path(),
            &|file| {
                println!(
                    "{:<width$.precision$} | {}",
                    file.metadata().unwrap().len(),
                    file.path()
                        .strip_prefix(env::current_dir().unwrap())
                        .unwrap()
                        .display()
                );
            },
            &mut 0,
            options.max_depth,
            &mut deq,
        );

        Ok(())
    }
}
