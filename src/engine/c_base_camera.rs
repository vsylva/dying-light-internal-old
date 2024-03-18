#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct CBaseCamera {
    __: [u8; 76],
    x: f32,
    ___: [u8; 12],
    y: f32,
    ____: [u8; 12],
    z: f32,
    _____: [u8; 64],
    pub(crate) matrix: [[f32; 4]; 4],
}
