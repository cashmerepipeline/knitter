/*
Project: grpc_server
Creator: 闫刚
Create time: 2020-09-24 14:30
Introduction:
*/

use crate::CashmereServer;
use bson::Document;
use tokio::sync::mpsc;
// use tokio::io::AsyncReadExt;
use linked_hash_map::LinkedHashMap;
use tonic::{codec::Streaming, Request, Response, Status};

use crate::protocol::*;
use auth::jwt::validate_is_root;
use cash_core::field::ids::MANAGES_SCHEMA_FIELD_ID;
use cash_core::field::{FieldDataType, Name, PropertyField};
use majordomo;
use majordomo::get_majordomo;
use manage_define::manage_ids::MANAGES_MANAGE_ID;
use managers::traits::ManagerTrait;
use view;

type StreamResponseResult<T> = Result<Response<T>, Status>;
type EntityResponseStream = mpsc::Receiver<Result<Entity, Status>>;

impl CashmereServer {
    // pub(crate) async fn handle_new_manage(
    //     &self,
    //     request: Request<NewManageRequest>,
    // ) -> UnaryResponseResult<NewManageResponse> {
    //     let metadata = request.metadata();
    //     let id_new = &request.get_ref().id;
    //     let name_new = request.get_ref().name.clone();
    //     let schema_new = request.get_ref().schema.clone();

    //     let name_doc = Document::from_reader(&mut name_new.as_slice()).unwrap();
    //     let schema_doc = Document::from_reader(&mut schema_new.as_slice()).unwrap();

    //     let auth_token = auth::get_auth_token(metadata).unwrap();
    //     let claims = auth::jwt::get_claims(&auth_token).unwrap();
    //     let (account_id, account_name, groups) = (&claims.aud, &claims.name, &claims.roles);

    //     if !view::can_manage_write(account_id, groups, &10000i32.to_string()).await {
    //         return Err(Status::unauthenticated("用户不具有可写权限"));
    //     }

    //     match manage::new_manage(*id_new, &name_doc, &schema_doc, account_id, groups).await {
    //         Ok(r) => Ok(Response::new(NewManageResponse { result: true })),
    //         Err(e) => Err(Status::internal(format!(
    //             "{} {}",
    //             e.operation(),
    //             e.details()
    //         ))),
    //     }
    // }

    /// 取得管理定义
    pub(crate) async fn handle_get_manages(
        &self,
        request: Request<GetManagesRequest>,
    ) -> StreamResponseResult<EntityResponseStream> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let managers_ids: Vec<i32> = get_majordomo().await.get_manager_ids().await;

        let (mut tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            for id in managers_ids {
                let manager = get_majordomo().await.get_manager_by_id(id).await.unwrap();
                let doc = manager.get_manage_document().await.read().clone();
                let mut data: Vec<u8> = Vec::new();
                doc.to_writer(&mut data).unwrap();
                tx.send(Ok(Entity { data: data })).await.unwrap();
            }
        });

        Ok(Response::new(rx))
    }

    /// 取得管理描写
    pub(crate) async fn handle_get_manage_schema(
        &self,
        request: Request<GetManageSchemaRequest>,
    ) -> Result<Response<GetManageSchemaResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let manage_id: i32 = request.get_ref().manage_id.parse().unwrap();

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(manage_id).await.unwrap();
        let data = manager.get_manage_schema_bytes().await;

        // let data = majordomo_arc.get_manage_schema_bytes(manage_id).await;

        match data {
            Ok(r) => Ok(Response::new(GetManageSchemaResponse { schema: r })),
            Err(e) => Err(Status::data_loss(format!(
                "取得管理描写失败 {} {} ",
                e.operation(),
                e.details()
            ))),
        }
    }

    /// 新建管理属性
    pub(crate) async fn handle_new_schema_field(
        &self,
        request: Request<NewSchemaFieldRequest>,
    ) -> Result<Response<NewSchemaFieldResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        if !view::can_manage_write(&account_id, &groups, &MANAGES_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let manage_id: i32 = request.get_ref().manage_id.parse().unwrap();
        let field: &Field = request.get_ref().field.as_ref().unwrap();

        let mut name_bytes = field.name.clone();
        let name_doc = Document::from_reader(&mut name_bytes.as_slice()).unwrap();
        let name: LinkedHashMap<String, String> = bson::from_document(name_doc).unwrap();

        let new_field: PropertyField = PropertyField {
            id: field.id,
            name: name,
            data_type: FieldDataType::from(field.data_type.clone()),
            removed: false,
        };

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(manage_id).await.unwrap();
        let result = manager.new_schema_field(new_field, &account_id).await;

        match result {
            Ok(r) => Ok(Response::new(NewSchemaFieldResponse {
                result: "ok".to_string(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }

    pub(crate) async fn handle_edit_schema_field_name(
        &self,
        request: Request<EditSchemaFieldNameRequest>,
    ) -> Result<Response<EditSchemaFieldNameResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let field_id = request.get_ref().field_id;
        let local = &request.get_ref().local;
        let new_name = &request.get_ref().new_name;

        if !view::can_manage_write(&account_id, &groups, manage_id).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(manage_id.parse().unwrap())
            .await
            .unwrap();
        let result = manager
            .edit_schema_field_name(field_id, local, new_name, &account_id)
            .await;

        match result {
            Ok(r) => Ok(Response::new(EditSchemaFieldNameResponse {
                result: "ok".to_string(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }

    pub(crate) async fn handle_remove_schema_field(
        &self,
        request: Request<RemoveSchemaFieldRequest>,
    ) -> Result<Response<RemoveSchemaFieldResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let manage_id: i32 = request.get_ref().manage_id.parse().unwrap();
        let field_id = request.get_ref().field_id;

        if !view::can_manage_write(&account_id, &groups, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(manage_id).await.unwrap();
        let result = manager
            .mark_schema_field_removed(field_id, &account_id)
            .await;

        match result {
            Ok(r) => Ok(Response::new(RemoveSchemaFieldResponse {
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
