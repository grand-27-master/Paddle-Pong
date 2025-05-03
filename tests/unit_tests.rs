use pong_game::model::utils::{clamp, rects_intersect};
use ggez::graphics::Rect;

#[test]
fn clamp_within_bounds() {
    assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
}

#[test]
fn clamp_below_min() {
    assert_eq!(clamp(-3.5, 0.0, 10.0), 0.0);
}

#[test]
fn clamp_above_max() {
    assert_eq!(clamp(42.0, 0.0, 10.0), 10.0);
}

#[test]
fn rects_intersect_true_when_overlapping() {
    let a = Rect::new(0.0, 0.0, 10.0, 10.0);
    let b = Rect::new(5.0, 5.0, 10.0, 10.0);
    assert!(rects_intersect(a, b));
}

#[test]
fn rects_intersect_false_when_separated() {
    let a = Rect::new(0.0, 0.0, 10.0, 10.0);
    let b = Rect::new(20.0, 20.0, 5.0, 5.0);
    assert!(!rects_intersect(a, b));
}
