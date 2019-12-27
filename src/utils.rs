use crate::geometry::vec3::{Vec3};

// pub fn limit_color(color: Vec3) -> (u8, u8, u8) {
//     let max = color.x.max(color.y.max(color.z));

//     if max > 255. {
//         let normalized_color = color.scale(255. / max);
//         (
//             normalized_color.x as u8,
//             normalized_color.y as u8,
//             normalized_color.z as u8,
//         )
//     } else {
//         (color.x as u8, color.y as u8, color.z as u8)
//     }   
// }

pub fn limit_color(color: Vec3) -> (u8, u8, u8) {
    let r = if color.x > 255. { 255 as u8 } else { color.x as u8 };
    let g = if color.y > 255. { 255 as u8 } else { color.y as u8 };
    let b = if color.z > 255. { 255 as u8 } else { color.z as u8 };

    (r, g ,b)
}

// #[test]
// fn test_limit_color() {
//     let color = (122., 0., 255.);
//     let color_expected = (122 as u8, 0 as u8, 255 as u8);

//     assert_eq!(color_expected, limit_color(color));

//     let color2 = (266., 12., 228.);
//     let color_expected2 = (11 as u8, 12 as u8, 228 as u8);

//     assert_eq!(color_expected2, limit_color(color2));
// }