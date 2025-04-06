# ROS2 Interfaces for Rust

This repository contains Rust structs for all interfaces (i.e., messages and services) that are listed as releases
on the [ROS Index for Jazzy](https://index.ros.org/packages/#jazzy).

The interfaces implement traits from the `ros2-client` library and can be used easily in conjunction.

Every package is a separate feature for this crate, so you can cherry-pick the interfaces you need.
Example are `std_msgs`, `geometry_msgs` and `nav2_msgs`. 

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
ros2-interfaces-jazzy = { version = "*", features = ["std_msgs"] }  # replace with the latest version and features you need
```

Then you can use the interfaces in your code:

```rust
use ros2_client::{Context, MessageTypeName, Name, NodeName, NodeOptions};
use ros2_interfaces_jazzy::std_msgs;

fn test_publisher() {
    let context = Context::new().unwrap();
    let mut node = context
        .new_node(
            NodeName::new("/rustdds", "rustdds_listener").unwrap(),
            NodeOptions::new().enable_rosout(true),
        )
        .unwrap();

    let topic = node
        .create_topic(
            &Name::new("/","topic").unwrap(),
            MessageTypeName::new("std_msgs", "String"),
            &ros2_client::DEFAULT_PUBLISHER_QOS,
        )
        .unwrap();

    let publisher = node
        .create_publisher::<std_msgs::msg::String>(&topic, None)
        .unwrap();


    let message = std_msgs::msg::String {
        data: "Hello, world!".to_string(),
    };

    publisher.publish(message).unwrap();
}
```


## Known Issues

### Disabled Interfaces
The following packages (and thereby: features) do not compile due to incompatible or missing interface definitions:

* `depthai_ros_msgs`
* `mrpt_msgs`
* `mrpt_nav_interfaces`
* `rosbag2_test_msgdefs`
* `sick_scan_xd`

Therefore, the features have been commented out in the `Cargo.toml` file.