/*
   初始化管理器表
*/

use log::info;
use managers::{
    accounts_manager, areas_manager, comments_manager, countries_manager, datas_manager,
    groups_manager, language_codes_manager, manages_manager, messages_manager, persons_manager,
    phone_area_codes_manager, view_rules_manager,
};

use crate::managers::{
    assemblies_manager, assets_manager, cuts_manager, epics_manager, libraries_manager,
    projects_manager, sequences_manager, sets_manager,
};

use super::KnitterServer;
use managers::traits::ManagerTrait;
use view::view_rules_map;

///初始化
impl KnitterServer {
    pub async fn init_managers(&self) {
        let manager_arcs = vec![
            // common services
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
            // knitter system
            projects_manager::get_manager().await,
            libraries_manager::get_manager().await,
            sets_manager::get_manager().await,
            assemblies_manager::get_manager().await,
            assets_manager::get_manager().await,
            epics_manager::get_manager().await,
            sequences_manager::get_manager().await,
            cuts_manager::get_manager().await,
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
