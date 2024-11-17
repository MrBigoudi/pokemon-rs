#[derive(Debug)]
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
}
