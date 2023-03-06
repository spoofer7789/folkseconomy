#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HopMessage {
    #[prost(enumeration = "hop_message::Type", required, tag = "1")]
    pub r#type: i32,
    #[prost(message, optional, tag = "2")]
    pub peer: ::core::option::Option<Peer>,
    #[prost(message, optional, tag = "3")]
    pub reservation: ::core::option::Option<Reservation>,
    #[prost(message, optional, tag = "4")]
    pub limit: ::core::option::Option<Limit>,
    #[prost(enumeration = "Status", optional, tag = "5")]
    pub status: ::core::option::Option<i32>,
}
/// Nested message and enum types in `HopMessage`.
pub mod hop_message {
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
        Reserve = 0,
        Connect = 1,
        Status = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Reserve => "RESERVE",
                Type::Connect => "CONNECT",
                Type::Status => "STATUS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESERVE" => Some(Self::Reserve),
                "CONNECT" => Some(Self::Connect),
                "STATUS" => Some(Self::Status),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopMessage {
    #[prost(enumeration = "stop_message::Type", required, tag = "1")]
    pub r#type: i32,
    #[prost(message, optional, tag = "2")]
    pub peer: ::core::option::Option<Peer>,
    #[prost(message, optional, tag = "3")]
    pub limit: ::core::option::Option<Limit>,
    #[prost(enumeration = "Status", optional, tag = "4")]
    pub status: ::core::option::Option<i32>,
}
/// Nested message and enum types in `StopMessage`.
pub mod stop_message {
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
        Connect = 0,
        Status = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Connect => "CONNECT",
                Type::Status => "STATUS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONNECT" => Some(Self::Connect),
                "STATUS" => Some(Self::Status),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    #[prost(bytes = "vec", required, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub addrs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reservation {
    /// Unix expiration time (UTC)
    #[prost(uint64, required, tag = "1")]
    pub expire: u64,
    /// relay addrs for reserving peer
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub addrs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// reservation voucher
    #[prost(bytes = "vec", optional, tag = "3")]
    pub voucher: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Limit {
    /// seconds
    #[prost(uint32, optional, tag = "1")]
    pub duration: ::core::option::Option<u32>,
    /// bytes
    #[prost(uint64, optional, tag = "2")]
    pub data: ::core::option::Option<u64>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Ok = 100,
    ReservationRefused = 200,
    ResourceLimitExceeded = 201,
    PermissionDenied = 202,
    ConnectionFailed = 203,
    NoReservation = 204,
    MalformedMessage = 400,
    UnexpectedMessage = 401,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::Ok => "OK",
            Status::ReservationRefused => "RESERVATION_REFUSED",
            Status::ResourceLimitExceeded => "RESOURCE_LIMIT_EXCEEDED",
            Status::PermissionDenied => "PERMISSION_DENIED",
            Status::ConnectionFailed => "CONNECTION_FAILED",
            Status::NoReservation => "NO_RESERVATION",
            Status::MalformedMessage => "MALFORMED_MESSAGE",
            Status::UnexpectedMessage => "UNEXPECTED_MESSAGE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OK" => Some(Self::Ok),
            "RESERVATION_REFUSED" => Some(Self::ReservationRefused),
            "RESOURCE_LIMIT_EXCEEDED" => Some(Self::ResourceLimitExceeded),
            "PERMISSION_DENIED" => Some(Self::PermissionDenied),
            "CONNECTION_FAILED" => Some(Self::ConnectionFailed),
            "NO_RESERVATION" => Some(Self::NoReservation),
            "MALFORMED_MESSAGE" => Some(Self::MalformedMessage),
            "UNEXPECTED_MESSAGE" => Some(Self::UnexpectedMessage),
            _ => None,
        }
    }
}
