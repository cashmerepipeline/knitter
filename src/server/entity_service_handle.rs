/*
Project: cashmere_server
Creator: 闫刚
Create time: 2020-12-28 13:33
Introduction:
*/

use bson::Document;
use bson::{doc, Bson};
use chrono::Utc;
use tonic::{Request, Response, Status};

use crate::protocol::*;
use crate::CashmereServer;
use cash_core::field::ids::*;
use majordomo::get_majordomo;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;

impl CashmereServer {
    // 管理实体
    pub(crate) async fn handle_new_manage_entity(
        &self,
        request: Request<NewEntityRequest>,
    ) -> Result<Response<NewEntityResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }

    pub(crate) async fn handle_edit_manage_entity(
        &self,
        request: Request<EditEntityRequest>,
    ) -> Result<Response<EditEntityResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }

    pub(crate) async fn handle_remove_manage_entity(
        &self,
        request: Request<RemoveEntityRequest>,
    ) -> Result<Response<RemoveEntityResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }

    // 实体模板
    pub(crate) async fn handle_new_entity_template(
        &self,
        request: Request<NewEntityTemplateRequest>,
    ) -> Result<Response<NewEntityTemplateResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let fields = &request.get_ref().fields;

        if !view::can_manage_write(&account_id, &groups, &TEMPLATES_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 取得第一个可写组作为组
        let group_id =
            match view::get_first_write_group(&groups, &TEMPLATES_MANAGE_ID.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let majordomo_arc = get_majordomo().await;
        let template_manager = majordomo_arc
            .get_manager_by_id(TEMPLATES_MANAGE_ID)
            .await
            .unwrap();

        if let Err(e) = template_manager.validate_data_fields(fields).await {
            return Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            )));
        }

        let fields: Vec<Bson> = fields
            .iter()
            .map(|x| {
                let b = x.clone();
                let d = Document::from_reader(&mut b.as_slice()).unwrap();
                bson::to_bson(&d).unwrap()
            })
            .collect();

        let new_id = template_manager.get_new_entity_id().await.unwrap();
        let mut new_doc = Document::new();
        new_doc.insert("_id", new_id);
        new_doc.insert(ID_FIELD_ID.to_string(), new_id);
        new_doc.insert(TEMPLATES_MANAGE_FIELD_ID.to_string(), manage_id);
        new_doc.insert(TEMPLATES_PRESETS_FIELD_ID.to_string(), fields);

        let result = template_manager
            .new_entity(&mut new_doc, &account_id.to_string(), &group_id.to_string())
            .await;

        match result {
            Ok(r) => Ok(Response::new(NewEntityTemplateResponse {
                result: "ok".to_string(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }

    pub(crate) async fn handle_edit_entity_template(
        &self,
        request: Request<EditEntityTemplateRequest>,
    ) -> Result<Response<EditEntityTemplateResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }

    pub(crate) async fn handle_remove_entity_template(
        &self,
        request: Request<RemoveEntityTemplateRequest>,
    ) -> Result<Response<RemoveEntityTemplateResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }
}
