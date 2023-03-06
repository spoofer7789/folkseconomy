#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HolePunch {
    #[prost(enumeration = "hole_punch::Type", required, tag = "1")]
    pub r#type: i32,
    /// For hole punching, we'll send some additional observed addresses to the remote peer
    /// that could have been filtered by the Host address factory (for example: AutoRelay removes all public addresses if peer has private reachability).
    /// This is a hack!
    /// We plan to have a better address discovery and advertisement mechanism in the future.
    /// See <https://github.com/libp2p/go-libp2p-autonat/pull/98>
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub obs_addrs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `HolePunch`.
pub mod hole_punch {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Connect = 100,
        Sync = 300,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Connect => "CONNECT",
                Type::Sync => "SYNC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONNECT" => Some(Self::Connect),
                "SYNC" => Some(Self::Sync),
                _ => None,
            }
        }
    }
}
