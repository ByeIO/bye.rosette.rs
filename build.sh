cargo run --release --bin byeefree --target aarch64-unknown-linux-gnu ubuntu -c "uname -a"
rsync -avz --partial --progress qsbye@192.168.30.171:/home/qsbye/byeefree/exp226-rust-byeefree/target/aarch64-unknown-linux-gnu/release/byeefree /Users/workspace/Desktop/projects/ByeIO/software/exp226-rust-byeefree/target/byeefree.linux.aarch64
rsync -avz --partial --progress /Users/workspace/Desktop/projects/ByeIO/software/exp226-rust-byeefree/target/byeefree.linux.aarch64 qsbye@192.168.30.33:/home/qsbye
# 继续编译
cargo-zigbuild run --bin byeefree
cargo-zigbuild run --release --example redb_kv
cargo-zigbuild run --release --example sqlite3_sql
RUST_LOG=TRACE cargo-zigbuild run --bin byeefree account login "qsbye"
RUST_LOG=TRACE cargo-zigbuild run --release --example print_log
RUST_LOG=TRACE cargo-zigbuild run --bin byeefree util sysinfo
# git协作
git push -u home.ssh air
git pull home.ssh air
# 添加包
cargo add rosbag2-rs --package rosette_core
# 测试
cargo run --example rosbag_read_write > result/rosbag_read_write.log
# 导出workspace中的crate
cargo package
# 测试rosbag -> Retail_Street.bag
cd /home/qsbye/Documents
docker run --rm -it -p 8888:8888 -p 11311:11311 -v $PWD:/tmp pcl5pcl5/fast_livo2:v2 bash
source ~/.bashrc
cp -r /home/data /
nohup roscore &
# 循环播放
nohup rosbag play /tmp/Retail_Street.bag --clock --loop &
rostopic list
rostopic echo -n 5 /left_camera/image > /tmp/left_camera_image.log
rostopic echo -n 5 /clock > /tmp/clock.log
rostopic echo -n 5 /livox/imu > /tmp/livox_imu.log
cd /data/fast_livo2 && source devel/setup.bash
rostopic echo -n 5 /livox/lidar > /tmp/livox_lidar.log
# 测试
cargo run --example rosbag_read_fast_livo2 > result/rosbag_read_fast_livo2.log
