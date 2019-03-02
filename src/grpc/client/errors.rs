use std::fmt;

#[derive(Debug)]
pub enum ClientError {
    GrpcError(tower_grpc::Status),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for ClientError {}

impl From<tower_grpc::Status> for ClientError {
    fn from(err: tower_grpc::Status) -> Self {
        ClientError::GrpcError(err)
    }
}
