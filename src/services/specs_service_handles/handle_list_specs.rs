use async_trait::async_trait;
use bson::{doc, Document};
use service_common_handles::UnaryResponseResult;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;
use view::{add_query_filters, get_manage_schema_view};

use crate::{
    ids_codes::{field_ids::{SPECSES_OWNER_ENTITY_ID_FIELD_ID, SPECSES_OWNER_MANAGE_ID_FIELD_ID}, manage_ids::SPECSES_MANAGE_ID},
    services::{KnitterServer, protocol::{ListSpecsResponse, ListSpecsPrefabsRequest, ListSpecsRequest}},
};

impl KnitterServer {
    /// 取得产品分页
    pub(crate) async fn handle_list_specs(
        &self,
        request: Request<ListSpecsRequest>,
    ) -> UnaryResponseResult<ListSpecsResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let owner_manage_id = &request.get_ref().owner_manage_id;
        let owner_entity_id = &request.get_ref().owner_entity_id;

        let manage_id = SPECSES_MANAGE_ID;
        // 管理可读性检查
        if !view::can_manage_read(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有管理可读权限"));
        }

        // 集合可读性检查
        if !view::can_collection_read(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有集合可读权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(manage_id).await.unwrap();

        // 可读性过滤, 没有设置过滤即不可读
        // TODO: 根据组改写，加入可读过滤项
        let mut matches = doc! {};
        if let Some(filter_doc) =
            add_query_filters(&account_id.to_string(), &role_group, &manage_id.to_string()).await
        {
            filter_doc.iter().for_each(|(k, v)| {
                matches.insert(k, v);
            });
        } else {
            return Err(Status::unauthenticated(
                "没有可读描写字段，用户不具有集合可读权限",
            ));
        };

        let filter_doc = doc! {
          SPECSES_OWNER_MANAGE_ID_FIELD_ID.to_string(): owner_manage_id,
          SPECSES_OWNER_ENTITY_ID_FIELD_ID.to_string(): owner_entity_id.clone()
        };

        let result = manager.get_entities_by_filter(&Some(filter_doc)).await;

        match result {
            Ok(entities) => Ok(Response::new(ListSpecsResponse {
                specses: entities.iter().map(|x| bson::to_vec(x).unwrap()).collect(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
