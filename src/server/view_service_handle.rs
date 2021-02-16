/*
Project: grpc_server
Creator: 闫刚
Create time: 2020-10-23 17:25
Introduction:
*/

use super::CashmereServer;
use crate::protocol::{GetManageViewRequest, GetManageViewResponse};
use tonic::{Request, Response, Status};

type ResponseResult<T> = Result<Response<T>, Status>;

impl CashmereServer {
    pub(crate) async fn handle_get_manage_view(
        &self,
        request: Request<GetManageViewRequest>,
    ) -> ResponseResult<GetManageViewResponse> {
        unimplemented!()
    }
}
