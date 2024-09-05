use crate::model;
pub type Result<T> = core::result::Result<T,E>;
#[derive(Debug)]
pub enum Error{
    // Config
    ConfigMissingEnv(&'static str),
    // Modules
    Model(model::Error),
}