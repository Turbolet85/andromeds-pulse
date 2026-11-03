//! Test support for the workspace's tests, taken as a dev-dependency only.
//!
//! `otlp` builds the spans, metric points and log records a test sends or
//! hands to the buffer. `ports` claims a loopback port for a test that hands
//! a port number to another process or task.

pub mod otlp;
pub mod ports;
