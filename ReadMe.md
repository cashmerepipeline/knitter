# 织女是基于绒框架，为解决影视游戏生产流程而开发的支撑平台工程。

## 特性和目标：
  
  充分体现绒框架理念：柔性、优美、开发人员简单温暖的特性
  历史5年的研发，从根本理论上解决了下面的问题

  1. 资产变体问题，同时支持镜头的多版本变体
  2. 事件驱动流程
  3. 数据驱动流程

## 速度和稳定性并重

    使用rust语言开发
    使用mongodb作为数据库
    使用grpc协议

## 跨平台

    可以在rust语言支持的任何平台上运行

## 高度可订制化

  基于绒框架的高度可订制化特点，可以像搭积木一样按需组装生产流程系统，
  可按需组装为单机服务器或者微服务系统，
  可根据需要无缝整合到当前服务体系中

## 路线图

  - [x] 工程管理
  - [x] 资产集管理
  - [x] 资产管理
  - [x] 资产规格和预制件
  - [x] 数据管理
  - [x] 事件系统

以下为企业版的特性，社区版不需要
  - [ ] 工作任务管理
  - [ ] 通知系统
  - [ ] 会话系统
  - [ ] 资源管理系统

## 安装和编译运行

  1. 使用rustup安装rust
  2. 安装mongodb，并启动服务（默认设置为本地服务，如果在其他机器上，可以修改configs.toml的数据库地址端口字段）
  3. 克隆本仓库到本地
  4. 克隆cashmere_core并且和本仓库在同一目录下
  5. 克隆knitter_modules并且和本仓库在同一目录下
  6. 克隆event_modules并且和本仓库在同一目录下
  7. cd knitter && cargo run
  8. 克隆knitter_python_api, 并运行knitter_python_api的测试
  
## 相关仓库地址

  1. https://gitee.com/cashmere/knitter
  2. https://gitee.com/cashmere/knitter_module
  4. https://gitee.com/cashmere/event_module
  5. https://gitee.com/cashmere/knitter_python_api