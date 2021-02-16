#![recursion_limit = "256"]

mod account_service_handle;
mod entity_service_handle;
mod event_service_handle;
pub mod init;
mod manage_service_handle;
mod task_service_handle;
mod view_service_handle;
mod work_node_service_handle;
mod work_phase_service_handle;
mod work_service_handle;

pub mod protocol {
    include!("./swb.cashmere.rs");
}

use protocol::account_grpc_server::AccountGrpc;
use protocol::cashmere_grpc_server::CashmereGrpc;
use protocol::*;
// use protocol::{LoginRequest, LoginResponse, NewManageRequest, NewManageResponse};
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};

/// 管理服务
#[derive(Default)]
pub struct CashmereServer;

/// 账号服务
#[derive(Default)]
pub struct AccountServer;

type ResponseResult<T> = Result<Response<T>, Status>;
type UnaryResponseResult<T> = Result<Response<T>, Status>;
type StreamResponseResult<T> = Result<Response<T>, Status>;
type EntityResponseStream = mpsc::Receiver<Result<Entity, Status>>;

#[tonic::async_trait]
impl CashmereGrpc for CashmereServer {
    type GetManagesStream = mpsc::Receiver<Result<Entity, Status>>;

    // --------------------------
    // 管理
    async fn get_manages(
        &self,
        request: Request<GetManagesRequest>,
    ) -> Result<Response<Self::GetManagesStream>, Status> {
        self.handle_get_manages(request).await
    }

    // -----------------------------
    // 管理描写
    async fn get_manage_schema(
        &self,
        request: Request<GetManageSchemaRequest>,
    ) -> Result<Response<GetManageSchemaResponse>, Status> {
        self.handle_get_manage_schema(request).await
    }

    async fn new_schema_field(
        &self,
        request: Request<NewSchemaFieldRequest>,
    ) -> Result<Response<NewSchemaFieldResponse>, Status> {
        self.handle_new_schema_field(request).await
    }

    async fn edit_schema_field_name(
        &self,
        request: Request<EditSchemaFieldNameRequest>,
    ) -> Result<Response<EditSchemaFieldNameResponse>, Status> {
        self.handle_edit_schema_field_name(request).await
    }

    async fn remove_schema_field(
        &self,
        request: Request<RemoveSchemaFieldRequest>,
    ) -> Result<Response<RemoveSchemaFieldResponse>, Status> {
        self.handle_remove_schema_field(request).await
    }

    // 管理实体
    async fn new_manage_entity(
        &self,
        request: Request<NewEntityRequest>,
    ) -> Result<Response<NewEntityResponse>, Status> {
        self.handle_new_manage_entity(request).await
    }

    async fn edit_manage_entity(
        &self,
        request: Request<EditEntityRequest>,
    ) -> Result<Response<EditEntityResponse>, Status> {
        self.handle_edit_manage_entity(request).await
    }

    async fn remove_manage_entity(
        &self,
        request: Request<RemoveEntityRequest>,
    ) -> Result<Response<RemoveEntityResponse>, Status> {
        self.handle_remove_manage_entity(request).await
    }

    // 实体模板
    async fn new_entity_template(
        &self,
        request: Request<NewEntityTemplateRequest>,
    ) -> Result<Response<NewEntityTemplateResponse>, Status> {
        self.handle_new_entity_template(request).await
    }

    async fn edit_entity_template(
        &self,
        request: Request<EditEntityTemplateRequest>,
    ) -> Result<Response<EditEntityTemplateResponse>, Status> {
        self.handle_edit_entity_template(request).await
    }

    async fn remove_entity_template(
        &self,
        request: Request<RemoveEntityTemplateRequest>,
    ) -> Result<Response<RemoveEntityTemplateResponse>, Status> {
        self.handle_remove_entity_template(request).await
    }

    // 管理映像
    async fn get_manage_view(
        &self,
        request: Request<GetManageViewRequest>,
    ) -> ResponseResult<GetManageViewResponse> {
        self.handle_get_manage_view(request).await
    }

