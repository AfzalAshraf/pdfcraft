//! A one-shot discovery worker. The owner retains at most one receiver at a time; a hung
//! native call is not "cancelled" by spawning unbounded replacement threads.

use std::sync::mpsc::{Receiver, sync_channel};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DiscoveryError {
    Stopped,
}

pub(super) fn start<T: Send + 'static>(
    source: impl FnOnce() -> T + Send + 'static,
    wake: impl FnOnce() + Send + 'static,
) -> std::io::Result<Receiver<Result<T, DiscoveryError>>> {
    let (tx, rx) = sync_channel(1);
    std::thread::Builder::new().name("pdfcraft-printer-list".into()).spawn(move || {
        // Last-resort containment: a broken native source must not leave the dialog loading
        // forever. Never display the panic payload, which could include private driver data.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(source)).map_err(|_| DiscoveryError::Stopped);
        let _ = tx.send(result);
        // Repaint on failure too, and after the receiver has gone (a closed app ignores it).
        wake();
    })?;
    Ok(rx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::{RecvTimeoutError, TryRecvError, channel};
    use std::time::Duration;

    const WAIT: Duration = Duration::from_secs(5);

    #[test]
    fn listing_does_not_block_the_caller_and_wakes_after_the_result() {
        let (release, gate) = channel();
        let (woken, wake) = channel();
        let rx = start(
            move || {
                gate.recv_timeout(WAIT).unwrap();
                vec!["Office"]
            },
            move || woken.send(()).unwrap(),
        )
        .unwrap();
        assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
        release.send(()).unwrap();
        wake.recv_timeout(WAIT).unwrap();
        assert_eq!(rx.try_recv(), Ok(Ok(vec!["Office"])));
        assert_eq!(rx.recv_timeout(WAIT), Err(RecvTimeoutError::Disconnected));
    }

    #[test]
    fn failed_source_returns_an_error_and_still_wakes_the_dialog() {
        let (woken, wake) = channel();
        let rx = start(|| -> Vec<String> { panic!("synthetic private source failure") }, move || woken.send(()).unwrap()).unwrap();
        wake.recv_timeout(WAIT).unwrap();
        assert_eq!(rx.try_recv(), Ok(Err(DiscoveryError::Stopped)));
    }

    #[test]
    fn an_empty_success_is_distinct_from_a_failed_source() {
        let rx = start(Vec::<String>::new, || {}).unwrap();
        assert_eq!(rx.recv_timeout(WAIT), Ok(Ok(Vec::new())));
    }

    #[test]
    fn dropping_the_receiver_does_not_crash_the_worker() {
        let (release, gate) = channel();
        let (woken, wake) = channel();
        let rx = start(
            move || {
                gate.recv_timeout(WAIT).unwrap();
                1
            },
            move || woken.send(()).unwrap(),
        )
        .unwrap();
        drop(rx);
        release.send(()).unwrap();
        assert_eq!(wake.recv_timeout(WAIT), Ok(()));
        assert_eq!(wake.recv_timeout(WAIT), Err(RecvTimeoutError::Disconnected));
    }
}
