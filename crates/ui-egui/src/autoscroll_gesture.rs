//! Distinguish click-to-latch auto-scroll from hold-and-move auto-scroll.
//! Kept independent of egui so release behaviour can be checked without a window or renderer.

pub(crate) const DEAD_ZONE: f32 = 15.0;

#[derive(Default)]
pub(crate) struct MiddleGesture {
    held: bool,
    moved: bool,
}

impl MiddleGesture {
    pub(crate) fn start(&mut self) {
        self.held = true;
        self.moved = false;
    }

    /// True when the activating hold ends after movement outside the dead zone. Once a click
    /// has latched, later pointer movement must not turn it back into a held gesture.
    pub(crate) fn release_cancels(&mut self, distance: f32, down: bool, released: bool) -> bool {
        if !self.held {
            return false;
        }
        self.moved |= distance.is_finite() && distance > DEAD_ZONE;
        if released || !down {
            self.held = false;
            return self.moved;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_click_latches_and_later_movement_does_not_cancel_it() {
        let mut gesture = MiddleGesture::default();
        gesture.start();
        assert!(!gesture.release_cancels(0.0, true, false));
        assert!(!gesture.release_cancels(2.0, false, true));
        assert!(!gesture.release_cancels(100.0, false, false));
    }

    #[test]
    fn a_held_drag_stops_on_release_even_after_returning_to_the_anchor() {
        let mut gesture = MiddleGesture::default();
        gesture.start();
        assert!(!gesture.release_cancels(100.0, true, false));
        assert!(!gesture.release_cancels(0.0, true, false));
        assert!(gesture.release_cancels(0.0, false, true));
        assert!(!gesture.release_cancels(100.0, false, false));
    }

    #[test]
    fn movement_in_the_release_frame_counts_and_a_new_click_resets_the_drag() {
        let mut gesture = MiddleGesture::default();
        gesture.start();
        assert!(gesture.release_cancels(30.0, false, true));
        gesture.start();
        assert!(!gesture.release_cancels(0.0, false, true));
    }

    #[test]
    fn jitter_is_not_a_drag_and_a_missing_release_still_ends_a_hold() {
        let mut gesture = MiddleGesture::default();
        gesture.start();
        assert!(!gesture.release_cancels(DEAD_ZONE, true, false));
        assert!(!gesture.release_cancels(f32::NAN, true, false));
        assert!(!gesture.release_cancels(f32::INFINITY, false, true));
        gesture.start();
        assert!(!gesture.release_cancels(30.0, true, false));
        assert!(gesture.release_cancels(30.0, false, false));
    }
}
