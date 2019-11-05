pub fn limit_color(color: (f32, f32, f32)) {
    let r = if color.0 > 255. { 255. } else if color.0 < 0. { 0. } else { color.0 };
    let g = if color.1 > 255. { 255. } else if color.1 < 0. { 0. } else { color.1 };
    let b = if color.2 > 255. { 255. } else if color.2 < 0. { 0. } else { color.2 };

    (r, g, b);
}