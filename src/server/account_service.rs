/// 使用手机号码 密码登录
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginRequest {
    #[prost(string, tag = "1")]
    pub country_code: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub phone: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub person: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
}
/// 登出
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogoutRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogoutResponse {
    #[prost(enumeration = "LoginStatus", tag = "1")]
    pub result: i32,
}
/// 使用校验码登录
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginWithValidCodeRequest {
    #[prost(string, tag = "1")]
    pub phone: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub valid_code: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginWithValidCodeResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得校验码
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidateCodeRequest {
    #[prost(string, tag = "1")]
    pub phone: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidateCodeResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 添加账号, 需要手工添加账号的情景
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAccountRequest {
    #[prost(string, tag = "1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub department_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub phone: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAccountResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 注册账号, 用户需要自己注册账号的情景
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRequest {
    #[prost(string, tag = "1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub department_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub phone: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 自己修改手机号码
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeOwnPhoneRequest {
    #[prost(string, tag = "1")]
    pub old_phone: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_phone: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePhoneOwnResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 修改自己的密码
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeOwnPasswordRequest {
    #[prost(string, tag = "1")]
    pub old_password: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeOwnPasswordResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 重置密码, 管理员操作或者后台操作
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPasswordRequest {
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPasswordResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LoginStatus {
    Login = 0,
    Out = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountStatus {
    Stopped = 0,
    Actived = 1,
}
#[doc = r" Generated client implementations."]
pub mod account_grpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct AccountGrpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AccountGrpcClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AccountGrpcClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AccountGrpcClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            AccountGrpcClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " 登录"]
        pub async fn login(
            &mut self,
            request: impl tonic::IntoRequest<super::LoginRequest>,
        ) -> Result<tonic::Response<super::LoginResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/AccountGrpc/Login");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod account_grpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with AccountGrpcServer."]
    #[async_trait]
    pub trait AccountGrpc: Send + Sync + 'static {
        #[doc = " 登录"]
        async fn login(
            &self,
            request: tonic::Request<super::LoginRequest>,
        ) -> Result<tonic::Response<super::LoginResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct AccountGrpcServer<T: AccountGrpc> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AccountGrpc> AccountGrpcServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AccountGrpcServer<T>
    where
        T: AccountGrpc,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/account_service.AccountGrpc/Login" => {
                    #[allow(non_camel_case_types)]
                    struct LoginSvc<T: AccountGrpc>(pub Arc<T>);
                    impl<T: AccountGrpc> tonic::server::UnaryService<super::LoginRequest> for LoginSvc<T> {
                        type Response = super::LoginResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LoginRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).login(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LoginSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: AccountGrpc> Clone for AccountGrpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: AccountGrpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AccountGrpc> tonic::transport::NamedService for AccountGrpcServer<T> {
        const NAME: &'static str = "AccountGrpc";
    }
}
