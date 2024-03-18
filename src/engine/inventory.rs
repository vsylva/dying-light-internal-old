#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct Inventory {
    pub(crate) id: u32,
    pub(crate) count: u32,
    __: [u8; 8],
}
