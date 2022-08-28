/// MsgRegisterAccount defines the payload for Msg/RegisterAccount
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, ::prost::Message)]
pub struct MsgRegisterAccount {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
}
/// MsgRegisterAccountResponse defines the response for Msg/RegisterAccount
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterAccountResponse {}
/// MsgSubmitTx defines the payload for Msg/SubmitTx
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitTx {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub msg: ::core::option::Option<::prost_types::Any>,
}
/// MsgSubmitTxResponse defines the response for Msg/SubmitTx
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitTxResponse {}
