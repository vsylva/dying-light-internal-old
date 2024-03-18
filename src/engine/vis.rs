#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct Vis {
    __: [u8; 0x194],
    yaw: f32,
}
