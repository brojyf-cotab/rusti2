pub mod auth;
pub mod config;
pub mod policy;
pub mod service;
pub mod telemetry;

// tonic-generated signatures are outside this crate's control.
#[allow(clippy::result_large_err, clippy::double_must_use)]
pub mod pb {
    tonic::include_proto!("rusti2.v1");
}
