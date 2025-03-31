# Rosette(玫瑰之心): 机器人框架(WIP: Work In Progress)
rust实现.

|小王子怎么也想不到, |机器人也有理想, 通往诗和宇宙深深处。|
|-------------------------------------------------------|----------------------------------------|
|![logo-text](https://img.shields.io/badge/Rosette-pink)|![logo](./assets/rosette_logo_64x61.png)|

## 使用说明
### 特性
1. 分布式通信(类似ROS2)

### 对比
||Rosette|ROS2|
|--|-----|----|
|分布式通信|✅|✅|
|开发语言|Rust|C++|

## 开发说明
### 目录说明
```sh
- assets文件夹 : 资源文件
- crates文件夹 : 组件
- docs文件夹 : 文档
- files文件夹 : 资源文件
- static文件夹 : 静态资源
    * rosonweb.io文件夹 : ros wasm版, 运行于浏览器中[@ref](https://rosonweb.io/)
```

### 组件(crates文件夹)
1. rosette core : 核心组件, 处理消息通信
2. webviz : 网页中进行rosette消息查看(类似rviz)[@ref]()
3. mujoco : 网页中进行机器人仿真(类似gazebo)

## 参考资料
1. TinyROS[@ref](https://github.com/neuralsandwich/TinyROS)
2. ROS2[@ref](https://github.com/ros2)
3. aimRT[@ref](https://github.com/AimRT/AimRT)
4. copper[@ref](https://github.com/copper-project/copper-rs)
5. prometheus[@ref](https://github.com/amov-lab/Prometheus)

## 项目状态
![star-history](https://www.star-history.com/#ByeIO/bye.rosette.rs&Date)
<picture>
  <source
    media="(prefers-color-scheme: dark)"
    srcset="
      https://api.star-history.com/svg?repos=star-history/star-history&type=Date&theme=dark
    "
  />
  <source
    media="(prefers-color-scheme: light)"
    srcset="
      https://api.star-history.com/svg?repos=star-history/star-history&type=Date
    "
  />
  <img
    alt="Star History Chart"
    src="https://api.star-history.com/svg?repos=star-history/star-history&type=Date"
  />
</picture>
