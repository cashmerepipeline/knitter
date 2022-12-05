use bson::doc;
use cash_result::{Failed, OperationResult};
use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::ID_FIELD_ID;
use managers::traits::ManagerTrait;
use service_common_handles::UnaryResponseResult;
use tonic::{Request, Response, Status};

use crate::services::protocol::*;

use crate::services::KnitterServer;

impl KnitterServer {
    /// 新建产品
    pub(crate) async fn handle_list_references(
        &self,
        request: Request<ListReferencesRequest>,
    ) -> UnaryResponseResult<ListReferencesResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let subject_manage_id = &request.get_ref().subject_manage_id;
        let subject_entity_id = &request.get_ref().subject_entity_id;
        let reference_field_id = &request.get_ref().reference_field_id;

        if !view::can_collection_read(&account_id, &role_group, &subject_manage_id.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有可读权限"));
        }

        // TODO: 可能需要关联用户工程可读检查

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(*subject_manage_id)
            .await
            .unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string():subject_entity_id.clone(),
        };

        let result = manager.get_entity_by_id(subject_entity_id).await;

        let result = if let Ok(refs) = result {
            match refs.get_array(reference_field_id) {
                Ok(r) => Ok(r
                    .to_vec()
                    .iter()
                    .map(|x| bson::to_vec(x).unwrap())
                    .collect()),
                _ => Err(OperationResult::Failed(Failed {
                    details: "没有参考".to_string(),
                    operation: "handle_list_references".to_string(),
                })),
            }
        } else {
            return Err(Status::data_loss("实体不存在"));
        };

        match result {
            Ok(r) => Ok(Response::new(ListReferencesResponse { references: r })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
