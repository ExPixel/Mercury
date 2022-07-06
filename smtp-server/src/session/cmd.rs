#[derive(Debug)]
pub enum Command {
    EHLO { domain: String },
}
