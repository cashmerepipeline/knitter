/*
   初始化管理器表
*/


use managers::{
    groups_manager,
    areas_manager, comments_manager, country_codes_manager, 
    language_codes_manager, manages_manager, persons_manager,
    phone_area_codes_manager, view_rules_manager, 
};

use account_module::managers::{
    accounts_manager
};

use knitter_module::managers::{
    assemblies_manager, assets_manager, cuts_manager, epics_manager, 
    projects_manager, sequences_manager, sets_manager, set_collections_manager, asset_collections_manager,
};

use data_module::managers::*;
use event_module::managers::*;

use super::KnitterServer;

use view::init_view_rules;

///初始化
impl KnitterServer {
    pub async fn init_managers(&self) {
        let manager_arcs = vec![
            // cashmere services
            manages_manager::get_manager().await,
            country_codes_manager::get_manager().await,
            areas_manager::get_manager().await,
            phone_area_codes_manager::get_manager().await,
            language_codes_manager::get_manager().await,
            accounts_manager::get_manager().await,
            groups_manager::get_manager().await,
            persons_manager::get_manager().await,
            view_rules_manager::get_manager().await,
            datas_manager::get_manager().await,
            comments_manager::get_manager().await,
            // event system
            event_emitters_manager::get_manager().await,
            event_listeners_manager::get_manager().await,
            event_types_manager::get_manager().await,

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
            specses_manager::get_manager().await,
            prefabs_manager::get_manager().await,
        ];
        
        majordomo::init_managers(manager_arcs);

    }

    /// 初始化可见规则
    pub async fn init_view_rules(&self) {
        init_view_rules().await;
    }
}
