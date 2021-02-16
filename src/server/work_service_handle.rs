/*
Author: 闫刚 (yes7rose@sina.com)
work_service_handle.rs (c) 2020
Desc: 工作管理服务
Created:  2020-12-18T09:12:58.129Z
Modified: !date!
*/

use bson::doc;
use bson::Document;
use chrono::Utc;
use tonic::{Request, Response, Status};

use crate::protocol::*;
use crate::CashmereServer;
use cash_core::field::ids::*;
use cash_core::results::OperationResult;
use majordomo::get_majordomo;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;

type ResponseResult<T> = Result<Response<T>, Status>;

impl CashmereServer {
    pub(crate) async fn handle_new_work(
        &self,
        request: Request<NewWorkRequest>,
    ) -> ResponseResult<NewWorkResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let work_name = &request.get_ref().name;

        if !view::can_manage_write(&account_id, &groups, manage_id).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }
        // 取得第一个可写组作为组
        let group_id = match view::get_first_write_group(&groups, &manage_id.to_string()).await {
            Some(r) => r,
            None => return Err(Status::unauthenticated("用户不具有可写权限")),
        };

        let mut new_entity_doc = Document::new();

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(WORKS_MANAGE_ID)
            .await
            .unwrap();
        let new_id = manager.get_new_entity_id().await.unwrap();

        new_entity_doc.insert("_id", new_id.to_string());
        new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.to_string());
        new_entity_doc.insert(NAME_FIELD_ID.to_string(), work_name);
        new_entity_doc.insert(WORK_MANAGE_FIELD_ID.to_string(), manage_id);

        let result = manager
            .new_entity(&mut new_entity_doc, &account_id, &group_id)
            .await;

        match result {
            Ok(r) => Ok(Response::new(NewWorkResponse {
                result: "ok".to_string(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }

    pub(crate) async fn handle_new_phase_for_work(
        &self,
        request: Request<NewPhaseForWorkRequest>,
    ) -> Result<Response<NewPhaseForWorkResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let work_id = &request.get_ref().work_id;
        let phase_name = &request.get_ref().phase_name;

        if !view::can_entity_write(
            &account_id,
            &groups,
            &WORKS_MANAGE_ID.to_string(),
            &WORK_WORK_PHASES_FIELD_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 取得第一个可写组作为组
        let group_id =
            match view::get_first_write_group(&groups, &WORKS_MANAGE_ID.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let majordomo_arc = get_majordomo().await;
        let work_manager = majordomo_arc
            .get_manager_by_id(WORKS_MANAGE_ID)
            .await
            .unwrap();

        let procedure_manager = majordomo_arc
            .get_manager_by_id(PROCEDURES_MANAGE_ID)
            .await
            .unwrap();

        // TODO: 阶段名是否已经在工作中检查

        // 新过程
        let new_procedure_id = procedure_manager.get_new_entity_id().await.unwrap();
        let mut new_procedure_doc = Document::new();
        new_procedure_doc.insert("_id", new_procedure_id.to_string());
        new_procedure_doc.insert(ID_FIELD_ID.to_string(), new_procedure_id.to_string());
        new_procedure_doc.insert(PROCEDURE_WORK_ID_FIELD_ID.to_string(), work_id);
        new_procedure_doc.insert(PROCEDURE_PHASE_NAME_FIELD_ID.to_string(), phase_name);

        let result = procedure_manager
            .new_entity(&mut new_procedure_doc, &account_id, &group_id)
            .await;

        let query_doc = doc! {
            "_id":work_id
        };
        let modify_doc = doc! {
            format!("{}.{}", WORK_WORK_PHASES_FIELD_ID.to_string(), phase_name):new_procedure_id
        };

        match result {
            Ok(r) => {
                let query_doc = doc! {
                    "_id":work_id
                };
                let modify_doc = doc! {
                    format!("{}.{}", WORK_WORK_PHASES_FIELD_ID.to_string(), phase_name):new_procedure_id
                };
                match work_manager
                    .insert_entity_map_field(query_doc, modify_doc, &account_id)
                    .await
                {
                    Err(e) => {
                        return Err(Status::aborted(format!(
                            "{} {}",
                            e.operation(),
                            e.details()
                        )));
                    }
                    _ => (),
                };

                Ok(Response::new(NewPhaseForWorkResponse {
                    result: "ok".to_string(),
                }))
            }
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }

    // pub(crate) async fn handle_new_procedure_for_work(
    //     &self,
    //     request: Request<NewProcedureForWorkRequest>,
    // ) -> Result<Response<NewProcedureForWorkResponse>, Status> {
    //     let metadata = request.metadata();
    //     // 已检查过，不需要再检查正确性
    //     let token = auth::get_auth_token(metadata).unwrap();
    //     let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
    //
    //     let work_id = &request.get_ref().work_id;
    //     let phase_index = &request.get_ref().phase_index;
    //
    //     if !view::can_entity_write(&account_id, &groups, &PROCEDURES_MANAGE_ID.to_string()).await {
    //         return Err(Status::unauthenticated("用户不具有可写权限"));
    //     }
    //
    //     // 取得第一个可写组作为组
    //     let group_id =
    //         match view::get_first_write_group(&groups, &PROCEDURES_MANAGE_ID.to_string()).await {
    //             Some(r) => r,
    //             None => return Err(Status::unauthenticated("用户不具有可写权限")),
    //         };
    //
    //     let majordomo_arc = get_majordomo().await;
    //     let procedure_manager = majordomo_arc
    //         .get_manager_by_id(PROCEDURES_MANAGE_ID)
    //         .await
    //         .unwrap();
    //     let work_manager = majordomo_arc
    //         .get_manager_by_id(WORKS_MANAGE_ID)
    //         .await
    //         .unwrap();
    //
    //     let new_id = procedure_manager.get_new_entity_id().await.unwrap();
    //     let mut new_doc = Document::new();
    //     new_doc.insert("_id", new_id);
    //     new_doc.insert(ID_FIELD_ID.to_string(), new_id);
    //     new_doc.insert(PROCEDURE_WORK_ID_FIELD_ID.to_string(), work_id);
    //     new_doc.insert(PROCEDURE_PHASE_INDEX_FIELD_ID.to_string(), phase_index);
    //
    //     let result = procedure_manager
    //         .new_entity(&mut new_doc, &account_id.to_string(), &group_id.to_string())
    //         .await;
    //
    //     match result {
    //         Ok(r) => {
    //             if let Err(e) = work_manager
    //                 .push_entity_array_field(
    //                     work_id,
    //                     &WORK_PROCEDURE_FIELD_ID.to_string(),
    //                     bson::to_bson(&doc! {phase_index:new_id}).unwrap(),
    //                     &account_id,
    //                 )
    //                 .await
    //             {
    //                 return Err(Status::aborted(format!(
    //                     "{} {}",
    //                     e.operation(),
    //                     e.details()
    //                 )));
    //             }
    //             Ok(Response::new(NewProcedureForWorkResponse {
    //                 result: "ok".to_string(),
    //             }))
    //         }
    //         Err(e) => Err(Status::aborted(format!(
    //             "{} {}",
    //             e.operation(),
    //             e.details()
    //         ))),
    //     }
    // }

    pub(crate) async fn handle_new_work_node_for_procedure_graph(
        &self,
        request: Request<NewWorkNodeForProcedureGraphRequest>,
    ) -> Result<Response<NewWorkNodeForProcedureGraphResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let procedure_id = &request.get_ref().procedure_id;
        let node_name = &request.get_ref().node_name;

        if !view::can_entity_write(
            &account_id,
            &groups,
            &PROCEDURES_MANAGE_ID.to_string(),
            &PROCEDURE_WORK_NODES_FIELD_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        if !view::can_manage_write(&account_id, &groups, &WORK_NODES_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 取得第一个可写组作为组
        let group_id =
            match view::get_first_write_group(&groups, &WORK_NODES_MANAGE_ID.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let majordomo_arc = get_majordomo().await;
        let node_manager = majordomo_arc
            .get_manager_by_id(WORK_NODES_MANAGE_ID)
            .await
            .unwrap();

        let procedure_mamager = majordomo_arc
            .get_manager_by_id(PROCEDURES_MANAGE_ID)
            .await
            .unwrap();

        let node_id = node_manager.get_new_entity_id().await.unwrap();
        let mut new_doc = Document::new();
        new_doc.insert("_id".to_string(), node_id.to_string());
        new_doc.insert(ID_FIELD_ID.to_string(), node_id.to_string());
        new_doc.insert(NAME_FIELD_ID.to_string(), node_name);
        new_doc.insert(WORK_NODE_PROCEDURE_FIELD_ID.to_string(), procedure_id);

        let result = node_manager
            .new_entity(&mut new_doc, &account_id, &group_id)
            .await;

        match result {
            Ok(r) => {
                // 添加成功后，更新过程
                let query_doc = doc! {
                    "_id":procedure_id
                };
                let modify_doc = doc! {
                     PROCEDURE_WORK_NODES_FIELD_ID.to_string():node_id
                };
                match procedure_mamager
                    .push_entity_array_field(query_doc, modify_doc, &account_id)
                    .await
                {
                    Err(e) => {
                        return Err(Status::aborted(format!(
                            "{} {}",
                            e.operation(),
                            e.details()
                        )));
                    }
                    _ => (),
                }
                Ok(Response::new(NewWorkNodeForProcedureGraphResponse {
                    result: "ok".to_string(),
                }))
            }
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }

    pub(crate) async fn handle_assign_work_node_to_worker(
        &self,
        request: Request<AssignWorkNodeToWorkerRequest>,
    ) -> Result<Response<AssignWorkNodeToWorkerResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let work_node_id = &request.get_ref().work_node_id;
        let worker_id = &request.get_ref().worker_id;

        if !view::can_entity_write(
            &account_id,
            &groups,
            &WORK_NODES_MANAGE_ID.to_string(),
            &WORK_NODE_WORKER_FIELD_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let node_manager = majordomo_arc
            .get_manager_by_id(WORK_NODES_MANAGE_ID)
            .await
            .unwrap();

        let query_doc = doc! {
            "_id":work_node_id
        };
        let modify_doc = doc! {
             WORK_NODE_WORKER_FIELD_ID.to_string():worker_id
        };

        let result = node_manager
            .update_entity_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(AssignWorkNodeToWorkerResponse {
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
