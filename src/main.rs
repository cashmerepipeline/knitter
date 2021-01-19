/*
Author: 闫刚 (yes7rose@sina.com)
lib (c) 2020
Desc: description
Created:  2020-09-24T06:17:47.131Z
Modified: !date!
*/

use std::fs::File;

use configs;

// 日志相关
#[macro_use]
extern crate log;

use simplelog::{CombinedLogger, LevelFilter, TermLogger, TerminalMode, WriteLogger};

use tokio;
// 终止相关
use tokio::runtime::{self};
use tokio::signal;
use tokio::sync::oneshot::{self};

use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};

use auth::check::check_auth_token;
use server::protocol::account_grpc_server::AccountGrpcServer;
use server::protocol::cashmere_grpc_server::CashmereGrpcServer;
use server::AccountServer;
use server::CashmereServer;

use runtime_handle::set_runtime_handle;

// pub static mut TOKIO_RUNTIME_HANDLE:Option<Handle> = None;

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    let log_config = simplelog::ConfigBuilder::new()
        .set_time_to_local(true)
        .set_time_format_str("%F %T")
        .build();
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, log_config.clone(), TerminalMode::Mixed),
        WriteLogger::new(
            LevelFilter::Info,
            log_config,
            File::create("log/grpc_server.log").unwrap(),
        ),
    ])
    .unwrap();

    let runtime = runtime::Runtime::new().expect("新建tokio运行时失败");
    let handle = runtime.handle().clone();
    // unsafe {
        // TOKIO_RUNTIME_HANDLE.replace(runtime_handle.handle().clone());
    // }
    set_runtime_handle(runtime.handle().clone()).expect("设置运行时句柄失败");

    handle.block_on(async {
        // 读取配置
        let configs = configs::get_configs();
        let server_address: &String = &configs.server.address;
        let server_port: &String = &configs.server.port;
        let use_tls: &bool = &configs.server.use_tls;
        let server_key_path: &String = &configs.tls.server_key_path;
        let server_ca_path: &String = &configs.tls.server_ca_path;
        let client_ca_path: &String = &configs.tls.client_ca_path;

        // Ctrl+c 终止程序
        let (tx, rx) = oneshot::channel();
        let _ = tokio::spawn(wait_for_end_signal(tx));

        // 服务地址
        let address = format!("{}:{}", server_address, server_port)
            .parse()
            .expect("address and port error");

        // tls文件读取
        let cert = tokio::fs::read(server_ca_path)
            .await
            .expect("read server crt file failed");
        let key = tokio::fs::read(server_key_path)
            .await
            .expect("read server key file failed");
        let server_identity = Identity::from_pem(cert, key);

        let client_ca_cert = tokio::fs::read(client_ca_path)
            .await
            .expect("read client crt file failed");
        let client_ca_cert = Certificate::from_pem(client_ca_cert);

        let tls = ServerTlsConfig::new()
            .identity(server_identity)
            .client_ca_root(client_ca_cert);

        info!("wav2anim server listening on: {}", address);

        let cash_server = CashmereServer::default();
        let account_server = AccountServer::default();

        // 初始化服务
        cash_server.init_managers().await;

        let cash_service = CashmereGrpcServer::with_interceptor(cash_server, check_auth_token);
        // let service = CashmereGrpcServer::new(cash_server);
        let account_service = AccountGrpcServer::new(account_server);

        // 部署在ngnix后时，不使用tls， 本地测试时或者单独启动服务时使用tls
        if *use_tls {
            Server::builder()
                .tls_config(tls)?
                .add_service(cash_service)
                .add_optional_service(Some(account_service))
                .serve_with_shutdown(address, async {
                    rx.await.ok();
                    // 关闭操作
                    info!("服务器正常关闭");
                })
                .await
        } else {
            Server::builder()
                .add_service(cash_service)
                .add_optional_service(Some(account_service))
                .serve_with_shutdown(address, async {
                    rx.await.ok();
                    // 关闭操作
                    info!("服务器正常关闭");
                })
                .await
        }
    }).expect("bad");

    Ok(())
}

// 退出程序信号
async fn wait_for_end_signal(tx: oneshot::Sender<()>) {
    let _ = signal::ctrl_c().await;
    info!("SIGINT 事件发生, 开始终止程序");
    let _ = tx.send(());
}
