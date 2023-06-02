#[derive(Debug)]
pub(crate) enum Error {
    Internal(String),
    Env(std::env::VarError),
    SystemTime(std::time::SystemTimeError),
}