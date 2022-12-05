use bson::doc;
use majordomo::{self, get_majordomo};
use managers::traits::ManagerTrait;
use service_common_handles::UnaryResponseResult;
use tonic::{Request, Response, Status};

use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::*;
use crate::services::protocol::*;

use crate::services::KnitterServer;

impl KnitterServer {
    /// 新建产品
    pub(crate) async fn handle_get_project_epics(
        &self,
        request: Request<GetProjectEpicsRequest>,
    ) -> UnaryResponseResult<GetProjectEpicsResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let project_id = &request.get_ref().project_id;

        if !view::can_collection_read(&account_id, &role_group, &PROJECTS_MANAGE_ID.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有工程可读权限"));
        }
        if !view::can_collection_read(&account_id, &role_group, &EPICS_MANAGE_ID.to_string()).await
        {
            return Err(Status::unauthenticated("用户不具有库可读权限"));
        }

        // TODO: 可能需要关联用户工程可读检查

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(EPICS_MANAGE_ID)
            .await
            .unwrap();

        let query_doc = doc! {
            EPICS_PROJECT_ID_FIELD_ID.to_string():project_id.clone(),
        };

        let result = manager.get_entities_by_filter(&Some(query_doc)).await;

        match result {
            Ok(r) => Ok(Response::new(GetProjectEpicsResponse {
                epics: r.iter().map(|x| bson::to_vec(x).unwrap()).collect(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
