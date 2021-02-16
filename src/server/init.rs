/*
Author: 闫刚 (yes7rose@sina.com)
cash_server.rs (c) 2020
Desc: 管理服务初始化
Created:  2020-11-16T01:14:05.709Z
Modified: !date!
*/

use super::CashmereServer;
use configs;
use event;
use log::info;
use majordomo;
use managers::traits::ManagerTrait;
use std::fs::FileType;
use std::path::{Path, PathBuf};

impl CashmereServer {
    pub async fn init_managers(&self) {
        // 初始化管理
        let managers_manager_lock = majordomo::get_majordomo().await;

        // 显示加载的管理
        let majordomo_lock = majordomo::get_majordomo().await;
        let managers_map_lock = majordomo_lock.get_managers_map().await;
        let manages_map = managers_map_lock.read();
        for (k, m) in manages_map.iter() {
            println!("已加载管理: {} {}", k, m.get_manager_name())
        }
    }
}
