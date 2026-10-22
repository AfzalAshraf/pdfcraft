//! Hand-pan geometry, independent of egui so invalid pointer values can be tested directly.

/// Move the viewport opposite to the pointer and clamp both axes to the content. Reject bad
/// input instead of storing NaN in ScrollArea state. Finite subtraction may overflow to infinity;
/// clamping that overshoot to a finite boundary is intentional and cannot panic.
pub(super) fn offset(previous: [f32; 2], moved: [f32; 2], maximum: [f32; 2]) -> Option<[f32; 2]> {
    if !previous.into_iter().chain(moved).chain(maximum).all(f32::is_finite) || maximum.into_iter().any(|v| v < 0.0) {
        return None;
    }
    Some(std::array::from_fn(|axis| (previous[axis] - moved[axis]).clamp(0.0, maximum[axis])))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_axes_follow_the_pointer_one_for_one() {
        assert_eq!(offset([100.0, 300.0], [-40.0, -150.0], [1000.0, 1000.0]), Some([140.0, 450.0]));
        assert_eq!(offset([140.0, 450.0], [25.0, 60.0], [1000.0, 1000.0]), Some([115.0, 390.0]));
    }

    #[test]
    fn bounds_and_fitting_pages_do_not_spring_back() {
        assert_eq!(offset([10.0, 20.0], [80.0, -500.0], [100.0, 200.0]), Some([0.0, 200.0]));
        assert_eq!(offset([0.0, 20.0], [-50.0, -50.0], [0.0, 200.0]), Some([0.0, 70.0]));
        assert_eq!(offset([0.0, 0.0], [-50.0, -50.0], [0.0, 0.0]), Some([0.0, 0.0]));
    }

    #[test]
    fn stationary_pointer_has_no_residual_movement() {
        assert_eq!(offset([100.0, 200.0], [0.0, 0.0], [500.0, 500.0]), Some([100.0, 200.0]));
    }

    #[test]
    fn nonfinite_input_never_enters_scroll_state() {
        for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            for axis in 0..2 {
                let mut invalid = [50.0; 2];
                invalid[axis] = bad;
                assert_eq!(offset(invalid, [0.0; 2], [100.0; 2]), None);
                assert_eq!(offset([50.0; 2], invalid, [100.0; 2]), None);
                assert_eq!(offset([50.0; 2], [0.0; 2], invalid), None);
            }
        }
        assert_eq!(offset([0.0; 2], [0.0; 2], [-1.0, 10.0]), None);
    }

    #[test]
    fn finite_overflow_is_a_bounded_overshoot() {
        assert_eq!(offset([f32::MAX; 2], [-f32::MAX; 2], [100.0; 2]), Some([100.0; 2]));
        assert_eq!(offset([-f32::MAX; 2], [f32::MAX; 2], [100.0; 2]), Some([0.0; 2]));
    }
}
