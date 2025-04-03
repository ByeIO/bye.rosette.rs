#![allow(unused)]

//! 读取fast_livo2示例rosbag

// 错误处理
use anyhow::{Result, Ok};

// rosbag1处理
use rosbag::{ChunkRecord, MessageRecord, RosBag, record_types::{Chunk, IndexData}};

// 字节处理
use byteorder::{LittleEndian, ReadBytesExt};

// 标准库
use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::prelude::*;

#[derive(Debug)]
struct Stamp {
    secs: i32,
    nsecs: i32,
}

#[derive(Debug)]
struct Header {
    seq: u32,
    stamp: Stamp,
    frame_id: String,
}

#[derive(Debug)]
struct Point {
    offset_time: u32,
    x: f32,
    y: f32,
    z: f32,
    reflectivity: u8,
    tag: u8,
    line: u8,
}

#[derive(Debug)]
struct CustomMsg {
    header: Header,
    timebase: u64,
    point_num: u32,
    lidar_id: u8,
    rsvd: [u8; 3],
    points: Vec<Point>,
}

fn parse_custom_msg(data: &[u8]) -> Result<CustomMsg> {
    let mut cursor = Cursor::new(data);
    let seq = cursor.read_u32::<LittleEndian>()?;
    let secs = cursor.read_i32::<LittleEndian>()?;
    let nsecs = cursor.read_i32::<LittleEndian>()?;
    let frame_id_len = cursor.read_u32::<LittleEndian>()? as usize;
    let mut frame_id_bytes = vec![0u8; frame_id_len];
    cursor.read_exact(&mut frame_id_bytes)?;
    let frame_id = String::from_utf8(frame_id_bytes)?;
    let timebase = cursor.read_u64::<LittleEndian>()?;
    let point_num = cursor.read_u32::<LittleEndian>()?;
    let lidar_id = cursor.read_u8()?;
    let mut rsvd = [0u8; 3];
    cursor.read_exact(&mut rsvd)?;
    let mut points = Vec::with_capacity(point_num as usize);
    for _ in 0..point_num {
        points.push(Point {
            offset_time: cursor.read_u32::<LittleEndian>()?,
            x: cursor.read_f32::<LittleEndian>()?,
            y: cursor.read_f32::<LittleEndian>()?,
            z: cursor.read_f32::<LittleEndian>()?,
            reflectivity: cursor.read_u8()?,
            tag: cursor.read_u8()?,
            line: cursor.read_u8()?,
        });
    }
    Ok(CustomMsg {
        header: Header { seq, stamp: Stamp { secs, nsecs }, frame_id },
        timebase,
        point_num,
        lidar_id,
        rsvd,
        points,
    })
}

/// 处理单个数据块中的消息记录
fn process_chunk(
    chunk: Chunk,
    conn_id_to_topic: &mut HashMap<u32, String>,
    message_count: &mut usize,
) -> Result<()> {
    for msg_record in chunk.messages() {
        match msg_record? {
            MessageRecord::Connection(conn) => {
                conn_id_to_topic.insert(conn.id, conn.topic.to_ascii_lowercase());
            }
            MessageRecord::MessageData(msg_data) => {
                if let Some(topic) = conn_id_to_topic.get(&msg_data.conn_id) {
                    if topic == "/livox/lidar" {
                        let custom_msg = parse_custom_msg(&msg_data.data)?;
                        
                        // 结构化输出
                        println!("header: ");
                        println!("  seq: {}", custom_msg.header.seq);
                        println!("  stamp: ");
                        println!("    secs: {}", custom_msg.header.stamp.secs);
                        println!("    nsecs: {}", custom_msg.header.stamp.nsecs);
                        println!("  frame_id: \"{}\"", custom_msg.header.frame_id);
                        println!("timebase: {}", custom_msg.timebase);
                        println!("point_num: {}", custom_msg.point_num);
                        println!("lidar_id: {}", custom_msg.lidar_id);
                        println!("rsvd: [{}, {}, {}]", custom_msg.rsvd[0], custom_msg.rsvd[1], custom_msg.rsvd[2]);
                        println!("points: ");
                        for point in &custom_msg.points {
                            println!("  - ");
                            println!("    offset_time: {}", point.offset_time);
                            println!("    x: {:.15}", point.x);
                            println!("    y: {:.15}", point.y);
                            println!("    z: {:.15}", point.z);
                            println!("    reflectivity: {}", point.reflectivity);
                            println!("    tag: {}", point.tag);
                            println!("    line: {}", point.line);
                        }
                        println!("\n-----------------------------");
                        
                        *message_count += 1;
                        if *message_count >= 5 {
                            return Ok(());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let bag_relative_path = "../../assets/Retail_Street.bag";
    let current_dir = env::current_dir().expect("获取当前目录失败");
    let bag_abs_path = current_dir.join(bag_relative_path);
    println!("正在打开ROS bag文件: {}", bag_abs_path.display());
    
    let bag = RosBag::new(bag_abs_path)?;
    let mut conn_id_to_topic = HashMap::new();
    let mut message_count = 0;

    for chunk_record in bag.chunk_records() {
        match chunk_record? {
            ChunkRecord::Chunk(chunk) => {
                // 处理当前数据块
                process_chunk(chunk, &mut conn_id_to_topic, &mut message_count)?;
                if message_count >= 5 {
                    break;
                }
            }
            ChunkRecord::IndexData(_) => {}
        }
    }

    println!("成功读取前5条消息");
    Ok(())
}
