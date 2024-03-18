use super::camera_fpp_di::CameraFPPDI;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct CameraManagerDI {
    __: [u8; 0x50],
    pub(crate) camera_p: *mut CameraFPPDI,
}
