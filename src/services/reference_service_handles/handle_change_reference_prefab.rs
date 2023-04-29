use bson::{doc, Document};
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
    pub(crate) async fn handle_change_reference_prefab(
        &self,
        request: Request<ChangeReferencePrefabRequest>,
    ) -> UnaryResponseResult<ChangeReferencePrefabResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let subject_manage_id = &request.get_ref().subject_manage_id;
        let subject_entity_id = &request.get_ref().subject_entity_id;
        let reference_field_id = &request.get_ref().reference_field_id;
        let old_reference = &request.get_ref().old_reference;
        let new_reference = &request.get_ref().new_reference;

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

        let mut modify_doc = Document::new();
        modify_doc.insert(
            reference_field_id,
            doc! {"$each":bson::to_document(&vec![bson::to_document(old_reference).unwrap()]).unwrap()}
        );
        let mut push_modify_doc = Document::new();
        modify_doc.insert(
            reference_field_id,
            doc! {"$each":bson::to_document(&vec![bson::to_document(new_reference).unwrap()]).unwrap()}
        );

        let pull_result = manager.add_to_array_field(query_doc.clone(), modify_doc, &account_id);
        let push_result = manager.add_to_array_field(query_doc, push_modify_doc, &account_id);

        let result = tokio::try_join!(pull_result, push_result);

        match result {
            Ok(_r) => Ok(Response::new(ChangeReferencePrefabResponse { result: "ok".to_string()})),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}

