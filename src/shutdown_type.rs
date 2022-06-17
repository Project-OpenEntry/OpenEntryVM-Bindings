
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ShutdownType {
    None       = 0,
    Gracefully = 1,
    Restarting = 2,
    Error      = 3,
}