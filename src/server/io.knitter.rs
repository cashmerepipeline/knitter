#[doc = r" Generated server implementations."]
pub mod knitter_grpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with KnitterGrpcServer."]
    #[async_trait]
    pub trait KnitterGrpc: Send + Sync + 'static {
        #[doc = "Server streaming response type for the GetManages method."]
        type GetManagesStream: futures_core::Stream<Item = Result<::service_handles::cashmere::Entity, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " =====管理====="]
        #[doc = " 取得管理列表"]
        async fn get_manages(
            &self,
            request: tonic::Request<::service_handles::cashmere::GetManagesRequest>,
        ) -> Result<tonic::Response<Self::GetManagesStream>, tonic::Status>;
        #[doc = " =====管理描写====="]
        async fn get_manage_schema(
            &self,
            request: tonic::Request<::service_handles::cashmere::GetManageSchemaRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::GetManageSchemaResponse>,
            tonic::Status,
        >;
        async fn new_schema_field(
            &self,
            request: tonic::Request<::service_handles::cashmere::NewSchemaFieldRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::NewSchemaFieldResponse>,
            tonic::Status,
        >;
        #[doc = " 编辑属性的用处？？？编号和类型 不可变，只能修改名字"]
        async fn edit_schema_field_name(
            &self,
            request: tonic::Request<::service_handles::cashmere::EditSchemaFieldNameRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::EditSchemaFieldNameResponse>,
            tonic::Status,
        >;
        #[doc = " 设置属性 移除标记 为真"]
        async fn remove_schema_field(
            &self,
            request: tonic::Request<::service_handles::cashmere::RemoveSchemaFieldRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::RemoveSchemaFieldResponse>,
            tonic::Status,
        >;
        #[doc = " =====管理对象====="]
        async fn new_manage_entity(
            &self,
            request: tonic::Request<::service_handles::cashmere::NewEntityRequest>,
        ) -> Result<tonic::Response<::service_handles::cashmere::NewEntityResponse>, tonic::Status>;
        async fn edit_manage_entity(
            &self,
            request: tonic::Request<::service_handles::cashmere::EditEntityRequest>,
        ) -> Result<tonic::Response<::service_handles::cashmere::EditEntityResponse>, tonic::Status>;
        #[doc = " =====管理对象模板====="]
        async fn new_entity_template(
            &self,
            request: tonic::Request<::service_handles::cashmere::NewEntityTemplateRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::NewEntityTemplateResponse>,
            tonic::Status,
        >;
        async fn edit_entity_template(
            &self,
            request: tonic::Request<::service_handles::cashmere::EditEntityTemplateRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::EditEntityTemplateResponse>,
            tonic::Status,
        >;
        async fn remove_entity_template(
            &self,
            request: tonic::Request<::service_handles::cashmere::RemoveEntityTemplateRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::RemoveEntityTemplateResponse>,
            tonic::Status,
        >;
        #[doc = " =====管理映像====="]
        async fn get_manage_view(
            &self,
            request: tonic::Request<::service_handles::cashmere::GetManageViewRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::GetManageViewResponse>,
            tonic::Status,
        >;
        #[doc = " =====事件====="]
        #[doc = " 事件管理"]
        async fn new_event(
            &self,
            request: tonic::Request<::service_handles::cashmere::NewEventRequest>,
        ) -> Result<tonic::Response<::service_handles::cashmere::NewEventResponse>, tonic::Status>;
        async fn new_event_queue(
            &self,
            request: tonic::Request<::service_handles::cashmere::NewEventQueueRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::NewEventQueueResponse>,
            tonic::Status,
        >;
        async fn new_event_handle(
            &self,
            request: tonic::Request<::service_handles::cashmere::NewEventHandleRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::NewEventHandleResponse>,
            tonic::Status,
        >;
        #[doc = " =====工作====="]
        #[doc = " 工作管理"]
        async fn new_work(
            &self,
            request: tonic::Request<::service_handles::cashmere::NewWorkRequest>,
        ) -> Result<tonic::Response<::service_handles::cashmere::NewWorkResponse>, tonic::Status>;
        async fn new_phase_for_work(
            &self,
            request: tonic::Request<::service_handles::cashmere::NewPhaseForWorkRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::NewPhaseForWorkResponse>,
            tonic::Status,
        >;
        async fn new_work_node_for_procedure_graph(
            &self,
            request: tonic::Request<
                ::service_handles::cashmere::NewWorkNodeForProcedureGraphRequest,
            >,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::NewWorkNodeForProcedureGraphResponse>,
            tonic::Status,
        >;
        #[doc = " 指派工作"]
        async fn assign_work_node_to_worker(
            &self,
            request: tonic::Request<::service_handles::cashmere::AssignWorkNodeToWorkerRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::AssignWorkNodeToWorkerResponse>,
            tonic::Status,
        >;
        #[doc = " 工作节点"]
        async fn create_work_node_link(
            &self,
            request: tonic::Request<::service_handles::cashmere::CreateWorkNodeLinkRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::CreateWorkNodeLinkResponse>,
            tonic::Status,
        >;
        async fn remove_work_node_link(
            &self,
            request: tonic::Request<::service_handles::cashmere::RemoveWorkNodeLinkRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::RemoveWorkNodeLinkResponse>,
            tonic::Status,
        >;
        async fn new_data_slot_for_work_node(
            &self,
            request: tonic::Request<::service_handles::cashmere::NewDataSlotForWorkNodeRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::NewDataSlotForWorkNodeResponse>,
            tonic::Status,
        >;
        #[doc = " 新任务"]
        async fn new_task_for_work_node(
            &self,
            request: tonic::Request<::service_handles::cashmere::NewTaskForWorkNodeRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::NewTaskForWorkNodeResponse>,
            tonic::Status,
        >;
        #[doc = " 标记任务状态"]
        async fn mark_task_status(
            &self,
            request: tonic::Request<::service_handles::cashmere::MarkTaskStatusRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::MarkTaskStatusResponse>,
            tonic::Status,
        >;
        #[doc = " 提交任务"]
        async fn commit_task(
            &self,
            request: tonic::Request<::service_handles::cashmere::CommitTaskRequest>,
        ) -> Result<tonic::Response<::service_handles::cashmere::CommitTaskResponse>, tonic::Status>;
        #[doc = " =====数据====="]
        async fn new_data_for_task(
            &self,
            request: tonic::Request<::service_handles::cashmere::NewDataForTaskRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::NewDataForTaskResponse>,
            tonic::Status,
        >;
        async fn associate_data_to_task(
            &self,
            request: tonic::Request<::service_handles::cashmere::AssociateDataToTaskRequest>,
        ) -> Result<
            tonic::Response<::service_handles::cashmere::AssociateDataToTaskResponse>,
            tonic::Status,
        >;
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for KnitterGrpcServer<T>
    where
        T: KnitterGrpc,
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
                "/io.knitter.KnitterGrpc/GetManages" => {
                    #[allow(non_camel_case_types)]
                    struct GetManagesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::ServerStreamingService<
                            ::service_handles::cashmere::GetManagesRequest,
                        > for GetManagesSvc<T>
                    {
                        type Response = ::service_handles::cashmere::Entity;
                        type ResponseStream = T::GetManagesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<::service_handles::cashmere::GetManagesRequest>,
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetManageSchema" => {
                    #[allow(non_camel_case_types)]
                    struct GetManageSchemaSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::GetManageSchemaRequest,
                        > for GetManageSchemaSvc<T>
                    {
                        type Response = ::service_handles::cashmere::GetManageSchemaResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::GetManageSchemaRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_manage_schema(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewSchemaField" => {
                    #[allow(non_camel_case_types)]
                    struct NewSchemaFieldSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::NewSchemaFieldRequest,
                        > for NewSchemaFieldSvc<T>
                    {
                        type Response = ::service_handles::cashmere::NewSchemaFieldResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::NewSchemaFieldRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_schema_field(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewSchemaFieldSvc(inner);
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
                "/io.knitter.KnitterGrpc/EditSchemaFieldName" => {
                    #[allow(non_camel_case_types)]
                    struct EditSchemaFieldNameSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::EditSchemaFieldNameRequest,
                        > for EditSchemaFieldNameSvc<T>
                    {
                        type Response = ::service_handles::cashmere::EditSchemaFieldNameResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::EditSchemaFieldNameRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).edit_schema_field_name(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditSchemaFieldNameSvc(inner);
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
                "/io.knitter.KnitterGrpc/RemoveSchemaField" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveSchemaFieldSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::RemoveSchemaFieldRequest,
                        > for RemoveSchemaFieldSvc<T>
                    {
                        type Response = ::service_handles::cashmere::RemoveSchemaFieldResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::RemoveSchemaFieldRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_schema_field(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveSchemaFieldSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewManageEntity" => {
                    #[allow(non_camel_case_types)]
                    struct NewManageEntitySvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<::service_handles::cashmere::NewEntityRequest>
                        for NewManageEntitySvc<T>
                    {
                        type Response = ::service_handles::cashmere::NewEntityResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<::service_handles::cashmere::NewEntityRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_manage_entity(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewManageEntitySvc(inner);
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
                "/io.knitter.KnitterGrpc/EditManageEntity" => {
                    #[allow(non_camel_case_types)]
                    struct EditManageEntitySvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<::service_handles::cashmere::EditEntityRequest>
                        for EditManageEntitySvc<T>
                    {
                        type Response = ::service_handles::cashmere::EditEntityResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<::service_handles::cashmere::EditEntityRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).edit_manage_entity(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditManageEntitySvc(inner);
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
                "/io.knitter.KnitterGrpc/NewEntityTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct NewEntityTemplateSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::NewEntityTemplateRequest,
                        > for NewEntityTemplateSvc<T>
                    {
                        type Response = ::service_handles::cashmere::NewEntityTemplateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::NewEntityTemplateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_entity_template(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewEntityTemplateSvc(inner);
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
                "/io.knitter.KnitterGrpc/EditEntityTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct EditEntityTemplateSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::EditEntityTemplateRequest,
                        > for EditEntityTemplateSvc<T>
                    {
                        type Response = ::service_handles::cashmere::EditEntityTemplateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::EditEntityTemplateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).edit_entity_template(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditEntityTemplateSvc(inner);
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
                "/io.knitter.KnitterGrpc/RemoveEntityTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveEntityTemplateSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::RemoveEntityTemplateRequest,
                        > for RemoveEntityTemplateSvc<T>
                    {
                        type Response = ::service_handles::cashmere::RemoveEntityTemplateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::RemoveEntityTemplateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_entity_template(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveEntityTemplateSvc(inner);
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
                "/io.knitter.KnitterGrpc/GetManageView" => {
                    #[allow(non_camel_case_types)]
                    struct GetManageViewSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::GetManageViewRequest,
                        > for GetManageViewSvc<T>
                    {
                        type Response = ::service_handles::cashmere::GetManageViewResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::GetManageViewRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_manage_view(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetManageViewSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewEvent" => {
                    #[allow(non_camel_case_types)]
                    struct NewEventSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<::service_handles::cashmere::NewEventRequest>
                        for NewEventSvc<T>
                    {
                        type Response = ::service_handles::cashmere::NewEventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<::service_handles::cashmere::NewEventRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_event(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewEventSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewEventQueue" => {
                    #[allow(non_camel_case_types)]
                    struct NewEventQueueSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::NewEventQueueRequest,
                        > for NewEventQueueSvc<T>
                    {
                        type Response = ::service_handles::cashmere::NewEventQueueResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::NewEventQueueRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_event_queue(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewEventQueueSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewEventHandle" => {
                    #[allow(non_camel_case_types)]
                    struct NewEventHandleSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::NewEventHandleRequest,
                        > for NewEventHandleSvc<T>
                    {
                        type Response = ::service_handles::cashmere::NewEventHandleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::NewEventHandleRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_event_handle(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewEventHandleSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewWork" => {
                    #[allow(non_camel_case_types)]
                    struct NewWorkSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<::service_handles::cashmere::NewWorkRequest>
                        for NewWorkSvc<T>
                    {
                        type Response = ::service_handles::cashmere::NewWorkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<::service_handles::cashmere::NewWorkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_work(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewWorkSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewPhaseForWork" => {
                    #[allow(non_camel_case_types)]
                    struct NewPhaseForWorkSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::NewPhaseForWorkRequest,
                        > for NewPhaseForWorkSvc<T>
                    {
                        type Response = ::service_handles::cashmere::NewPhaseForWorkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::NewPhaseForWorkRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_phase_for_work(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewPhaseForWorkSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewWorkNodeForProcedureGraph" => {
                    #[allow(non_camel_case_types)]
                    struct NewWorkNodeForProcedureGraphSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::NewWorkNodeForProcedureGraphRequest,
                        > for NewWorkNodeForProcedureGraphSvc<T>
                    {
                        type Response =
                            ::service_handles::cashmere::NewWorkNodeForProcedureGraphResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::NewWorkNodeForProcedureGraphRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).new_work_node_for_procedure_graph(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewWorkNodeForProcedureGraphSvc(inner);
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
                "/io.knitter.KnitterGrpc/AssignWorkNodeToWorker" => {
                    #[allow(non_camel_case_types)]
                    struct AssignWorkNodeToWorkerSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::AssignWorkNodeToWorkerRequest,
                        > for AssignWorkNodeToWorkerSvc<T>
                    {
                        type Response = ::service_handles::cashmere::AssignWorkNodeToWorkerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::AssignWorkNodeToWorkerRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).assign_work_node_to_worker(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssignWorkNodeToWorkerSvc(inner);
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
                "/io.knitter.KnitterGrpc/CreateWorkNodeLink" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWorkNodeLinkSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::CreateWorkNodeLinkRequest,
                        > for CreateWorkNodeLinkSvc<T>
                    {
                        type Response = ::service_handles::cashmere::CreateWorkNodeLinkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::CreateWorkNodeLinkRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_work_node_link(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateWorkNodeLinkSvc(inner);
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
                "/io.knitter.KnitterGrpc/RemoveWorkNodeLink" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveWorkNodeLinkSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::RemoveWorkNodeLinkRequest,
                        > for RemoveWorkNodeLinkSvc<T>
                    {
                        type Response = ::service_handles::cashmere::RemoveWorkNodeLinkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::RemoveWorkNodeLinkRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_work_node_link(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveWorkNodeLinkSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewDataSlotForWorkNode" => {
                    #[allow(non_camel_case_types)]
                    struct NewDataSlotForWorkNodeSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::NewDataSlotForWorkNodeRequest,
                        > for NewDataSlotForWorkNodeSvc<T>
                    {
                        type Response = ::service_handles::cashmere::NewDataSlotForWorkNodeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::NewDataSlotForWorkNodeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).new_data_slot_for_work_node(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewDataSlotForWorkNodeSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewTaskForWorkNode" => {
                    #[allow(non_camel_case_types)]
                    struct NewTaskForWorkNodeSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::NewTaskForWorkNodeRequest,
                        > for NewTaskForWorkNodeSvc<T>
                    {
                        type Response = ::service_handles::cashmere::NewTaskForWorkNodeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::NewTaskForWorkNodeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_task_for_work_node(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewTaskForWorkNodeSvc(inner);
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
                "/io.knitter.KnitterGrpc/MarkTaskStatus" => {
                    #[allow(non_camel_case_types)]
                    struct MarkTaskStatusSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::MarkTaskStatusRequest,
                        > for MarkTaskStatusSvc<T>
                    {
                        type Response = ::service_handles::cashmere::MarkTaskStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::MarkTaskStatusRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).mark_task_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkTaskStatusSvc(inner);
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
                "/io.knitter.KnitterGrpc/CommitTask" => {
                    #[allow(non_camel_case_types)]
                    struct CommitTaskSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<::service_handles::cashmere::CommitTaskRequest>
                        for CommitTaskSvc<T>
                    {
                        type Response = ::service_handles::cashmere::CommitTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<::service_handles::cashmere::CommitTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).commit_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CommitTaskSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewDataForTask" => {
                    #[allow(non_camel_case_types)]
                    struct NewDataForTaskSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::NewDataForTaskRequest,
                        > for NewDataForTaskSvc<T>
                    {
                        type Response = ::service_handles::cashmere::NewDataForTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::NewDataForTaskRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_data_for_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewDataForTaskSvc(inner);
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
                "/io.knitter.KnitterGrpc/AssociateDataToTask" => {
                    #[allow(non_camel_case_types)]
                    struct AssociateDataToTaskSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<T: KnitterGrpc>
                        tonic::server::UnaryService<
                            ::service_handles::cashmere::AssociateDataToTaskRequest,
                        > for AssociateDataToTaskSvc<T>
                    {
                        type Response = ::service_handles::cashmere::AssociateDataToTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::service_handles::cashmere::AssociateDataToTaskRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).associate_data_to_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssociateDataToTaskSvc(inner);
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
