use super::player_di::PlayerDI;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct LocalClientDI {
    __: [u8; 80],
    pub(crate) player_di_p: *mut PlayerDI,
}
