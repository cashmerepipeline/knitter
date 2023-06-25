#![recursion_limit = "256"]

mod protocol;
mod services;

use dependencies_sync::once_cell;
use dependencies_sync::rust_i18n::{self, i18n, t};
use server_utils::get_tls_configs;
i18n!("locales");

use std::fs::File;
use std::str::FromStr;
use std::time::Duration;

use dependencies_sync::tokio;
use dependencies_sync::tower;

use event_module::protocols::Event;
// 日志相关
use dependencies_sync::log::{error, info};
use dependencies_sync::simplelog::{
    self, ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};

// 终止相关
use dependencies_sync::tokio::runtime::{self};
use dependencies_sync::tokio::signal;
use dependencies_sync::tokio::sync::oneshot::{self};

use dependencies_sync::tonic::codec::CompressionEncoding;
use dependencies_sync::tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};

use account_module::account_server::AccountServer;
use account_module::protocols::account_grpc_server::AccountGrpcServer;
use auth::check::check_auth_token;
use configs::read_configs_file_path;
use runtime_handle::set_runtime_handle;

use crate::protocol::knitter_grpc_server::KnitterGrpcServer;
use services::KnitterServer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化设置
    let configs_path = read_configs_file_path();
    configs::init_configs_path(configs_path).expect(t!("配置文件不存在").as_str());

    let configs = configs::get_configs();

    // 语言设置
    rust_i18n::set_locale(configs.server.language_code.as_str());

    // 初始化日志
    server_utils::init_log_dir(&configs.server.log_dir).expect(t!("创建日志目录失败").as_str());
    let log_level = LevelFilter::from_str(configs.server.log_level.as_str()).unwrap();

    let log_config = simplelog::ConfigBuilder::new()
        .set_time_format_rfc3339()
        .set_time_offset_to_local()
        .unwrap()
        .build();
    CombinedLogger::init(vec![
        TermLogger::new(
            log_level,
            log_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Always,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            log_config,
            File::create("log/grpc_server.log").expect("打开日志文件失败"),
        ),
    ])
    .unwrap();

    let runtime = runtime::Runtime::new().expect("新建tokio运行时失败");
    let handle = runtime.handle().clone();
    set_runtime_handle(runtime.handle().clone()).expect("设置运行时句柄失败");

    handle
        .block_on(async {
            // 读取配置
            let server_address: &String = &configs.server.address;
            let server_port: &String = &configs.server.port;
            let use_tls: &bool = &configs.server.use_tls;

            // Ctrl+c 终止程序
            let (tx, rx) = oneshot::channel();
            let _sig = tokio::spawn(server_utils::wait_for_terminat_signal(tx));

            // 服务地址
            let address = format!("{}:{}", server_address, server_port)
                .parse()
                .expect(t!("地址或端口解析错误").as_str());

            let mut tls_configs = None;
            if *use_tls {
                let _ = tls_configs.insert(get_tls_configs(&configs));
            }

            info!("{}: {}", t!("服务监听地址-端口"), address);

            let knitter_server = KnitterServer::default();
            let account_server = AccountServer::default();

            // 初始化服务
            knitter_server.init_managers().await;
            knitter_server.init_view_rules().await;

            // 事件系统
            let event_service_configs = event_module::EventServiceConfigs {
                max_concurrent_queue: 4,
                max_event_type_queue_size: 1024,
                max_listener_instance_size: 1024,
            };

            match event_module::initialize_event_service(event_service_configs).await {
                Ok(_) => {
                    info!("{}", t!("事件系统初始化成功"));
                }
                Err(e) => {
                    error!("{}", t!("事件系统初始化失败"));
                    error!("{}", e.details());
                }
            };

            let layer = tower::ServiceBuilder::new()
                .timeout(Duration::from_secs(30))
                .layer(tonic::service::interceptor(check_auth_token))
                .into_inner();

            let knitter_service = KnitterGrpcServer::new(knitter_server)
                .send_compressed(CompressionEncoding::Gzip)
                .accept_compressed(CompressionEncoding::Gzip);

            // KnitterGrpcServer::with_interceptor( knitter_server, check_auth_token);

            let account_service = AccountGrpcServer::new(account_server);

            // 部署在ngnix后时，不使用tls， 本地测试时或者单独启动服务时使用tls
            if *use_tls {
                Server::builder()
                    .tls_config(tls_configs.unwrap())?
                    // .concurrency_limit_per_connection(32)
                    // .initial_connection_window_size(32)
                    .layer(layer)
                    .add_service(knitter_service)
                    .add_optional_service(Some(account_service))
                    .serve_with_shutdown(address, async {
                        rx.await.ok();
                        // 关闭操作
                        info!("{}", t!("服务器正常关闭"));
                    })
                    .await
            } else {
                Server::builder()
                    .add_service(knitter_service)
                    .add_optional_service(Some(account_service))
                    .serve_with_shutdown(address, async {
                        rx.await.ok();
                        // 关闭操作
                        info!("{}", t!("服务器正常关闭"));
                    })
                    .await
            }
        })
        .expect(t!("启动服务失败").as_str());

    Ok(())
}
