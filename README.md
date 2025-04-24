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

### 支持列表
#### 一级支持(Tier1)
* [x] Linux(ubuntu)

#### 二级支持(Tier2)
* [ ] macOS
* [ ] windows
* [ ] harmonyOS

### 功能规划
#### rosette_cli
* [ ] action动作指令
* [ ] bag数据包处理
* [ ] component组件
* [ ] daemon守护进程
* [ ] doctor自检
* [ ] interface接口
* [ ] launch启动
* [ ] lifecycle生命周期
* [ ] multicast多播
* [ ] node节点
* [ ] param参数
* [ ] pkg包管理
* [ ] playground可视化调试命令
* [ ] run运行
* [ ] security安全性
* [ ] service服务
* [ ] topic话题

#### rosette_core
* [ ] msg, srv, action文件解析
* [x] DDS分布式通信
* [ ] mqtt通信
* [x] rosbag1数据包读写
* [x] rogbag2数据包读写
* [ ] ros1_bridge与ros1通信
* [ ] ros2_bridge与ros2通信

#### rosette_playground
* [ ] webviz浏览器可视化话题内容
* [ ] webrqt浏览器可视化调试话题、服务
* [ ] mujoco浏览器机器人仿真

#### rosette_derive
**暂无规划**

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
1. rosette_cli : rosette的总入口, 引用rosette_cli作为库即可使用rosette的全部能力.
2. rosette_core: 核心组件, 处理消息通信
2. rosette_derive : 宏编程, 内部库
3. rosette_playground : 可视化调试工具和仿真工具(webviz, mujoco)

### 技术说明
1. MCP通信
    >MCP 遵循客户端-服务器架构（client-server），其中包含以下几个核心概念：
    >MCP 主机（MCP Hosts）：发起请求的 LLM 应用程序（例如 Claude Desktop、IDE 或 AI 工具）。
    >MCP 客户端（MCP Clients）：在主机程序内部，与 MCP server 保持 1:1 的连接。
    >MCP 服务器（MCP Servers）：为 MCP client 提供上下文、工具和 prompt 信息。
    >本地资源（Local Resources）：本地计算机中可供 MCP server 安全访问的资源（例如文件、数据库）。
    >远程资源（Remote Resources）：MCP server 可以连接到的远程资源（例如通过 API）。

2. ROS2协议(ros2_humble)
    - **架构设计**：ROS2采用去中心化的架构，摒弃了ROS1中依赖单一主节点（ROS Master）的方式，不再需要中心节点来协调通信。
    - **通信机制**：
        - **DDS分布式通信**：
            - **核心概念**：基于数据分发服务（DDS）协议，DDS是一种面向分布式系统的协议，支持实时、可扩展和多平台的通信。其核心是一个以数据为中心的发布-订阅（Data-Centric Publish-Subscribe，DCPS）模型，该模型旨在为分布式异构平台上的进程间提供高效的数据传输。
            - **通信实体**：DCPS模型由DomainParticipant（域参与者）、Publisher（发布者）、Subscriber（订阅者）、DataWriter（数据写入器）、DataReader（数据读取器）和Topic（主题）组成。各进程之间的数据传输是根据服务质量（QoS）策略执行的。
            - **RTPS协议**：DataWriter和DataReader之间的数据传输通过RTPS协议（DDS标准协议）进行，该协议允许来自多个供应商的DDS实现通过抽象和优化传输（如TCP/UDP/IP）进行互操作。
            - **QoS配置**：支持QoS配置，开发者可根据需求调整传输策略，如保证数据必达、容忍延迟等。
        - **进程内通信**：在同一个进程内的节点之间，ROS2采用进程内通信机制，这种通信方式不依赖DDS，而是通过直接传递指针等方式实现更高效的数据传输。
    - **功能特性**：
        - **实时性支持**：支持实时性，适合实时系统。
        - **节点生命周期管理**：引入了节点生命周期管理。
        - **多机支持**：原生支持分布式、跨机器通信。
        - **消息序列化**：使用DDS高效的序列化方式。
        - **安全性增强**：更加注重安全性，使用DDS安全扩展进行加密、访问控制和身份验证。
    - **通信范式**：
        - **主题**：允许在发布-订阅模式下进行消息传递，一个节点将数据发布到一个命名的主题上，任意数量的节点可以订阅该主题以接收数据。通过QoS策略（可靠性、持久性、历史记录）增强，以满足定制的通信需求。
        - **服务**：允许进行请求-响应交互，如果节点A需要从节点B进行一次性计算或数据检索，它可以发送一个服务请求，节点B会进行回复。从DDS的改进中受益。
        - **操作**：引入用于基于目标的交互，允许发送、取消目标并获取反馈/结果，适用于长时间运行的任务，如导航。

3. EasyTier虚拟局域网(软件组网, 针对DDS中需要暴露大量端口的情况, 解决复杂网络环境下节点之间的通信)
    EasyTier是一个简单、安全、去中心化的异地组网方案，由Rust和Tokio驱动，具有以下特点：
    - **去中心化**：不区分客户端/服务端，无需依赖中心化服务，节点平等且独立。
    - **易用**：支持通过网页、客户端、命令行多种方式使用，操作简单。支持使用共享节点一键组网。
    - **跨平台**：支持Windows、MacOS、Linux、FreeBSD、Android等系统，支持X86、ARM、MIPS等硬件架构。
    - **安全**：支持AES-GCM或WireGuard加密保护中转流量，免受中间人攻击。
    - **高效NAT穿透**：支持基于UDP的NAT穿透和IPV6穿透，在某些情况下可以打通NAT4-NAT4的网络。
    - **子网代理**：节点可以将其可访问的网段转发到虚拟网，允许其他节点通过该节点访问这些子网。
    - **智能路由**：支持延迟优先模式，自动选择最优路径，提供最佳的网络体验。
    - **高性能**：全链路零拷贝，性能与主流组网软件相当。节点间通信支持TCP/UDP/QUIC/WG等多种协议。

4. embedded-nano-mesh(硬件组网)
    Embedded-nano-mesh 是一个轻量级的、去中心化的、自组织的无线网络协议栈，专为嵌入式设备设计，具有以下特点：
    - **轻量级**：代码简洁，资源占用少，适合在资源受限的嵌入式设备上运行。
    - **去中心化**：网络中没有中心节点，每个节点地位平等，增强了网络的鲁棒性。
    - **自组织**：节点能够自动发现邻居节点并建立连接，形成网络拓扑结构，无需人工干预。
    - **无线通信**：主要通过无线方式（如Wi-Fi、蓝牙等）进行通信，方便设备之间的灵活组网。
    - **低功耗**：优化了通信协议和算法，降低了设备的功耗，延长了设备的使用寿命。
    - **安全性**：支持加密通信，保障数据传输的安全性，防止数据泄露和被篡改。
    - **可扩展性**：能够支持一定数量的节点加入网络，适应不同规模的嵌入式设备组网需求。

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
