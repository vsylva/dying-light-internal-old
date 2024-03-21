use super::game_di::GameDI;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct CGame {
    __: [u8; 0x98],
    pub(crate) game_di_p: *mut GameDI,
    ___: [u8; 0x28],
    pub(crate) screen_width: i32,
    pub(crate) screen_height: i32,
}
