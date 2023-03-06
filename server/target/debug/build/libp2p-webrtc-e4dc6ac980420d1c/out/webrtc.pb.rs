#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(enumeration = "message::Flag", optional, tag = "1")]
    pub flag: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `Message`.
pub mod message {
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
    pub enum Flag {
        /// The sender will no longer send messages on the stream.
        Fin = 0,
        /// The sender will no longer read messages on the stream. Incoming data is
        /// being discarded on receipt.
        StopSending = 1,
        /// The sender abruptly terminates the sending part of the stream. The
        /// receiver can discard any data that it already received on that stream.
        Reset = 2,
    }
    impl Flag {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Flag::Fin => "FIN",
                Flag::StopSending => "STOP_SENDING",
                Flag::Reset => "RESET",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FIN" => Some(Self::Fin),
                "STOP_SENDING" => Some(Self::StopSending),
                "RESET" => Some(Self::Reset),
                _ => None,
            }
        }
    }
}
