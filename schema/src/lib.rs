pub mod harm {
    pub mod controller {
        pub mod v0 {
            tonic::include_proto!("harm.controller.v0");
        }
    }
    pub mod servers {
        pub mod v0 {
            tonic::include_proto!("harm.servers.v0");

            impl TryInto<String> for ServerConfig {
                type Error = serde_json::Error;

                fn try_into(self) -> Result<String, Self::Error> {
                    Ok(serde_json::to_string(&self)?)
                }
            }
        }
    }
}
