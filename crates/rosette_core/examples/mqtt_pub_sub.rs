#![allow(unused)]

//! 订阅/发布mqtt_v3.1.1消息
//! mqtt元素: 队列, 发布者, 订阅者

// mqtt服务端
use rumqttd::{Broker, Config, Notification, PrometheusSetting, ServerSettings, ConnectionSettings};

// mqtt客户端
use rumqttc::mqttbytes::QoS;
use rumqttc::{AsyncClient, MqttOptions};

// 异步框架
use tokio::task;
use tokio::time;

// 日志
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::Subscriber;

// 标准库
use std::error::Error;
use std::time::Duration;
use std::thread;
use std::collections::HashMap;
use core::net::SocketAddr;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化 `tracing` 日志记录器
    let subscriber = Subscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set the global tracing subscriber");

    // 安装彩色回溯功能
    color_backtrace::install();
    
    // 配置 v4 的 ServerSettings
    let mut v4_servers = HashMap::new();
    let v4_server_settings = ServerSettings {
        name: "mqtt_v3.3.3".to_string(),
        listen: "0.0.0.0:11883".parse::<SocketAddr>().expect("Invalid SocketAddr"),
        // 如果需要 TLS 配置，可以在这里设置
        tls: None, 
        // 示例值
        next_connection_delay_ms: 1000, 
        // 假设 ConnectionSettings 有默认值
        connections: ConnectionSettings::default(), 
    };
    v4_servers.insert("hello".to_string(), v4_server_settings);

    // 创建 MQTT Broker 配置
    let mut config = Config::default();

    // 将 v4_servers 添加到配置中
    config.v4 = Some(v4_servers);
    
    // 继续创建 MQTT Broker 配置
    let broker = Broker::new(config);
    let alerts = broker.alerts().unwrap();

    // 创建一个线程用于处理 Broker 的警报
    let handle = thread::spawn(move || loop {
        if let Ok(alert) = alerts.recv() {
            info!("Alert: {:?}", alert);
        }
        thread::sleep(Duration::from_secs(1));
    });

    // 确保 Broker 启动后再让客户端连接
    tokio::time::sleep(Duration::from_secs(1)).await;

    // 创建 MQTT 客户端选项，指定客户端 ID、主机名和端口
    let mut mqttoptions = MqttOptions::new("test-1", "127.0.0.1", 11883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    // 创建 MQTT 客户端和事件循环
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // 在后台任务中运行 `requests` 函数
    task::spawn(async move {
        requests(client).await;
        time::sleep(Duration::from_secs(3)).await;
    });

    // 主循环，用于处理事件循环中的事件
    loop {
        let event = eventloop.poll().await;
        match &event {
            Ok(v) => {
                // 如果事件成功，打印事件内容
                info!("Event = {:?}", v);
            }
            Err(e) => {
                // 如果发生错误，打印错误信息
                info!("Error = {:?}", e);
                // 返回成功结果以退出程序
                return Ok(());
            }
        }
    }
}

// 异步函数，用于发送和订阅 MQTT 消息
async fn requests(client: AsyncClient) {
    // 订阅主题 "hello/world"，QoS 级别为最多一次
    client
        .subscribe("hello/world", QoS::AtMostOnce)
        .await
        .unwrap();

    // 发送 10 条消息到主题 "hello/world"
    for i in 1..=10 {
        // 发布消息，QoS 级别为确保一次，消息内容为长度为 i 的字节数组
        client
            .publish("hello/world", QoS::ExactlyOnce, false, vec![1; i])
            .await
            .unwrap();
        // 每次发送后等待 1 秒
        time::sleep(Duration::from_secs(1)).await;
    }

    // 在函数结束前等待 120 秒
    time::sleep(Duration::from_secs(120)).await;
}
