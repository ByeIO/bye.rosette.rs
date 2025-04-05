#![allow(unused)]

//! 数据包命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 错误处理
use anyhow;

/// 数据包命令解析器
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct BagArgs {
    #[command(subcommand)]
    pub command: BagCommand,
}

/// 数据包子命令定义
#[derive(Subcommand, Debug)]
pub enum BagCommand {
    /// Burst 数据从数据包
    Burst {
        #[arg(short, long, value_name = "STORAGE", default_value = "auto")]
        storage: String,
        #[arg(long, value_name = "READ_AHEAD_QUEUE_SIZE")]
        read_ahead_queue_size: Option<usize>,
        #[arg(long, value_name = "TOPICS", value_delimiter = ' ')]
        topics: Vec<String>,
        #[arg(long, value_name = "SERVICES", value_delimiter = ' ')]
        services: Vec<String>,
        #[arg(long, value_name = "QOS_PROFILE_OVERRIDES_PATH")]
        qos_profile_overrides_path: Option<String>,
        #[arg(long, value_name = "REMAP", value_delimiter = ' ')]
        remap: Vec<String>,
        #[arg(long, value_name = "STORAGE_CONFIG_FILE")]
        storage_config_file: Option<String>,
        #[arg(long, value_name = "START_OFFSET")]
        start_offset: Option<f64>,
        #[arg(short, long, value_name = "NUM_MESSAGES")]
        num_messages: Option<usize>,
        #[arg(value_name = "BAG_PATH")]
        bag_path: String,
    },
    /// 转换数据包
    Convert {
        #[arg(short, long, value_name = "URI", required = true)]
        input: Vec<String>,
        #[arg(short, long, value_name = "OUTPUT_OPTIONS", required = true)]
        output_options: String,
    },
    /// 打印数据包信息
    Info {
        #[arg(short, long, value_name = "STORAGE", default_value = "auto")]
        storage: String,
        #[arg(long)]
        topic_name: bool,
        #[arg(long)]
        verbose: bool,
        #[arg(value_name = "BAG_PATH")]
        bag_path: String,
    },
    /// 列出可用插件
    List {
        #[arg(value_name = "PLUGIN_TYPE", required = true)]
        plugin_type: String,
        #[arg(long)]
        verbose: bool,
    },
    /// 播放数据包
    Play {
        #[arg(short, long, value_name = "STORAGE", default_value = "auto")]
        storage: String,
        #[arg(long, value_name = "READ_AHEAD_QUEUE_SIZE")]
        read_ahead_queue_size: Option<usize>,
        #[arg(short, long, value_name = "RATE")]
        rate: Option<f64>,
        #[arg(long, value_name = "TOPICS", value_delimiter = ' ')]
        topics: Vec<String>,
        #[arg(long, value_name = "SERVICES", value_delimiter = ' ')]
        services: Vec<String>,
        #[arg(long, value_name = "REGEX")]
        regex: Option<String>,
        #[arg(long, value_name = "EXCLUDE_REGEX")]
        exclude_regex: Option<String>,
        #[arg(long, value_name = "EXCLUDE_TOPICS", value_delimiter = ' ')]
        exclude_topics: Vec<String>,
        #[arg(long, value_name = "EXCLUDE_SERVICES", value_delimiter = ' ')]
        exclude_services: Vec<String>,
        #[arg(long, value_name = "QOS_PROFILE_OVERRIDES_PATH")]
        qos_profile_overrides_path: Option<String>,
        #[arg(short, long)]
        loop_playback: bool,
        #[arg(long, value_name = "REMAP", value_delimiter = ' ')]
        remap: Vec<String>,
        #[arg(long, value_name = "STORAGE_CONFIG_FILE")]
        storage_config_file: Option<String>,
        #[arg(long, value_name = "DELAY")]
        delay: Option<f64>,
        #[arg(long, value_name = "PLAYBACK_DURATION")]
        playback_duration: Option<f64>,
        #[arg(long, value_name = "PLAYBACK_UNTIL_SEC")]
        playback_until_sec: Option<f64>,
        #[arg(long, value_name = "PLAYBACK_UNTIL_NSEC")]
        playback_until_nsec: Option<u64>,
        #[arg(long)]
        disable_keyboard_controls: bool,
        #[arg(short, long)]
        start_paused: bool,
        #[arg(long, value_name = "START_OFFSET")]
        start_offset: Option<f64>,
        #[arg(long, value_name = "WAIT_FOR_ALL_ACKED")]
        wait_for_all_acked: Option<u64>,
        #[arg(long)]
        disable_loaned_message: bool,
        #[arg(long)]
        publish_service_requests: bool,
        #[arg(long, value_name = "SERVICE_REQUESTS_SOURCE")]
        service_requests_source: Option<String>,
        #[arg(long, value_name = "LOG_LEVEL")]
        log_level: Option<String>,
        #[arg(value_name = "BAG_PATH")]
        bag_path: String,
    },
    /// 记录数据到数据包
    Record {
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<String>,
        #[arg(short, long, value_name = "STORAGE", default_value = "mcap")]
        storage: String,
        #[arg(long, value_name = "TOPICS", value_delimiter = ' ')]
        topics: Vec<String>,
        #[arg(long, value_name = "SERVICES", value_delimiter = ' ')]
        services: Vec<String>,
        #[arg(long, value_name = "TOPIC_TYPES", value_delimiter = ' ')]
        topic_types: Vec<String>,
        #[arg(short, long)]
        all: bool,
        #[arg(long)]
        all_topics: bool,
        #[arg(long)]
        all_services: bool,
        #[arg(long, value_name = "REGEX")]
        regex: Option<String>,
        #[arg(long, value_name = "EXCLUDE_REGEX")]
        exclude_regex: Option<String>,
        #[arg(long, value_name = "EXCLUDE_TOPIC_TYPES", value_delimiter = ' ')]
        exclude_topic_types: Vec<String>,
        #[arg(long, value_name = "EXCLUDE_TOPICS", value_delimiter = ' ')]
        exclude_topics: Vec<String>,
        #[arg(long, value_name = "EXCLUDE_SERVICES", value_delimiter = ' ')]
        exclude_services: Vec<String>,
        #[arg(long)]
        include_unpublished_topics: bool,
        #[arg(long)]
        include_hidden_topics: bool,
        #[arg(long)]
        no_discovery: bool,
        #[arg(short, long, value_name = "POLLING_INTERVAL")]
        polling_interval: Option<u64>,
        #[arg(long)]
        ignore_leaf_topics: bool,
        #[arg(long, value_name = "QOS_PROFILE_OVERRIDES_PATH")]
        qos_profile_overrides_path: Option<String>,
        #[arg(short, long, value_name = "SERIALIZATION_FORMAT")]
        serialization_format: Option<String>,
        #[arg(short, long, value_name = "MAX_BAG_SIZE")]
        max_bag_size: Option<u64>,
        #[arg(short, long, value_name = "MAX_BAG_DURATION")]
        max_bag_duration: Option<u64>,
        #[arg(long, value_name = "MAX_CACHE_SIZE")]
        max_cache_size: Option<u64>,
        #[arg(long)]
        disable_keyboard_controls: bool,
        #[arg(long)]
        start_paused: bool,
        #[arg(long)]
        use_sim_time: bool,
        #[arg(long, value_name = "NODE_NAME")]
        node_name: Option<String>,
        #[arg(long, value_name = "CUSTOM_DATA", value_delimiter = ' ')]
        custom_data: Vec<String>,
        #[arg(long)]
        snapshot_mode: bool,
        #[arg(long, value_name = "LOG_LEVEL")]
        log_level: Option<String>,
        #[arg(long, value_name = "STORAGE_CONFIG_FILE")]
        storage_config_file: Option<String>,
        #[arg(long, value_name = "STORAGE_PRESET_PROFILE")]
        storage_preset_profile: Option<String>,
        #[arg(long, value_name = "COMPRESSION_QUEUE_SIZE")]
        compression_queue_size: Option<usize>,
        #[arg(long, value_name = "COMPRESSION_THREADS")]
        compression_threads: Option<usize>,
        #[arg(long, value_name = "COMPRESSION_MODE")]
        compression_mode: Option<String>,
        #[arg(long, value_name = "COMPRESSION_FORMAT")]
        compression_format: Option<String>,
        #[arg(value_name = "TOPICS", value_delimiter = ' ')]
        topics_to_record: Vec<String>,
    },
    /// 重建数据包元数据文件
    Reindex {
        #[arg(short, long, value_name = "STORAGE", default_value = "auto")]
        storage: String,
        #[arg(value_name = "BAG_PATH")]
        bag_path: String,
    },
}

