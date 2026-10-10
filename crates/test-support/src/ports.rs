//! One claim of a loopback port, shared by every test that hands a port
//! number to another process or task.
//!
//! A port bound to 0, read and released is taken again by another test often
//! enough to fail a run when many start at once. A claimed number lies below
//! the kernel's ephemeral range, so no bind to port 0 is ever handed it, and
//! an advisory lock on `target/test-ports/{number}` holds it against every
//! other test of this workspace for as long as the claim lives. The lock file
//! is never unlinked.

use std::fs::{File, OpenOptions};
use std::hash::{BuildHasher, Hasher};
use std::net::TcpListener;
use std::ops::Range;
use std::path::{Path, PathBuf};

/// A loopback port claimed for one test.
pub struct PortClaim {
    pub number: u16,
    _lock: File,
}

const CLAIMED_PORTS: Range<u16> = 20_000..32_000;
/// The harness's own registered ports inside that range.
const HARNESS_PORTS: [u16; 2] = [24_317, 24_318];

// The repository root is read from this crate's compile-time manifest
// directory, two levels below it; nothing is read from the environment at run
// time, so a test under a cleared environment finds the same directory.
fn claims_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("the crate sits two levels under the workspace root")
        .join("target")
        .join("test-ports")
}

fn random_word() -> u64 {
    std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish()
}

/// Claims a port nothing holds: locked against the workspace's other tests
/// and shown bindable at the moment of the claim. Panics when none can be
/// claimed.
pub fn claim_port() -> PortClaim {
    let claims = claims_dir();
    std::fs::create_dir_all(&claims).expect("the port claims directory");
    let span = u64::from(CLAIMED_PORTS.end - CLAIMED_PORTS.start);
    for _ in 0..10_000 {
        let offset = u16::try_from(random_word() % span).expect("an offset inside the range");
        let number = CLAIMED_PORTS.start + offset;
        if HARNESS_PORTS.contains(&number) {
            continue;
        }
        let Ok(lock) = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(claims.join(number.to_string()))
        else {
            continue;
        };
        if lock.try_lock().is_ok() && TcpListener::bind(("127.0.0.1", number)).is_ok() {
            return PortClaim {
                number,
                _lock: lock,
            };
        }
    }
    panic!("no loopback port could be claimed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_claimed_number_lies_in_the_range_and_is_no_harness_port() {
        for _ in 0..64 {
            let claim = claim_port();
            assert!(CLAIMED_PORTS.contains(&claim.number), "{}", claim.number);
            assert!(!HARNESS_PORTS.contains(&claim.number), "{}", claim.number);
        }
    }

    #[test]
    fn two_live_claims_differ() {
        let claims: Vec<PortClaim> = (0..32).map(|_| claim_port()).collect();
        let mut numbers: Vec<u16> = claims.iter().map(|claim| claim.number).collect();
        numbers.sort_unstable();
        numbers.dedup();
        assert_eq!(numbers.len(), claims.len(), "every live claim is its own");
    }

    #[test]
    fn the_lock_file_of_a_live_claim_cannot_be_locked_again() {
        let claim = claim_port();
        let path = claims_dir().join(claim.number.to_string());
        let again = OpenOptions::new()
            .write(true)
            .open(&path)
            .expect("the claim's lock file is in the claims directory");
        assert!(
            again.try_lock().is_err(),
            "a second lock is refused while the claim lives"
        );

        drop(claim);
        assert!(again.try_lock().is_ok(), "and granted once it is dropped");
    }

    #[test]
    fn a_claimed_port_binds_on_loopback() {
        let claim = claim_port();
        assert!(TcpListener::bind(("127.0.0.1", claim.number)).is_ok());
    }
}
