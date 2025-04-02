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
