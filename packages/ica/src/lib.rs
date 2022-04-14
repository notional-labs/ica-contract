/// IBC protobuf definitions.
pub mod ibc {
    /// IBC applications.
    pub mod applications {
        /// Interchain accounts support.
        pub mod interchain_accounts {
            pub mod controller {
                pub mod v1 {
                    include!("prost/ibc.applications.interchain_accounts.controller.v1.rs");
                }
            }

            pub mod host {
                pub mod v1 {
                    include!("prost/ibc.applications.interchain_accounts.host.v1.rs");
                }
            }

            pub mod v1 {
                include!("prost/ibc.applications.interchain_accounts.v1.rs");
            }
        }
    }
}