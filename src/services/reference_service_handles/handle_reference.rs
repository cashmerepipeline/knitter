use bson::{doc, Document};
use majordomo::{self, get_majordomo};
use managers::traits::ManagerTrait;
use service_common_handles::UnaryResponseResult;
use tonic::{Request, Response, Status};

use manage_define::general_field_ids::ID_FIELD_ID;
use crate::ids_codes::manage_ids::*;
use crate::services::protocol::*;
use crate::services::KnitterServer;


impl KnitterServer {
    /// 新建产品
    pub(crate) async fn handle_reference_assets(
        &self,
        request: Request<ReferenceAssetsRequest>,
    ) -> UnaryResponseResult<ReferenceAssetsResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let entity_id = &request.get_ref().entity_id;
        let reference_field_id = &request.get_ref().reference_field_id;
        let references = &request.get_ref().references;

        if !view::can_collection_write(&account_id, &role_group, &manage_id.to_string())
            .await
        {
            return Err(Status::unauthenticated(format!("用户不具有可写权限：{}", manage_id)));
        }

        // TODO: 可能需要关联用户工程可读检查

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(*manage_id)
            .await
            .unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string():entity_id.clone(),
        };
        let mut modify_doc = Document::new();
        modify_doc.insert(
            reference_field_id,
            doc! {"$each":bson::to_document(references).unwrap()}
        );

        let result = manager
            .push_entity_array_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(r) => Ok(Response::new(ReferenceAssetsResponse {
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


