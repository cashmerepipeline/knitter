/*
Author: 闫刚 (yes7rose@sina.com)
work_phase_service_handle (c) 2020
Desc: 工作阶段服务
Created:  2020-12-23T07:24:33.953Z
Modified: !date!
*/

use bson::doc;
use bson::Document;
use chrono::Utc;
use tonic::{Request, Response, Status};

use crate::protocol::*;
use crate::CashmereServer;
use cash_core::field::ids::*;
use manage_define::manage_ids::*;

use majordomo::get_majordomo;
use managers::traits::ManagerTrait;

type ResponseResult<T> = Result<Response<T>, Status>;

impl CashmereServer {
    pub(crate) async fn handle_new_work_phase(
        &self,
        request: Request<NewWorkPhaseRequest>,
    ) -> ResponseResult<NewWorkPhaseResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let name = &request.get_ref().name;

        if !view::can_collection_write(&account_id, &groups, &WORK_PHASES_MANAGE_ID.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 取得第一个可写组作为组
        let group_id =
            match view::get_first_write_group(&groups, &WORK_PHASES_MANAGE_ID.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let mut new_entity_doc = Document::new();

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(WORK_PHASES_MANAGE_ID)
            .await
            .unwrap();
        let new_id = manager.get_new_entity_id().await.unwrap();

        new_entity_doc.insert("_id", new_id.to_string());
        new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.to_string());
        new_entity_doc.insert(NAME_FIELD_ID.to_string(), name);

        let result = manager
            .new_entity(&mut new_entity_doc, &account_id, &group_id)
            .await;

        match result {
            Ok(r) => Ok(Response::new(NewWorkPhaseResponse {
                result: "ok".to_string(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }

    pub(crate) async fn handle_edit_work_phase(
        &self,
        request: Request<EditWorkPhaseRequest>,
    ) -> ResponseResult<EditWorkPhaseResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let phase_id = &request.get_ref().phase_id;
        let new_phase = &request.get_ref().new_phase;

        if !view::can_collection_write(&account_id, &groups, &WORK_PHASES_MANAGE_ID.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(WORK_PHASES_MANAGE_ID)
            .await
            .unwrap();

        let mut new_value = new_phase.clone();
        let new_value_docs = bson::Document::from_reader(&mut new_value.as_slice()).unwrap();
        let new_value = bson::to_bson(&new_value_docs).unwrap();

        let query_doc = doc! {
            "_id":phase_id
        };
        let modify_doc = doc! {
             WORK_PHASE_PHASES_FIELD_ID.to_string():worker_id
        };

        let result = manager
            .update_entity_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(r) => Ok(Response::new(EditWorkPhaseResponse {
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
