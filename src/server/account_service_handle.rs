/*
Project: grpc_server
Creator: 闫刚
Create time: 2020-09-24 14:30
Introduction:
*/

use super::AccountServer;
use crate::protocol::*;
use account;
use auth;
use bson::Document;
use chrono::Utc;
use tonic::{Request, Response, Status};

use manage_define::manage_ids::ACCOUNTS_MANAGE_ID;
use manage_define::manage_ids::PERSONS_MANAGE_ID;
use managers::traits::ManagerTrait;

type ResponseResult<T> = Result<Response<T>, Status>;

impl AccountServer {
    pub(crate) async fn handle_login(
        &self,
        request: Request<LoginRequest>,
    ) -> ResponseResult<LoginResponse> {
        let country_code = &request.get_ref().country_code;
        let phone = &request.get_ref().phone;
        let password = &request.get_ref().password;

        // 取得账户记录
        let id: String = format!("{}{}", country_code, phone);

        // tokio::pin!(id);

        // let mut rt = Runtime::new().expect("build runtime_handle failed");
        let mut doc_op: Option<Document> = None;
        {
            let majordomo_lock_arc = majordomo::get_majordomo().await;
            let manager_arc = majordomo_lock_arc
                // .read()
                .get_manager_by_id(ACCOUNTS_MANAGE_ID)
                .await
                .unwrap();

            let account_doc = manager_arc.get_entity_by_id(&id.clone()).await;
            let account_doc = match account_doc {
                Ok(d) => d,
                Err(e) => {
                    return Err(Status::data_loss(format!(
                        "取得账户数据错误{} {}",
                        e.operation(),
                        e.details()
                    )));
                }
            };
            doc_op.replace(account_doc);
        }

        let account_doc = doc_op.unwrap();

        // 验证密码
        // println!("{}", account_doc);
        let password_hash = match account::get_account_passwd_hash(&account_doc) {
            Some(d) => d,
            None => return Err(Status::data_loss("取得账户密码错误")),
        };
        let pw_ok = match auth::jwt::verify_passwd(password, &password_hash).await {
            Some(ok) => ok,
            None => false,
        };
        if !pw_ok {
            return Err(Status::permission_denied("用户名或者密码错误"));
        }

        // 构造token
        let groups = match account::get_account_groups(&account_doc) {
            Some(a) => a,
            None => return Err(Status::data_loss("取得group数据失败")),
        };

        let token = match auth::jwt::gen_jwt_token(&id, phone, &groups).await {
            Some(t) => t,
            None => return Err(Status::data_loss("取得token数据失败")),
        };

        // 个人信息
        let mut person_bytes: Vec<u8> = Vec::new();
        let person_result = entity::get_entity_by_id(&PERSONS_MANAGE_ID.to_string(), &id).await;
        match person_result {
            Ok(p) => p.to_writer(&mut person_bytes).expect("转换记录到bytes失败"),
            Err(_e) => return Err(Status::data_loss("取得个人信息失败")),
        };

        // 更新登录时间点
        let now = Utc::now().timestamp();
        let timestamps = match account::get_account_login_timestamps(&account_doc) {
            Some(r) => r.clone(),
            None => return Err(Status::data_loss("获取时间戳失败")),
        };
        match account::update_account_login_timestamps(&id, &timestamps, now).await {
            Ok(_) => (),
            Err(_e) => return Err(Status::data_loss("更新时间戳失败")),
        };

        // 返回
        Ok(Response::new(LoginResponse {
            person: person_bytes,
            token: token,
        }))
    }
}
