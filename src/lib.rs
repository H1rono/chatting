pub mod grpc {
    pub mod id {
        tonic::include_proto!("chatting.id");
    }

    pub mod user {
        tonic::include_proto!("chatting.user");
    }
}

pub mod prelude;
pub mod router;
pub mod user;
