/*
   初始化管理器表
*/

use cash_core::view_rules;
use log::info;
use managers::{
    countries_manager,
    language_codes_manager,
    phone_area_codes_manager,
    accounts_manager, areas_manager, comments_manager, datas_manager, groups_manager,
    manages_manager, messages_manager, persons_manager, view_rules_manager,
};

use super::KnitterServer;
use managers::traits::ManagerTrait;
use view::view_rules_map;


///初始化
impl KnitterServer {
    pub async fn init_managers(&self) {
        let manager_arcs = vec![
            manages_manager::get_manager().await,
            countries_manager::get_manager().await,
            areas_manager::get_manager().await,
            phone_area_codes_manager::get_manager().await,
            language_codes_manager::get_manager().await,
            accounts_manager::get_manager().await,
            groups_manager::get_manager().await,
            persons_manager::get_manager().await,
            view_rules_manager::get_manager().await,
            datas_manager::get_manager().await,
            messages_manager::get_manager().await,
            comments_manager::get_manager().await,
        ];

        // 显示加载的管理
        info!("初始化主管");
        let majordomo_lock = majordomo::get_majordomo().await;
        info!("初始化管理映射表");
        let managers_map_lock = majordomo_lock.get_managers_map().await;
        let mut manages_map = managers_map_lock.write();
        for m in manager_arcs {
            manages_map.insert(m.get_manager_id(), m.clone());
        }

        for (k, m) in manages_map.iter() {
            info!("已加载管理: {} {}", k, m.get_manager_name())
        }
    }

    /// 初始化可见规则
    pub async fn init_view_rules(&self) {
        let _ = view_rules_map::get_view_rules_map().await;
    }
}
