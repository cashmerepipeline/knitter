use bson::{doc, Document};
use majordomo::{self, get_majordomo};
use managers::traits::ManagerTrait;
use service_common_handles::UnaryResponseResult;
use tonic::{Request, Response, Status};

use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::*;
use crate::services::protocol::*;
use crate::services::KnitterServer;
use manage_define::general_field_ids::ID_FIELD_ID;

impl KnitterServer {
    /// 新建产品
    pub(crate) async fn handle_associate_asset_collections_to_project(
        &self,
        request: Request<AssociateAssetCollectionsToProjectRequest>,
    ) -> UnaryResponseResult<AssociateAssetCollectionsToProjectResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let project_id = &request.get_ref().project_id;
        let library_ids = &request.get_ref().collection_ids;

        if !view::can_collection_write(&account_id, &role_group, &PROJECTS_MANAGE_ID.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(PROJECTS_MANAGE_ID)
            .await
            .unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string():project_id.clone(),
        };
        let mut modify_doc = Document::new();
        modify_doc.insert(
            PROJECTS_ASSET_COLLECTIONS_FIELD_ID.to_string(),
            doc! {"$each":library_ids.clone()},
        );

        let result = manager
            .push_entity_array_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(AssociateAssetCollectionsToProjectResponse {
                result: "ok".to_string(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
