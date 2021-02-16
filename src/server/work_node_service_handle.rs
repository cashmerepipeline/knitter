/*
Author: 闫刚 (yes7rose@sina.com)
work_node_service_handle.rs (c) 2020
Desc: 工作节点服务
Created:  2020-12-23T13:04:04.919Z
Modified: !date!
*/

use bson::doc;
use bson::Document;
use chrono::Utc;
use tonic::{Request, Response, Status};

use crate::protocol::*;
use crate::CashmereServer;
use cash_core::field::ids::*;
use majordomo::get_majordomo;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;

use crate::UnaryResponseResult;

type ResponseResult<T> = Result<Response<T>, Status>;

// TODO:: SLotTYpe 已经移动

impl CashmereServer {
    pub(crate) async fn handle_create_work_node_link(
        &self,
        request: Request<CreateWorkNodeLinkRequest>,
    ) -> ResponseResult<CreateWorkNodeLinkResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let procedure_id = &request.get_ref().procedure_id;
        let start_node_id = &request.get_ref().start_node_id;
        let out_slot = &request.get_ref().out_slot;
        let end_node_id = &request.get_ref().end_node_id;
        let in_slot = &request.get_ref().in_slot;

        if !view::can_manage_write(&account_id, &groups, &PROCEDURES_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(PROCEDURES_MANAGE_ID)
            .await
            .unwrap();

        let mut link = Document::new();
        link.insert("start_node", start_node_id);
        link.insert("out_slot", start_node_id);
        link.insert("end_node", start_node_id);
        link.insert("in_slot", start_node_id);

        let result = manager
            .push_entity_array_field(
                procedure_id,
                &PROCEDURE_LINKS_FIELD_ID.to_string(),
                bson::to_bson(&link).unwrap(),
                &account_id,
            )
            .await;

        match result {
            Ok(r) => Ok(Response::new(CreateWorkNodeLinkResponse {
                result: "ok".to_string(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }

    pub(crate) async fn handle_remove_work_node_link(
        &self,
        request: Request<RemoveWorkNodeLinkRequest>,
    ) -> ResponseResult<RemoveWorkNodeLinkResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let procedure_id = &request.get_ref().procedure_id;
        let start_node_id = &request.get_ref().start_node_id;
        let out_slot = &request.get_ref().out_slot;
        let end_node_id = &request.get_ref().end_node_id;
        let in_slot = &request.get_ref().in_slot;

        let end_node_id = &request.get_ref().end_node_id;

        if !view::can_manage_write(&account_id, &groups, &PROCEDURES_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(PROCEDURES_MANAGE_ID)
            .await
            .unwrap();

        let mut link = Document::new();
        link.insert("start_node", start_node_id);
        link.insert("out_slot", start_node_id);
        link.insert("end_node", start_node_id);
        link.insert("in_slot", start_node_id);

        let result = manager
            .pull_entity_array_field(
                procedure_id,
                &PROCEDURE_LINKS_FIELD_ID.to_string(),
                bson::to_bson(&link).unwrap(),
                &account_id,
            )
            .await;

        match result {
            Ok(r) => Ok(Response::new(RemoveWorkNodeLinkResponse {
                result: "ok".to_string(),
            })),

            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }

    pub(crate) async fn handle_new_data_slot_for_work_node(
        &self,
        request: Request<NewDataSlotForWorkNodeRequest>,
    ) -> UnaryResponseResult<NewDataSlotForWorkNodeResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let node_id = &request.get_ref().node_id;
        let slot_name = &request.get_ref().slot_name;
        let slot_type = &request.get_ref().slot_type;
        let slot_type = SlotType::from(slot_type).unwrap();

        if !view::can_entity_write(
            &account_id,
            &groups,
            &WORK_NODES_MANAGE_ID.to_string(),
            &WORK_NODE_REFERENCE_DATAS_FIELD_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(WORK_NODES_MANAGE_ID)
            .await
            .unwrap();

        let result = match slot_type {
            SlotType::RefrenceData => {
                manager
                    .insert_entity_map_field(
                        node_id,
                        &WORK_NODE_REFERENCE_DATAS_FIELD_ID.to_string(),
                        slot_name,
                        bson::to_bson("").unwrap(),
                        &account_id,
                    )
                    .await
            }
            SlotType::DepedentData => {
                manager
                    .insert_entity_map_field(
                        node_id,
                        &WORK_NODE_DEPENDED_DATAS_FIELD_ID.to_string(),
                        slot_name,
                        bson::to_bson("").unwrap(),
                        &account_id,
                    )
                    .await
            }
            SlotType::WorkData => {
                manager
                    .insert_entity_map_field(
                        node_id,
                        &WORK_NODE_WORK_DATAS_FIELD_ID.to_string(),
                        slot_name,
                        bson::to_bson("").unwrap(),
                        &account_id,
                    )
                    .await
            }
            SlotType::OutData => {
                manager
                    .insert_entity_map_field(
                        node_id,
                        &WORK_NODE_OUT_DATAS_FIELD_ID.to_string(),
                        slot_name,
                        bson::to_bson("").unwrap(),
                        &account_id,
                    )
                    .await
            }
        };

        match result {
            Ok(r) => Ok(Response::new(NewDataSlotForWorkNodeResponse {
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
