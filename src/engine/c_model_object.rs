use super::model_object::ModelObject;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct CModelObject {
    __: [u8; 0x11C],
    pub(crate) x: f32,
    ___: [u8; 0xC],
    pub(crate) y: f32,
    ____: [u8; 0xC],
    pub(crate) z: f32,
    _____: [u8; 0x1F0],
    pub(crate) active: i32,
    ______: [u8; 0xC],
    pub(crate) flag: i32,
    _______: [u8; 0x74],
    pub(crate) model_obj_p: *mut ModelObject,
    ________: [u8; 0x10],
    pub(crate) name_p: *const i8,
}

impl CModelObject {
    pub(crate) unsafe fn is_pos_empty(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }
}
