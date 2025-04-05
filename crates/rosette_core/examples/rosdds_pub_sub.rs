#![allow(unused)]

//! 订阅/发布ros2格式消息

// 异步处理
use futures::StreamExt;
use tokio::time::{sleep, Duration};

// ros2消息通信协议(RustDDS)
use ros2_client::{ros2, rosout, ros2::policy, Context, MessageTypeName, Name, NodeName, NodeOptions};

// 日志
use log::info;

// 使用Tokio的异步主函数宏
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志系统(从项目根目录开始)）
    log4rs::init_file("./assets/rosdds_pub_sub.log4rs.yaml", Default::default())?;

    // 创建ROS2上下文和节点 
    let context = Context::new()?;
    
    // 创建两个独立节点
    let mut pub_node = context.new_node(NodeName::new("/rustdds", "pub")?, NodeOptions::new().enable_rosout(true))?;
    let mut sub_node = context.new_node(NodeName::new("/rustdds", "sub")?, NodeOptions::new().enable_rosout(true))?;

    // 配置可靠传输的QoS策略 
    let reliable_qos = ros2::QosPolicyBuilder::new()
        .history(policy::History::KeepLast { depth: 10 })
        .reliability(policy::Reliability::Reliable {
            max_blocking_time: ros2::Duration::from_millis(100),
        })
        .durability(policy::Durability::TransientLocal)
        .build();

    // 创建主题（Topic）
    let chatter_topic = pub_node.create_topic(
        &Name::new("/", "chatter")?,
        MessageTypeName::new("std_msgs", "String"),
        &reliable_qos,
    )?;

    // 创建发布者（Publisher）
    let chatter_publisher = pub_node.create_publisher::<String>(&chatter_topic, None)?;
    
    // 创建订阅者（Subscription）
    let chatter_subscription = sub_node.create_subscription::<String>(&chatter_topic, Some(reliable_qos))?;

    // 启动并发任务：发布消息 
    let publisher_task = tokio::spawn(async move {
        let mut count = 0;
        let filler = "All work and no play makes ROS a dull boy. ".repeat(4);
        
        loop {
            count += 1;
            let message = format!("count={} {}", count, filler);
            println!("[发布端] 正在发送，计数={} 长度={}", count, message.len());
            
            // 异步发布消息 
            if let Err(e) = chatter_publisher.async_publish(message).await {
                eprintln!("发布失败: {:?}", e);
            }
            
            // 每2秒发送一次 
            sleep(Duration::from_secs(2)).await;
        }
    });

    // 启动并发任务：订阅消息 
    let subscriber_task = tokio::spawn(async move {
       // loop{
            let mut subscription_stream = chatter_subscription
                .async_stream()
                .for_each(|result| async {
                    match result {
                        Ok((msg, _)) => println!("[订阅端] 收到消息: {}", msg),
                        Err(e) => eprintln!("接收错误: {:?}", e),
                    }
                });
    
            // 记录ROS日志 
            rosout!(
                sub_node,
                ros2::LogLevel::Info,
                "节点初始化完成，开始监听消息"
            );
    
            subscription_stream.await
            // } // end loop
    });

    // 等待所有任务完成 
    let (pub_res, sub_res) = tokio::join!(publisher_task, subscriber_task);
    pub_res?;
    sub_res?;

    Ok(())
}
