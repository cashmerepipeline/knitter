/*
Project: grpc_server
Creator: 闫刚
Create time: 2020-09-24 14:30
Introduction:
*/

use super::CashmereServer;
use bson::Document;
use tokio::sync::mpsc;
// use tokio::io::AsyncReadExt;
use tonic::{Request, Response, Status};

use crate::protocol::*;
use auth::jwt::validate_is_root;
use cash_core::Manage;
use event::event;
use majordomo;
use manage_define::manage_ids::EVENTS_MANAGE_ID;
use managers::{traits::ManagerTrait, Manager};
use std::sync::Arc;
use view;
use view::ViewLevel;

type UnaryResponseResult<T> = Result<Response<T>, Status>;
type StreamResponseResult<T> = Result<Response<T>, Status>;
type EntityResponseStream = mpsc::Receiver<Result<Entity, Status>>;

impl CashmereServer {
    pub(crate) async fn handle_new_event(
        &self,
        request: Request<NewEventRequest>,
    ) -> UnaryResponseResult<NewEventResponse> {
        let metadata = request.metadata();
        let manage_id = &request.get_ref().manage_id;
        let event_name = &request.get_ref().event_name;

        let auth_token = auth::get_auth_token(metadata).unwrap();
        let claims = auth::jwt::get_claims(&auth_token).unwrap();
        let (account_id, account_name, groups) = (&claims.aud, &claims.name, &claims.roles);

        // 取得第一个可写组作为组
        let group_id = match view::get_first_write_group(groups, &manage_id.to_string()).await {
            Some(r) => r,
            None => return Err(Status::unauthenticated("用户不具有可写权限")),
        };

        let mut event_manager_arc: Option<Arc<Manager>> = None;
        {
            let major_arc = majordomo::get_majordomo().await;
            let major_lock = major_arc;
            let manager_arc = major_lock
                .get_manager_by_id(EVENTS_MANAGE_ID)
                .await
                .unwrap();
            event_manager_arc.replace(manager_arc);
        }
        let event_manager_arc = event_manager_arc.unwrap();
        let mut new_event_doc = Document::new();
        new_event_doc.insert("name", event_name);
        if let Some(id) = event_manager_arc.get_new_entity_id().await {
            new_event_doc.insert("_id", id);
        }

        match event_manager_arc
            .new_entity(&mut new_event_doc, account_id, &group_id)
            .await
        {
            Ok(r) => Ok(Response::new(NewEventResponse {
                result: "succeed".to_string(),
            })),
            Err(e) => Err(Status::internal(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }

    pub(crate) async fn handle_new_event_queue(
        &self,
        request: Request<NewEventQueueRequest>,
    ) -> UnaryResponseResult<NewEventQueueResponse> {
        let metadata = request.metadata();
        let manage_id = &request.get_ref().manage_id;
        let name_new = &request.get_ref().name;

        Err(Status::unimplemented("unimplemented"))
    }

    pub(crate) async fn handle_new_event_handle(
        &self,
        request: Request<NewEventHandleRequest>,
    ) -> UnaryResponseResult<NewEventHandleResponse> {
        let metadata = request.metadata();
        let event_id = &request.get_ref().event_id;
        let queue_id = &request.get_ref().queue_id;
        let name_new = &request.get_ref().name;

        Err(Status::unimplemented("unimplemented"))
    }
}
