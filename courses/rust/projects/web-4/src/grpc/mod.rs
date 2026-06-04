pub mod service;
pub mod client;

pub use service::TaskServiceImpl;

/// The generated protobuf/gRPC code is included here.
pub mod proto {
    tonic::include_proto!("taskforge");
}
