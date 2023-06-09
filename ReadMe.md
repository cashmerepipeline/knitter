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
  - [ ] 文档
  - [ ] 授权系统完善
  - [ ] 前端图形界面

以下为企业版的特性，30人以内小工作室不是必要的沟通和管理工具，一般需要根据实际情况订制开发
  - [ ] 任务系统
  - [ ] 通知系统
  - [ ] 会话系统
  - [ ] 资源系统

## 编译运行

    脚本的扩展名为ps1，在windows下使用powershell运行，如果是linux或者macos，可以使用bash或者zsh运行，其中没有使用到windows特有的命令，所以可以直接使用bash或者zsh运行

  1. 使用rustup安装rust
  2. 安装mongodb，并启动服务（默认设置为本地服务，如果在其他机器上，可以修改configs.toml的数据库地址端口字段）
  3. 安装git
  4. 新建一个目录，如目录名为cashmere
  5. cd cashmere, 运行`git clone https://gitee.com/cashmere/knitter.git`
  6. 运行./knitter/scripts/prepare_develop_repos.ps1，初始化knitter的依赖仓库
  7. cd cashmere_core/minit, 运行`cargo build`，生成minit初始化工具, 将生成的minit放到knitter仓库目录下或者将minit所在目录加入到环境变量中, 或者运行cargo install minit，将minit安装到\[home\]/.cargo/bin目录下
  8. cd knitter, 运行./scripts/init_db.ps1, 初始化数据库
  9.  cargo run 编译并启动服务
  10. 运行knitter_python_api的测试用例，确保服务正常运行

## 部署

    如果内部使用，一般使用cargo build --release编译，如果对外发布不想暴露调试信息，可以使用cargo build --release --strip编译
    默认编译不启用访问授权检查，如果需要启用内置的授权检查，使用cargo build --release --features view_rules_validate 开启授权检查

  1. 新建一个目录，将生成的knitter执行文件放到该目录下，将configs.toml放到该目录下
  2. 如果数据库没有初始化，使用init_db.ps1初始化数据库
  3. 根据实际情况修改configs.toml
  4. 开启一个终端，cd到该目录，运行knitter
  
## 相关仓库地址

  1. https://gitee.com/cashmere/cashmere_core
  2. https://gitee.com/cashmere/knitter
  3. https://gitee.com/cashmere/knitter-module
  4. https://gitee.com/cashmere/event-module
  4. https://gitee.com/cashmere/data-module
  5. https://gitee.com/cashmere/account-module
  6. https://gitee.com/cashmere/knitter-python-api