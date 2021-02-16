/*
Author: 闫刚 (yes7rose@sina.com)
task_service_handle.rs (c) 2020
Desc: 任务处理
Created:  2020-12-22T12:31:04.306Z
Modified: !date!
*/

use auth;
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

impl CashmereServer {
    pub(crate) async fn handle_new_task_for_work_node(
        &self,
        request: Request<NewTaskForWorkNodeRequest>,
    ) -> Result<Response<NewTaskForWorkNodeResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let work_node_id = &request.get_ref().work_node_id;

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
            match view::get_first_write_group(&groups, &WORK_NODES_MANAGE_ID.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let majordomo_arc = get_majordomo().await;
        let node_manager = majordomo_arc
            .get_manager_by_id(WORK_NODES_MANAGE_ID)
            .await
            .unwrap();
        let task_manager = majordomo_arc
            .get_manager_by_id(TASKS_MANAGE_ID)
            .await
            .unwrap();

        let task_id = task_manager.get_new_entity_id().await.unwrap();
        let mut new_doc = Document::new();
        new_doc.insert("_id".to_string(), task_id.to_string());
        new_doc.insert(ID_FIELD_ID.to_string(), task_id.to_string());
        new_doc.insert(TASK_WORK_NODE_FIELD_ID.to_string(), work_node_id);

        let result = task_manager
            .new_entity(&mut new_doc, &account_id, &group_id)
            .await;

        match result {
            Ok(r) => {
                // 添加成功后，更新节点
                match node_manager
                    .push_entity_array_field(
                        work_node_id,
                        &WORK_NODE_TASKS_FIELD_ID.to_string(),
                        bson::to_bson(&task_id).unwrap(),
                        &account_id,
                    )
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
                Ok(Response::new(NewTaskForWorkNodeResponse {
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

    pub(crate) async fn handle_new_task_data(
        &self,
        request: Request<NewDataForTaskRequest>,
    ) -> Result<Response<NewDataForTaskResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let task_id = &request.get_ref().task_id;

        if !view::can_entity_write(
            &account_id,
            &groups,
            &TASKS_MANAGE_ID.to_string(),
            &TASK_DATA_FIELD_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 取得第一个可写组作为组
        let group_id =
            match view::get_first_write_group(&groups, &TASKS_MANAGE_ID.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let majordomo_arc = get_majordomo().await;
        let task_manager = majordomo_arc
            .get_manager_by_id(TASKS_MANAGE_ID)
            .await
            .unwrap();
        let data_manager = majordomo_arc
            .get_manager_by_id(DATAS_MANAGE_ID)
            .await
            .unwrap();

        let data_id = data_manager.get_new_entity_id().await.unwrap();
        let mut new_doc = Document::new();
        new_doc.insert("_id".to_string(), task_id.clone());
        new_doc.insert(ID_FIELD_ID.to_string(), task_id.clone());
        new_doc.insert(DATA_TASK_FIELD_ID.to_string(), task_id.clone());

        let result = data_manager
            .new_entity(&mut new_doc, &account_id, &group_id)
            .await;

        match result {
            Ok(r) => {
                // 添加成功后，更新节点
                match task_manager
                    .update_entity_field(
                        task_id,
                        &TASK_DATA_FIELD_ID.to_string(),
                        bson::to_bson(&data_id).unwrap(),
                        &account_id,
                    )
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
                Ok(Response::new(NewDataForTaskResponse {
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

    pub(crate) async fn handle_associate_data_to_task(
        &self,
        request: Request<AssociateDataToTaskRequest>,
    ) -> Result<Response<AssociateDataToTaskResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let task_id = &request.get_ref().task_id;
        let data_id = &request.get_ref().data_id;

        if !view::can_entity_write(
            &account_id,
            &groups,
            &TASKS_MANAGE_ID.to_string(),
            &TASK_DATA_FIELD_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 取得第一个可写组作为组
        let group_id =
            match view::get_first_write_group(&groups, &TASKS_MANAGE_ID.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let majordomo_arc = get_majordomo().await;
        let task_manager = majordomo_arc
            .get_manager_by_id(TASKS_MANAGE_ID)
            .await
            .unwrap();

        let result = task_manager
            .update_entity_field(
                task_id,
                &TASK_DATA_FIELD_ID.to_string(),
                bson::to_bson(data_id).unwrap(),
                &account_id,
            )
            .await;

        match result {
            Ok(_r) => Ok(Response::new(AssociateDataToTaskResponse {
                result: "ok".to_string(),
            })),

            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }

    pub(crate) async fn handle_commit_task(
        &self,
        request: Request<CommitTaskRequest>,
    ) -> Result<Response<CommitTaskResponse>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }

    pub(crate) async fn handle_mark_task_status(
        &self,
        request: Request<MarkTaskStatusRequest>,
    ) -> Result<Response<MarkTaskStatusResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let task_id = &request.get_ref().task_id;
        let status_set_id = &request.get_ref().status_set_id;
        let status_index = &request.get_ref().status_index;

        if !view::can_entity_write(
            &account_id,
            &groups,
            &TASKS_MANAGE_ID.to_string(),
            &TASK_DATA_FIELD_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 取得第一个可写组作为组
        let group_id =
            match view::get_first_write_group(&groups, &TASKS_MANAGE_ID.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let majordomo_arc = get_majordomo().await;
        let task_manager = majordomo_arc
            .get_manager_by_id(TASKS_MANAGE_ID)
            .await
            .unwrap();
        let new_value = bson::to_bson(&doc! {status_set_id:status_index}).unwrap();
        let result = task_manager
            .update_entity_field(
                task_id,
                &TASK_STATUS_FIELD_ID.to_string(),
                new_value,
                &account_id,
            )
            .await;

        match result {
            Ok(_r) => Ok(Response::new(MarkTaskStatusResponse {
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
