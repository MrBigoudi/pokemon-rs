use wgpu::{CommandEncoder, SurfaceTexture};

/// The data contained in a frame
/// This is needed to begin the rendering with wgpu state and add commands to it depending on the game state
pub struct FrameData {
    /// The frame buffer to render to
    pub frame_buffer: SurfaceTexture,
    /// The command buffer to submit to
    pub command_buffer: CommandEncoder,
}
