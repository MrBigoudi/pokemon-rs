#[derive(Debug, PartialEq, Eq)]
pub enum ErrorCode {
    IO,
    Wgpu,
    Winit,
    Web,
    Unknown,
    SyncError,
    NotInitialized,
    AlreadyInitialized,
    Network,
    Duplicate,
    BadValue,
    NotFound,
}
