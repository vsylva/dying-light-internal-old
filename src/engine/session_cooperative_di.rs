use super::{
    camera_manager_di::CameraManagerDI,
    level_di::LevelDI,
    local_client_di::LocalClientDI,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct SessionCooperativeDI {
    __: [u8; 0xB0],
    pub(crate) level_di_p: *mut LevelDI,
    pub(crate) local_client_di_p: *mut LocalClientDI,
    pub(crate) camera_manager_p: *mut CameraManagerDI,
}
