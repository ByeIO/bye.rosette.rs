#![allow(unused)]

//! 预处理protobuf文件为rust文件

// 标准库
use std::io::Result;

// 主函数
fn main() -> Result<()> {
    // 转换proto文件为rust文件
    tonic_build::configure()
            // 是否编译生成用于服务端的代码
            .build_server(true) 
            // 是否编译生成用于客户端的代码
            .build_client(true) 
            // 输出的路径，此处指定为项目根目录下的assets目录
            .out_dir("assets")  
            // 指定要编译的proto文件路径列表，第二个参数是提供protobuf的扩展路径，
            // 因为protobuf官方提供了一些扩展功能，自己也可能会写一些扩展功能，
            // 如存在，则指定扩展文件路径，如果没有，则指定为proto文件所在目录即可
            .compile(&["./assets/num.proto"], &["protos"])?; 
    // 转换proto文件为rust文件
    tonic_build::configure()
            // 是否编译生成用于服务端的代码
            .build_server(true) 
            // 是否编译生成用于客户端的代码
            .build_client(true) 
            // 输出的路径，此处指定为项目根目录下的assets目录
            .out_dir("assets")  
            // 指定要编译的proto文件路径列表，第二个参数是提供protobuf的扩展路径，
            // 因为protobuf官方提供了一些扩展功能，自己也可能会写一些扩展功能，
            // 如存在，则指定扩展文件路径，如果没有，则指定为proto文件所在目录即可
            .compile(&["./assets/voting.proto"], &["protos"])?; 
    Ok(())
}
