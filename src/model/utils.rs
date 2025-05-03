use ggez::graphics::Rect;

/// Axis-aligned box intersection
pub fn rects_intersect(a: Rect, b: Rect) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x &&
    a.y < b.y + b.h && a.y + a.h > b.y
}

/// Clamp value between min and max
pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
    val.max(min).min(max)
}
