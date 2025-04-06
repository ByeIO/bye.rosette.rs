#![allow(unused)]

//! 使用protobuf解析消息文件num.proto为rust代码并测试相互通信

// 导入proto生成的rust文件
pub mod num {
    include!("../../assets/num.rs");
}

use num::Number;
use prost::Message;

fn main() {
    // 构造一个数字消息
    let number = Number { num: 42 };

    // 将数字消息序列化为字节数组
    let serialized = serialize_number(&number).unwrap();
    println!("序列化后的字节数组: {:?}", serialized);

    // 将字节数组反序列化为数字消息
    let deserialized = deserialize_number(&serialized).unwrap();
    println!("反序列化后的数字消息: {:?}", deserialized.num);
}

/// 将Number消息序列化为字节数组
/// 
/// 参数:
/// - number: 要序列化的Number消息
/// 
/// 返回:
/// - 序列化后的字节数组，如果失败则返回错误
fn serialize_number(number: &Number) -> Result<Vec<u8>, prost::EncodeError> {
    // 使用prost的Message trait的encode方法进行序列化
    let mut buf = Vec::new();
    number.encode(&mut buf)?;
    Ok(buf)
}

/// 将字节数组反序列化为Number消息
/// 
/// 参数:
/// - buf: 要反序列化的字节数组
/// 
/// 返回:
/// - 反序列化后的Number消息，如果失败则返回错误
fn deserialize_number(buf: &[u8]) -> Result<Number, prost::DecodeError> {
    // 使用prost的Message trait的decode方法进行反序列化
    Number::decode(buf)
}
