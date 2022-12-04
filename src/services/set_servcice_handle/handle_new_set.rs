use bson::{doc, Document};
use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::{
    DESCRIPTIONS_FIELD_ID, ID_FIELD_ID, NAME_MAP_FIELD_ID, TAGS_FIELD_ID,
};
use managers::traits::ManagerTrait;
use service_common_handles::name_utils::validate_name;
use service_common_handles::UnaryResponseResult;
use tonic::{Request, Response, Status};

use crate::ids_codes::manage_ids::*;
use crate::ids_codes::field_ids::*;
use crate::services::protocol::*;
use crate::services::KnitterServer;

impl KnitterServer {
    /// 新建产品
    pub(crate) async fn handle_new_set(
        &self,
        request: Request<NewSetRequest>,
    ) -> UnaryResponseResult<NewSetResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let name = &request.get_ref().name;
        let description = &request.get_ref().description;
        let collection_id = &request.get_ref().collection_id;

       if !view::can_collection_write(&account_id, &role_group, &SETS_MANAGE_ID.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        if validate_name(name).is_err() {
            return Err(Status::data_loss("名字不能为空."));
        }
        let name = name.as_ref().unwrap();

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(SETS_MANAGE_ID)
            .await
            .unwrap();

        // 新建条目
        let new_id = manager.get_new_entity_id().await.unwrap();
        let mut new_entity_doc = Document::new();
        new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.to_string());
        new_entity_doc.insert(
            NAME_MAP_FIELD_ID.to_string(),
            doc! {name.language.clone():name.name.clone()},
        );
        new_entity_doc.insert(
            SETS_ORIGINAL_COLLECTION_FIELD_ID.to_string(),
            collection_id.clone()
        );
        new_entity_doc.insert(
            DESCRIPTIONS_FIELD_ID.to_string(),
            description.clone()
        );
        
        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(NewSetResponse {
                // TODO: 发送新建事件
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


