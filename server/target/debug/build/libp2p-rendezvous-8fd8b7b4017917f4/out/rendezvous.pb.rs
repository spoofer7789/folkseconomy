#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(enumeration = "message::MessageType", optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub register: ::core::option::Option<message::Register>,
    #[prost(message, optional, tag = "3")]
    pub register_response: ::core::option::Option<message::RegisterResponse>,
    #[prost(message, optional, tag = "4")]
    pub unregister: ::core::option::Option<message::Unregister>,
    #[prost(message, optional, tag = "5")]
    pub discover: ::core::option::Option<message::Discover>,
    #[prost(message, optional, tag = "6")]
    pub discover_response: ::core::option::Option<message::DiscoverResponse>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Register {
        #[prost(string, optional, tag = "1")]
        pub ns: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub signed_peer_record: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        /// in seconds
        #[prost(uint64, optional, tag = "3")]
        pub ttl: ::core::option::Option<u64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RegisterResponse {
        #[prost(enumeration = "ResponseStatus", optional, tag = "1")]
        pub status: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub status_text: ::core::option::Option<::prost::alloc::string::String>,
        /// in seconds
        #[prost(uint64, optional, tag = "3")]
        pub ttl: ::core::option::Option<u64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Unregister {
        #[prost(string, optional, tag = "1")]
        pub ns: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Discover {
        #[prost(string, optional, tag = "1")]
        pub ns: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint64, optional, tag = "2")]
        pub limit: ::core::option::Option<u64>,
        #[prost(bytes = "vec", optional, tag = "3")]
        pub cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DiscoverResponse {
        #[prost(message, repeated, tag = "1")]
        pub registrations: ::prost::alloc::vec::Vec<Register>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(enumeration = "ResponseStatus", optional, tag = "3")]
        pub status: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "4")]
        pub status_text: ::core::option::Option<::prost::alloc::string::String>,
    }
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
    pub enum MessageType {
        Register = 0,
        RegisterResponse = 1,
        Unregister = 2,
        Discover = 3,
        DiscoverResponse = 4,
    }
    impl MessageType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MessageType::Register => "REGISTER",
                MessageType::RegisterResponse => "REGISTER_RESPONSE",
                MessageType::Unregister => "UNREGISTER",
                MessageType::Discover => "DISCOVER",
                MessageType::DiscoverResponse => "DISCOVER_RESPONSE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REGISTER" => Some(Self::Register),
                "REGISTER_RESPONSE" => Some(Self::RegisterResponse),
                "UNREGISTER" => Some(Self::Unregister),
                "DISCOVER" => Some(Self::Discover),
                "DISCOVER_RESPONSE" => Some(Self::DiscoverResponse),
                _ => None,
            }
        }
    }
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
    pub enum ResponseStatus {
        Ok = 0,
        EInvalidNamespace = 100,
        EInvalidSignedPeerRecord = 101,
        EInvalidTtl = 102,
        EInvalidCookie = 103,
        ENotAuthorized = 200,
        EInternalError = 300,
        EUnavailable = 400,
    }
    impl ResponseStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResponseStatus::Ok => "OK",
                ResponseStatus::EInvalidNamespace => "E_INVALID_NAMESPACE",
                ResponseStatus::EInvalidSignedPeerRecord => {
                    "E_INVALID_SIGNED_PEER_RECORD"
                }
                ResponseStatus::EInvalidTtl => "E_INVALID_TTL",
                ResponseStatus::EInvalidCookie => "E_INVALID_COOKIE",
                ResponseStatus::ENotAuthorized => "E_NOT_AUTHORIZED",
                ResponseStatus::EInternalError => "E_INTERNAL_ERROR",
                ResponseStatus::EUnavailable => "E_UNAVAILABLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OK" => Some(Self::Ok),
                "E_INVALID_NAMESPACE" => Some(Self::EInvalidNamespace),
                "E_INVALID_SIGNED_PEER_RECORD" => Some(Self::EInvalidSignedPeerRecord),
                "E_INVALID_TTL" => Some(Self::EInvalidTtl),
                "E_INVALID_COOKIE" => Some(Self::EInvalidCookie),
                "E_NOT_AUTHORIZED" => Some(Self::ENotAuthorized),
                "E_INTERNAL_ERROR" => Some(Self::EInternalError),
                "E_UNAVAILABLE" => Some(Self::EUnavailable),
                _ => None,
            }
        }
    }
}
