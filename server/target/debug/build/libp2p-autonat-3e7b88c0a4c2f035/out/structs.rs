#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(enumeration = "message::MessageType", optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub dial: ::core::option::Option<message::Dial>,
    #[prost(message, optional, tag = "3")]
    pub dial_response: ::core::option::Option<message::DialResponse>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PeerInfo {
        #[prost(bytes = "vec", optional, tag = "1")]
        pub id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub addrs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Dial {
        #[prost(message, optional, tag = "1")]
        pub peer: ::core::option::Option<PeerInfo>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DialResponse {
        #[prost(enumeration = "ResponseStatus", optional, tag = "1")]
        pub status: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub status_text: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bytes = "vec", optional, tag = "3")]
        pub addr: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
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
        Dial = 0,
        DialResponse = 1,
    }
    impl MessageType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MessageType::Dial => "DIAL",
                MessageType::DialResponse => "DIAL_RESPONSE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DIAL" => Some(Self::Dial),
                "DIAL_RESPONSE" => Some(Self::DialResponse),
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
        EDialError = 100,
        EDialRefused = 101,
        EBadRequest = 200,
        EInternalError = 300,
    }
    impl ResponseStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResponseStatus::Ok => "OK",
                ResponseStatus::EDialError => "E_DIAL_ERROR",
                ResponseStatus::EDialRefused => "E_DIAL_REFUSED",
                ResponseStatus::EBadRequest => "E_BAD_REQUEST",
                ResponseStatus::EInternalError => "E_INTERNAL_ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OK" => Some(Self::Ok),
                "E_DIAL_ERROR" => Some(Self::EDialError),
                "E_DIAL_REFUSED" => Some(Self::EDialRefused),
                "E_BAD_REQUEST" => Some(Self::EBadRequest),
                "E_INTERNAL_ERROR" => Some(Self::EInternalError),
                _ => None,
            }
        }
    }
}
