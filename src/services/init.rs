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
    assemblies_manager, assets_manager, cuts_manager, epics_manager, 
    projects_manager, sequences_manager, sets_manager, set_collections_manager, asset_collections_manager,
};

use super::KnitterServer;
use managers::traits::ManagerTrait;
use view::init_view_rules;

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
            asset_collections_manager::get_manager().await,
            set_collections_manager::get_manager().await,
            sets_manager::get_manager().await,
            assemblies_manager::get_manager().await,
            assets_manager::get_manager().await,
            epics_manager::get_manager().await,
            sequences_manager::get_manager().await,
            cuts_manager::get_manager().await,
        ];
        
        majordomo::init_managers(manager_arcs).await;

    }

    /// 初始化可见规则
    pub async fn init_view_rules(&self) {
        init_view_rules().await;
    }
}