    //-------------------
    // 事件
    async fn new_event(
        &self,
        request: Request<NewEventRequest>,
    ) -> ResponseResult<NewEventResponse> {
        self.handle_new_event(request).await
    }

    async fn new_event_queue(
        &self,
        request: Request<NewEventQueueRequest>,
    ) -> ResponseResult<NewEventQueueResponse> {
        self.handle_new_event_queue(request).await
    }

    async fn new_event_handle(
        &self,
        request: Request<NewEventHandleRequest>,
    ) -> ResponseResult<NewEventHandleResponse> {
        self.handle_new_event_handle(request).await
    }

    //-------------------
    // 工作
    async fn new_work(
        &self,
        request: Request<NewWorkRequest>,
    ) -> Result<Response<NewWorkResponse>, Status> {
        self.handle_new_work(request).await
    }

    async fn new_phase_for_work(
        &self,
        request: Request<NewPhaseForWorkRequest>,
    ) -> Result<Response<NewPhaseForWorkResponse>, Status> {
        self.handle_new_phase_for_work(request).await
    }

    //-------------------
    // 工作
    async fn new_work_node_for_procedure_graph(
        &self,
        request: Request<NewWorkNodeForProcedureGraphRequest>,
    ) -> Result<Response<NewWorkNodeForProcedureGraphResponse>, Status> {
        self.handle_new_work_node_for_procedure_graph(request).await
    }

    async fn assign_work_node_to_worker(
        &self,
        request: Request<AssignWorkNodeToWorkerRequest>,
    ) -> UnaryResponseResult<AssignWorkNodeToWorkerResponse> {
        self.handle_assign_work_node_to_worker(request).await
    }

    // 工作节点
    async fn new_data_slot_for_work_node(
        &self,
        request: Request<NewDataSlotForWorkNodeRequest>,
    ) -> UnaryResponseResult<NewDataSlotForWorkNodeResponse> {
        self.handle_new_data_slot_for_work_node(request).await
    }

    async fn create_work_node_link(
        &self,
        request: Request<CreateWorkNodeLinkRequest>,
    ) -> Result<Response<CreateWorkNodeLinkResponse>, Status> {
        self.handle_create_work_node_link(request).await
    }

    async fn remove_work_node_link(
        &self,
        request: Request<RemoveWorkNodeLinkRequest>,
    ) -> Result<Response<RemoveWorkNodeLinkResponse>, Status> {
        self.handle_remove_work_node_link(request).await
    }

    //-------------------
    // 任务
    async fn new_task_for_work_node(
        &self,
        request: Request<NewTaskForWorkNodeRequest>,
    ) -> Result<Response<NewTaskForWorkNodeResponse>, Status> {
        self.handle_new_task_for_work_node(request).await
    }

    async fn mark_task_status(
        &self,
        request: Request<MarkTaskStatusRequest>,
    ) -> Result<Response<MarkTaskStatusResponse>, Status> {
        self.handle_mark_task_status(request).await
    }

    async fn commit_task(
        &self,
        request: Request<CommitTaskRequest>,
    ) -> Result<Response<CommitTaskResponse>, Status> {
        self.handle_commit_task(request).await
    }

    async fn new_data_for_task(
        &self,
        request: Request<NewDataForTaskRequest>,
    ) -> Result<Response<NewDataForTaskResponse>, Status> {
        self.handle_new_task_data(request).await
    }

    async fn associate_data_to_task(
        &self,
        request: Request<AssociateDataToTaskRequest>,
    ) -> Result<Response<AssociateDataToTaskResponse>, Status> {
        self.handle_associate_data_to_task(request).await
    }
}

#[tonic::async_trait]
impl AccountGrpc for AccountServer {
    async fn login(&self, request: Request<LoginRequest>) -> ResponseResult<LoginResponse> {
        self.handle_login(request).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
