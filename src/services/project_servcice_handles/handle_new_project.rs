use bson::{doc};
use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::DESCRIPTIONS_FIELD_ID;
use manage_define::general_field_ids::{
    ID_FIELD_ID, NAME_MAP_FIELD_ID,
};
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use service_common_handles::name_utils::validate_name;
use service_common_handles::UnaryResponseResult;
use tonic::{Request, Response, Status};

use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::*;
use crate::services::protocol::*;
use crate::services::KnitterServer;

impl KnitterServer {
    /// 新建产品
    pub(crate) async fn handle_new_project(
        &self,
        request: Request<NewProjectRequest>,
    ) -> UnaryResponseResult<NewProjectResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let name = &request.get_ref().name;
        let inner_root_path = &request.get_ref().inner_root_path;
        let external_root_path = &request.get_ref().external_root_path;
        let picture = &request.get_ref().picture;
        let description = &request.get_ref().description;

        if !view::can_collection_write(&account_id, &role_group, &PROJECTS_MANAGE_ID.to_string())
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
            .get_manager_by_id(PROJECTS_MANAGE_ID)
            .await
            .unwrap();

        // 新建
        if let Some(mut new_entity_doc) = make_new_entity_document(&manager).await {
            new_entity_doc.insert(
                NAME_MAP_FIELD_ID.to_string(),
                doc! {name.language.clone():name.name.clone()},
            );
            new_entity_doc.insert(
                PROJECTS_INNER_ROOT_PATH_FIELD_ID.to_string(),
                inner_root_path.clone(),
            );
            new_entity_doc.insert(
                PROJECTS_EXTERNAL_ROOT_PATH_FIELD_ID.to_string(),
                external_root_path.clone(),
            );
            new_entity_doc.insert(
                PROJECTS_PICTURE_FIELD_ID.to_string(),
                bson::to_bson(picture).unwrap(),
            );
            new_entity_doc.insert(
                DESCRIPTIONS_FIELD_ID.to_string(),
                description,
            );

            let result = manager
                .sink_entity(&mut new_entity_doc, &account_id, &role_group)
                .await;

            match result {
                Ok(r) => Ok(Response::new(NewProjectResponse {
                    // TODO: 发送新建事件
                    result: r,
                })),
                Err(e) => Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                ))),
            }
        } else {
            return Err(Status::data_loss("新建工程实体失败."));
        }
    }
}
