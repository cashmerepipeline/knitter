/// Generated server implementations.
pub mod knitter_grpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with KnitterGrpcServer.
    #[async_trait]
    pub trait KnitterGrpc: Send + Sync + 'static {
        /// 管理
        async fn get_manages(
            &self,
            request: tonic::Request<::manage_define::cashmere::GetManagesRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::GetManagesResponse>,
            tonic::Status,
        >;
        async fn get_manage_entry_count(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::GetManageEntryCountRequest,
            >,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::GetManageEntryCountResponse>,
            tonic::Status,
        >;
        async fn get_manage_schema(
            &self,
            request: tonic::Request<::manage_define::cashmere::GetManageSchemaRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::GetManageSchemaResponse>,
            tonic::Status,
        >;
        async fn mark_schema_field_removed(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::MarkSchemaFieldRemovedRequest,
            >,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::MarkSchemaFieldRemovedResponse>,
            tonic::Status,
        >;
        /// 可见性权限
        async fn change_manage_read_rule(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::ChangeManageReadRuleRequest,
            >,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::ChangeManageReadRuleResponse>,
            tonic::Status,
        >;
        async fn change_manage_write_rule(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::ChangeManageWriteRuleRequest,
            >,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::ChangeManageWriteRuleResponse>,
            tonic::Status,
        >;
        async fn change_collection_read_rule(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::ChangeCollectionReadRuleRequest,
            >,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::ChangeCollectionReadRuleResponse>,
            tonic::Status,
        >;
        async fn change_collection_write_rule(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::ChangeCollectionWriteRuleRequest,
            >,
        ) -> Result<
            tonic::Response<
                ::manage_define::cashmere::ChangeCollectionWriteRuleResponse,
            >,
            tonic::Status,
        >;
        async fn change_field_read_rule(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::ChangeFieldReadRuleRequest,
            >,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::ChangeFieldReadRuleResponse>,
            tonic::Status,
        >;
        async fn change_field_write_rule(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::ChangeFieldWriteRuleRequest,
            >,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::ChangeFieldWriteRuleResponse>,
            tonic::Status,
        >;
        /// 实体，主要是实体查询
        async fn get_entity(
            &self,
            request: tonic::Request<::manage_define::cashmere::GetEntityRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::GetEntityResponse>,
            tonic::Status,
        >;
        async fn get_entities(
            &self,
            request: tonic::Request<::manage_define::cashmere::GetEntitiesRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::GetEntitiesResponse>,
            tonic::Status,
        >;
        async fn get_entities_page(
            &self,
            request: tonic::Request<::manage_define::cashmere::GetEntitiesPageRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::GetEntitiesPageResponse>,
            tonic::Status,
        >;
        /// 编辑实体属性，非数据结构
        async fn edit_entity_field(
            &self,
            request: tonic::Request<::manage_define::cashmere::EditEntityFieldRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::EditEntityFieldResponse>,
            tonic::Status,
        >;
        /// 编辑实体属性，MAP数据结构
        async fn edit_entity_map_field(
            &self,
            request: tonic::Request<::manage_define::cashmere::EditEntityMapFieldRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::EditEntityMapFieldResponse>,
            tonic::Status,
        >;
        async fn edit_entity_map_field_remove_key(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::EditEntityMapFieldRemoveKeyRequest,
            >,
        ) -> Result<
            tonic::Response<
                ::manage_define::cashmere::EditEntityMapFieldRemoveKeyResponse,
            >,
            tonic::Status,
        >;
        /// 编辑实体属性，List数据结构
        async fn edit_entity_list_field_add_items(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::EditEntityListFieldAddItemsRequest,
            >,
        ) -> Result<
            tonic::Response<
                ::manage_define::cashmere::EditEntityListFieldAddItemsResponse,
            >,
            tonic::Status,
        >;
        async fn edit_entity_list_field_remove_items(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::EditEntityListFieldRemoveItemsRequest,
            >,
        ) -> Result<
            tonic::Response<
                ::manage_define::cashmere::EditEntityListFieldRemoveItemsResponse,
            >,
            tonic::Status,
        >;
        /// 名字
        async fn rename(
            &self,
            request: tonic::Request<::manage_define::cashmere::RenameRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::RenameResponse>,
            tonic::Status,
        >;
        async fn new_language_name(
            &self,
            request: tonic::Request<::manage_define::cashmere::NewLanguageNameRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::NewLanguageNameResponse>,
            tonic::Status,
        >;
        /// 国家
        async fn new_country(
            &self,
            request: tonic::Request<::manage_define::cashmere::NewCountryRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::NewCountryResponse>,
            tonic::Status,
        >;
        /// 语言编码
        async fn new_language_code(
            &self,
            request: tonic::Request<::manage_define::cashmere::NewLanguageCodeRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::NewLanguageCodeResponse>,
            tonic::Status,
        >;
        async fn edit_language_code(
            &self,
            request: tonic::Request<::manage_define::cashmere::EditLanguageCodeRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::EditLanguageCodeResponse>,
            tonic::Status,
        >;
        /// 组
        async fn new_group(
            &self,
            request: tonic::Request<::manage_define::cashmere::NewGroupRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::NewGroupResponse>,
            tonic::Status,
        >;
        /// 数据
        async fn new_data(
            &self,
            request: tonic::Request<::manage_define::cashmere::NewDataRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::NewDataResponse>,
            tonic::Status,
        >;
        async fn get_data_list(
            &self,
            request: tonic::Request<::manage_define::cashmere::GetDataListRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::GetDataListResponse>,
            tonic::Status,
        >;
        async fn mark_data_removed(
            &self,
            request: tonic::Request<::manage_define::cashmere::MarkDataRemovedRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::MarkDataRemovedResponse>,
            tonic::Status,
        >;
        ///Server streaming response type for the FileDataUploadFile method.
        type FileDataUploadFileStream: futures_core::Stream<
                Item = Result<
                    ::manage_define::cashmere::FileDataUploadFileResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn file_data_upload_file(
            &self,
            request: tonic::Request<
                tonic::Streaming<::manage_define::cashmere::FileDataUploadFileRequest>,
            >,
        ) -> Result<tonic::Response<Self::FileDataUploadFileStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct KnitterGrpcServer<T: KnitterGrpc> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: KnitterGrpc> KnitterGrpcServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for KnitterGrpcServer<T>
    where
        T: KnitterGrpc,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/io.knitter.KnitterGrpc/GetManages" => {
                    #[allow(non_camel_case_types)]
                    struct GetManagesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::GetManagesRequest,
                    > for GetManagesSvc<T> {
                        type Response = ::manage_define::cashmere::GetManagesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::GetManagesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_manages(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetManagesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetManageEntryCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetManageEntryCountSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::GetManageEntryCountRequest,
                    > for GetManageEntryCountSvc<T> {
                        type Response = ::manage_define::cashmere::GetManageEntryCountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::GetManageEntryCountRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_manage_entry_count(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetManageEntryCountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetManageSchema" => {
                    #[allow(non_camel_case_types)]
                    struct GetManageSchemaSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::GetManageSchemaRequest,
                    > for GetManageSchemaSvc<T> {
                        type Response = ::manage_define::cashmere::GetManageSchemaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::GetManageSchemaRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_manage_schema(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetManageSchemaSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/MarkSchemaFieldRemoved" => {
                    #[allow(non_camel_case_types)]
                    struct MarkSchemaFieldRemovedSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::MarkSchemaFieldRemovedRequest,
                    > for MarkSchemaFieldRemovedSvc<T> {
                        type Response = ::manage_define::cashmere::MarkSchemaFieldRemovedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::MarkSchemaFieldRemovedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).mark_schema_field_removed(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkSchemaFieldRemovedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ChangeManageReadRule" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeManageReadRuleSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::ChangeManageReadRuleRequest,
                    > for ChangeManageReadRuleSvc<T> {
                        type Response = ::manage_define::cashmere::ChangeManageReadRuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::ChangeManageReadRuleRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).change_manage_read_rule(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeManageReadRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ChangeManageWriteRule" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeManageWriteRuleSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::ChangeManageWriteRuleRequest,
                    > for ChangeManageWriteRuleSvc<T> {
                        type Response = ::manage_define::cashmere::ChangeManageWriteRuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::ChangeManageWriteRuleRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).change_manage_write_rule(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeManageWriteRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ChangeCollectionReadRule" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeCollectionReadRuleSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::ChangeCollectionReadRuleRequest,
                    > for ChangeCollectionReadRuleSvc<T> {
                        type Response = ::manage_define::cashmere::ChangeCollectionReadRuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::ChangeCollectionReadRuleRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).change_collection_read_rule(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeCollectionReadRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ChangeCollectionWriteRule" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeCollectionWriteRuleSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::ChangeCollectionWriteRuleRequest,
                    > for ChangeCollectionWriteRuleSvc<T> {
                        type Response = ::manage_define::cashmere::ChangeCollectionWriteRuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::ChangeCollectionWriteRuleRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).change_collection_write_rule(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeCollectionWriteRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ChangeFieldReadRule" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeFieldReadRuleSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::ChangeFieldReadRuleRequest,
                    > for ChangeFieldReadRuleSvc<T> {
                        type Response = ::manage_define::cashmere::ChangeFieldReadRuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::ChangeFieldReadRuleRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).change_field_read_rule(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeFieldReadRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ChangeFieldWriteRule" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeFieldWriteRuleSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::ChangeFieldWriteRuleRequest,
                    > for ChangeFieldWriteRuleSvc<T> {
                        type Response = ::manage_define::cashmere::ChangeFieldWriteRuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::ChangeFieldWriteRuleRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).change_field_write_rule(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeFieldWriteRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetEntity" => {
                    #[allow(non_camel_case_types)]
                    struct GetEntitySvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::GetEntityRequest,
                    > for GetEntitySvc<T> {
                        type Response = ::manage_define::cashmere::GetEntityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::GetEntityRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_entity(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEntitySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetEntities" => {
                    #[allow(non_camel_case_types)]
                    struct GetEntitiesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::GetEntitiesRequest,
                    > for GetEntitiesSvc<T> {
                        type Response = ::manage_define::cashmere::GetEntitiesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::GetEntitiesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_entities(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEntitiesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetEntitiesPage" => {
                    #[allow(non_camel_case_types)]
                    struct GetEntitiesPageSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::GetEntitiesPageRequest,
                    > for GetEntitiesPageSvc<T> {
                        type Response = ::manage_define::cashmere::GetEntitiesPageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::GetEntitiesPageRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_entities_page(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEntitiesPageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/EditEntityField" => {
                    #[allow(non_camel_case_types)]
                    struct EditEntityFieldSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::EditEntityFieldRequest,
                    > for EditEntityFieldSvc<T> {
                        type Response = ::manage_define::cashmere::EditEntityFieldResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::EditEntityFieldRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).edit_entity_field(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditEntityFieldSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/EditEntityMapField" => {
                    #[allow(non_camel_case_types)]
                    struct EditEntityMapFieldSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::EditEntityMapFieldRequest,
                    > for EditEntityMapFieldSvc<T> {
                        type Response = ::manage_define::cashmere::EditEntityMapFieldResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::EditEntityMapFieldRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).edit_entity_map_field(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditEntityMapFieldSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/EditEntityMapFieldRemoveKey" => {
                    #[allow(non_camel_case_types)]
                    struct EditEntityMapFieldRemoveKeySvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::EditEntityMapFieldRemoveKeyRequest,
                    > for EditEntityMapFieldRemoveKeySvc<T> {
                        type Response = ::manage_define::cashmere::EditEntityMapFieldRemoveKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::EditEntityMapFieldRemoveKeyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).edit_entity_map_field_remove_key(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditEntityMapFieldRemoveKeySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/EditEntityListFieldAddItems" => {
                    #[allow(non_camel_case_types)]
                    struct EditEntityListFieldAddItemsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::EditEntityListFieldAddItemsRequest,
                    > for EditEntityListFieldAddItemsSvc<T> {
                        type Response = ::manage_define::cashmere::EditEntityListFieldAddItemsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::EditEntityListFieldAddItemsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).edit_entity_list_field_add_items(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditEntityListFieldAddItemsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/EditEntityListFieldRemoveItems" => {
                    #[allow(non_camel_case_types)]
                    struct EditEntityListFieldRemoveItemsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::EditEntityListFieldRemoveItemsRequest,
                    > for EditEntityListFieldRemoveItemsSvc<T> {
                        type Response = ::manage_define::cashmere::EditEntityListFieldRemoveItemsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::EditEntityListFieldRemoveItemsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).edit_entity_list_field_remove_items(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditEntityListFieldRemoveItemsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/Rename" => {
                    #[allow(non_camel_case_types)]
                    struct RenameSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::RenameRequest,
                    > for RenameSvc<T> {
                        type Response = ::manage_define::cashmere::RenameResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::RenameRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).rename(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RenameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewLanguageName" => {
                    #[allow(non_camel_case_types)]
                    struct NewLanguageNameSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::NewLanguageNameRequest,
                    > for NewLanguageNameSvc<T> {
                        type Response = ::manage_define::cashmere::NewLanguageNameResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::NewLanguageNameRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).new_language_name(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewLanguageNameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewCountry" => {
                    #[allow(non_camel_case_types)]
                    struct NewCountrySvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::NewCountryRequest,
                    > for NewCountrySvc<T> {
                        type Response = ::manage_define::cashmere::NewCountryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::NewCountryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_country(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewCountrySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewLanguageCode" => {
                    #[allow(non_camel_case_types)]
                    struct NewLanguageCodeSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::NewLanguageCodeRequest,
                    > for NewLanguageCodeSvc<T> {
                        type Response = ::manage_define::cashmere::NewLanguageCodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::NewLanguageCodeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).new_language_code(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewLanguageCodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/EditLanguageCode" => {
                    #[allow(non_camel_case_types)]
                    struct EditLanguageCodeSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::EditLanguageCodeRequest,
                    > for EditLanguageCodeSvc<T> {
                        type Response = ::manage_define::cashmere::EditLanguageCodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::EditLanguageCodeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).edit_language_code(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditLanguageCodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewGroup" => {
                    #[allow(non_camel_case_types)]
                    struct NewGroupSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::NewGroupRequest,
                    > for NewGroupSvc<T> {
                        type Response = ::manage_define::cashmere::NewGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::NewGroupRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewData" => {
                    #[allow(non_camel_case_types)]
                    struct NewDataSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::NewDataRequest,
                    > for NewDataSvc<T> {
                        type Response = ::manage_define::cashmere::NewDataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::NewDataRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_data(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetDataList" => {
                    #[allow(non_camel_case_types)]
                    struct GetDataListSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::GetDataListRequest,
                    > for GetDataListSvc<T> {
                        type Response = ::manage_define::cashmere::GetDataListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::GetDataListRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_data_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDataListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/MarkDataRemoved" => {
                    #[allow(non_camel_case_types)]
                    struct MarkDataRemovedSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::MarkDataRemovedRequest,
                    > for MarkDataRemovedSvc<T> {
                        type Response = ::manage_define::cashmere::MarkDataRemovedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::MarkDataRemovedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).mark_data_removed(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkDataRemovedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/FileDataUploadFile" => {
                    #[allow(non_camel_case_types)]
                    struct FileDataUploadFileSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::StreamingService<
                        ::manage_define::cashmere::FileDataUploadFileRequest,
                    > for FileDataUploadFileSvc<T> {
                        type Response = ::manage_define::cashmere::FileDataUploadFileResponse;
                        type ResponseStream = T::FileDataUploadFileStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    ::manage_define::cashmere::FileDataUploadFileRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).file_data_upload_file(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FileDataUploadFileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: KnitterGrpc> Clone for KnitterGrpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: KnitterGrpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: KnitterGrpc> tonic::transport::NamedService for KnitterGrpcServer<T> {
        const NAME: &'static str = "io.knitter.KnitterGrpc";
    }
}
