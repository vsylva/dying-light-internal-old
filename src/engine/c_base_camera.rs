#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub(crate) struct CBaseCamera {
    __: [u8; 76],
    pub(crate) x: f32,
    ___: [u8; 12],
    pub(crate) y: f32,
    ____: [u8; 12],
    pub(crate) z: f32,
    _____: [u8; 64],
    pub(crate) matrix: [[f32; 4]; 4],
}

impl CBaseCamera {
    pub(crate) unsafe fn is_pos_empty(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }
}

// use crate::{engine::CGAME_P, Vec2, Vec3};
// impl CBaseCamera {
//      unsafe fn world_to_screen(
//         &mut self,
//         screen_pos: *mut Vec2,
//         entyty_pos: * const Vec3,
//         mut view_w: f32,
//     ) {
//          pub(crate) static mut SIGHT_X: f32 = 0.0;
//          pub(crate) static mut SIGHT_Y: f32 = 0.0;

//         SIGHT_X = CGAME_P.read().window_width as f32 / 2.0;
//         SIGHT_Y = CGAME_P.read().window_height as f32 / 2.0;

//          pub(crate) static mut MATRIX: [[f32; 4]; 4] = [
//             [0.0, 0.0, 0.0, 0.0],
//             [0.0, 0.0, 0.0, 0.0],
//             [0.0, 0.0, 0.0, 0.0],
//             [0.0, 0.0, 0.0, 0.0],
//         ];

//         MATRIX = self.matrix;

//          pub(crate) static mut ENTYTY_POS: Vec3 = Vec3 {
//             x: 0.0,
//             y: 0.0,
//             z: 0.0,
//         };

//         ENTYTY_POS = *entyty_pos;

//         view_w = 1.0 / view_w;

//         (*screen_pos).x = SIGHT_X
//             + (MATRIX[0][0] * ENTYTY_POS.x
//                 + MATRIX[0][1] * ENTYTY_POS.y
//                 + MATRIX[0][2] * ENTYTY_POS.z
//                 + MATRIX[0][3])
//                 * view_w
//                 * SIGHT_X;

//         (*screen_pos).y = SIGHT_Y
//             - (MATRIX[1][0] * ENTYTY_POS.x
//                 + MATRIX[1][1] * ENTYTY_POS.y
//                 + MATRIX[1][2] * ENTYTY_POS.z
//                 + MATRIX[1][3])
//                 * view_w
//                 * SIGHT_Y;
//     }
// }
