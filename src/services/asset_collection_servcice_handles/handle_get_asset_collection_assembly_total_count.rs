use async_trait::async_trait;
use bson::doc;
use service_common_handles::UnaryResponseResult;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;
use view;

use crate::{
    ids_codes::{
        field_ids::*,
        manage_ids::{ASSETS_MANAGE_ID, ASSET_COLLECTIONS_MANAGE_ID},
    },
    services::{
        protocol::{
            GetAssetCollectionAssemblyTotalCountRequest,
            GetAssetCollectionAssemblyTotalCountResponse,
        },
        KnitterServer,
    },
};

impl KnitterServer {
    /// 取得资产数量
    pub(crate) async fn handle_get_asset_collection_assembly_total_count(
        &self,
        request: Request<GetAssetCollectionAssemblyTotalCountRequest>,
    ) -> UnaryResponseResult<GetAssetCollectionAssemblyTotalCountResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let collection_id = &request.get_ref().collection_id;
        let manage_id = ASSETS_MANAGE_ID;

        if !view::can_collection_read(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可读权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(manage_id).await.unwrap();

        let mut filter_doc = doc! {};
        filter_doc.insert(
            ASSEMBLIES_ASSOCIATED_COLLECTIONS_FIELD_ID.to_string(),
            doc! {"$in":[collection_id]},
        );

        let result = manager.get_entry_counts(filter_doc).await;

        match result {
            Ok(r) => Ok(Response::new(
                GetAssetCollectionAssemblyTotalCountResponse { total_count: r },
            )),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
