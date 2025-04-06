/// 数字消息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Number {
    #[prost(int64, tag = "1")]
    pub num: i64,
}
