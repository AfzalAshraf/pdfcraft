//! Destination policy shared by the print dialog and its submission entry point.
//! A background refresh must never reroute a job; a missing named queue is not Save as PDF.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DestinationError {
    Loading,
    Missing,
}

/// Pick a default only before a destination is resolved. `picked` also represents an explicit
/// Save as PDF (`None`), which must survive refreshes and reopening the dialog.
pub(super) fn choose(current: Option<String>, picked: bool, default: Option<String>) -> Option<String> {
    if picked { current } else { current.or(default) }
}

/// A named destination is always explicit: never turn it into another queue or a file when it
/// disappears. A cached, named queue can be used while discovery refreshes; the native spooler
/// must still validate that exact destination. Only an unresolved initial selection waits.
pub(super) fn validate<'a>(
    current: Option<&str>,
    picked: bool,
    loading: bool,
    mut known: impl Iterator<Item = &'a str>,
) -> Result<(), DestinationError> {
    match current {
        Some(name) if !known.any(|p| p == name) => Err(DestinationError::Missing),
        None if loading && !picked => Err(DestinationError::Loading),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_discovery_can_pick_a_default() {
        assert_eq!(choose(None, false, Some("Office".into())).as_deref(), Some("Office"));
        assert_eq!(choose(None, false, None), None);
    }

    #[test]
    fn a_refresh_cannot_replace_a_cached_queue_with_a_new_default() {
        for picked in [false, true] {
            assert_eq!(choose(Some("Private".into()), picked, Some("Public".into())).as_deref(), Some("Private"));
        }
    }

    #[test]
    fn disappearing_queue_stays_named_and_is_refused_instead_of_rerouted() {
        let selected = choose(Some("Private".into()), false, Some("Public".into()));
        assert_eq!(selected.as_deref(), Some("Private"));
        for loading in [false, true] {
            assert_eq!(validate(selected.as_deref(), false, loading, ["Public"].into_iter()), Err(DestinationError::Missing));
            assert_eq!(validate(selected.as_deref(), true, loading, std::iter::empty()), Err(DestinationError::Missing));
        }
    }

    #[test]
    fn explicit_save_as_pdf_survives_reopening_and_discovery() {
        assert_eq!(choose(None, true, Some("Office".into())), None);
        for loading in [false, true] {
            assert_eq!(validate(None, true, loading, ["Office"].into_iter()), Ok(()));
        }
    }

    #[test]
    fn unresolved_loading_is_not_permission_to_save_a_pdf() {
        assert_eq!(validate(None, false, true, std::iter::empty()), Err(DestinationError::Loading));
        assert_eq!(validate(None, false, false, std::iter::empty()), Ok(()));
    }

    #[test]
    fn existing_named_queue_can_print_during_refresh() {
        for loading in [false, true] {
            assert_eq!(validate(Some("Office"), false, loading, ["Office"].into_iter()), Ok(()));
        }
    }

    #[test]
    fn queue_identity_is_exact_and_supports_unicode() {
        assert_eq!(validate(Some("مكتب"), true, false, ["مكتب"].into_iter()), Ok(()));
        assert_eq!(validate(Some("Office"), true, false, ["office"].into_iter()), Err(DestinationError::Missing));
    }
}
