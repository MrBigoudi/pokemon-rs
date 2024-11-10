#[derive(Debug)]
pub enum ErrorCode {
    I0,
    Wgpu,
    Winit,
    Web,
    Unknown,
    SyncError,
    NotInitialized,
    AlreadyInitialized,
}
