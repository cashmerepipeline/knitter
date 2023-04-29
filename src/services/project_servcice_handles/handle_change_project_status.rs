use async_trait::async_trait;
use bson::doc;
use service_common_handles::UnaryResponseResult;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use view;

use crate::ids_codes::field_ids::PROJECTS_STATUS_FIELD_ID;
use crate::ids_codes::manage_ids::PROJECTS_MANAGE_ID;
use crate::services::*;

impl KnitterServer {
    pub(crate) async fn handle_change_project_status(
        &self,
        request: Request<ChangeProjectStatusRequest>,
    ) -> UnaryResponseResult<ChangeProjectStatusResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let project_id = &request.get_ref().project_id;
        let status = &request.get_ref().status;
        let manage_id = PROJECTS_MANAGE_ID;

        if !view::can_collection_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有集合可写权限"));
        }
        if !view::can_entity_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有实体可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(manage_id).await.unwrap();

        let mut query_doc = doc! {};
        query_doc.insert(ID_FIELD_ID.to_string(), project_id);
        
        if !manager.entity_exists(&query_doc).await{
            return Err(Status::data_loss(t!("工程不存在")));
        };

        let mut modify_doc = doc! {};
        modify_doc.insert(PROJECTS_STATUS_FIELD_ID.to_string(), status);

        let result = manager.update_entity_field(query_doc, &mut modify_doc, &account_id).await;

        match result {
            Ok(_r) => Ok(Response::new(ChangeProjectStatusResponse {
                status: *status,
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
