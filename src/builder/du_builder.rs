use crate::{
    cl_parser::parser::CliOption,
    core::du::{DuCommand, DuDefault},
};

pub struct DuBuilder {}

impl DuBuilder {
    pub fn build_and_run(options: CliOption) {
        if options.file.is_dir() {
            let d = DuDefault::default();
            d.run(options).ok();
        }
    }
}
