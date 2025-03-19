pub mod harm {
    pub mod controller {
        pub mod v0 {
            tonic::include_proto!("harm.controller.v0");
        }
    }
    pub mod servers {
        pub mod v0 {
            tonic::include_proto!("harm.servers.v0");
        }
    }
}