/// 数据包命令执行函数
pub async fn bag_cmd(cmd: BagCommand) -> anyhow::Result<()> {
    match cmd {
        BagCommand::Burst {
            storage,
            read_ahead_queue_size,
            topics,
            services,
            qos_profile_overrides_path,
            remap,
            storage_config_file,
            start_offset,
            num_messages,
            bag_path,
        } => {
            // 在这里实现 Burst 命令的逻辑
            println!("Executing Burst command...");
            // 示例：打印参数值
            println!("Storage: {}", storage);
            println!("Bag Path: {}", bag_path);
            // 其他参数可以根据需要处理
        }
        BagCommand::Convert {
            input,
            output_options,
        } => {
            // 在这里实现 Convert 命令的逻辑
            println!("Executing Convert command...");
            println!("Input: {:?}", input);
            println!("Output Options: {}", output_options);
        }
        BagCommand::Info {
            storage,
            topic_name,
            verbose,
            bag_path,
        } => {
            // 在这里实现 Info 命令的逻辑
            println!("Executing Info command...");
            println!("Storage: {}", storage);
            println!("Bag Path: {}", bag_path);
            if topic_name {
                println!("Only displaying topic names.");
            }
            if verbose {
                println!("Verbose mode enabled.");
            }
        }
        BagCommand::List {
            plugin_type,
            verbose,
        } => {
            // 在这里实现 List 命令的逻辑
            println!("Executing List command...");
            println!("Plugin Type: {}", plugin_type);
            if verbose {
                println!("Verbose mode enabled.");
            }
        }
        BagCommand::Play {
            storage,
            read_ahead_queue_size,
            rate,
            topics,
            services,
            regex,
            exclude_regex,
            exclude_topics,
            exclude_services,
            qos_profile_overrides_path,
            loop_playback,
            remap,
            storage_config_file,
            delay,
            playback_duration,
            playback_until_sec,
            playback_until_nsec,
            disable_keyboard_controls,
            start_paused,
            start_offset,
            wait_for_all_acked,
            disable_loaned_message,
            publish_service_requests,
            service_requests_source,
            log_level,
            bag_path,
        } => {
            // 在这里实现 Play 命令的逻辑
            println!("Executing Play command...");
            println!("Storage: {}", storage);
            println!("Bag Path: {}", bag_path);
            // 其他参数可以根据需要处理
        }
        BagCommand::Record {
            output,
            storage,
            topics,
            services,
            topic_types,
            all,
            all_topics,
            all_services,
            regex,
            exclude_regex,
            exclude_topic_types,
            exclude_topics,
            exclude_services,
            include_unpublished_topics,
            include_hidden_topics,
            no_discovery,
            polling_interval,
            ignore_leaf_topics,
            qos_profile_overrides_path,
            serialization_format,
            max_bag_size,
            max_bag_duration,
            max_cache_size,
            disable_keyboard_controls,
            start_paused,
            use_sim_time,
            node_name,
            custom_data,
            snapshot_mode,
            log_level,
            storage_config_file,
            storage_preset_profile,
            compression_queue_size,
            compression_threads,
            compression_mode,
            compression_format,
            topics_to_record,
        } => {
            // 在这里实现 Record 命令的逻辑
            println!("Executing Record command...");
            println!("Storage: {}", storage);
            println!("Output: {:?}", output);
            // 其他参数可以根据需要处理
        }
        BagCommand::Reindex {
            storage,
            bag_path,
        } => {
            // 在这里实现 Reindex 命令的逻辑
            println!("Executing Reindex command...");
            println!("Storage: {}", storage);
            println!("Bag Path: {}", bag_path);
        },
        _ => {
            // do nothing
        },
        
    }// end match

    // 返回成功结果
    Ok(())
}
