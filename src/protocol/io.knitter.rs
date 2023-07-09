/// Generated server implementations.
pub mod knitter_grpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with KnitterGrpcServer.
    #[async_trait]
    pub trait KnitterGrpc: Send + Sync + 'static {
        /// 管理
        async fn get_manages(
            &self,
            request: tonic::Request<::manage_define::cashmere::GetManagesRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::GetManagesResponse>,
            tonic::Status,
        >;
        async fn get_manage_entry_count(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::GetManageEntryCountRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::GetManageEntryCountResponse>,
            tonic::Status,
        >;
        async fn get_manage_schema(
            &self,
            request: tonic::Request<::manage_define::cashmere::GetManageSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::GetManageSchemaResponse>,
            tonic::Status,
        >;
        async fn mark_schema_field_removed(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::MarkSchemaFieldRemovedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::MarkSchemaFieldRemovedResponse>,
            tonic::Status,
        >;
        /// 可见性权限
        async fn change_manage_read_rule(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::ChangeManageReadRuleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::ChangeManageReadRuleResponse>,
            tonic::Status,
        >;
        async fn change_manage_write_rule(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::ChangeManageWriteRuleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::ChangeManageWriteRuleResponse>,
            tonic::Status,
        >;
        async fn change_collection_read_rule(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::ChangeCollectionReadRuleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::ChangeCollectionReadRuleResponse>,
            tonic::Status,
        >;
        async fn change_collection_write_rule(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::ChangeCollectionWriteRuleRequest,
            >,
        ) -> std::result::Result<
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
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::ChangeFieldReadRuleResponse>,
            tonic::Status,
        >;
        async fn change_field_write_rule(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::ChangeFieldWriteRuleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::ChangeFieldWriteRuleResponse>,
            tonic::Status,
        >;
        /// 实体，主要是实体查询
        async fn get_entity(
            &self,
            request: tonic::Request<::manage_define::cashmere::GetEntityRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::GetEntityResponse>,
            tonic::Status,
        >;
        async fn get_entities(
            &self,
            request: tonic::Request<::manage_define::cashmere::GetEntitiesRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::GetEntitiesResponse>,
            tonic::Status,
        >;
        async fn get_entities_page(
            &self,
            request: tonic::Request<::manage_define::cashmere::GetEntitiesPageRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::GetEntitiesPageResponse>,
            tonic::Status,
        >;
        async fn mark_entity_removed(
            &self,
            request: tonic::Request<::manage_define::cashmere::MarkEntityRemovedRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::MarkEntityRemovedResponse>,
            tonic::Status,
        >;
        async fn recover_removed_entity(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::RecoverRemovedEntityRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::RecoverRemovedEntityResponse>,
            tonic::Status,
        >;
        async fn get_removed_entities_page(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::GetRemovedEntitiesPageRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::GetRemovedEntitiesPageResponse>,
            tonic::Status,
        >;
        /// 通用编辑实体属性，非数据结构
        async fn edit_entity_field(
            &self,
            request: tonic::Request<::manage_define::cashmere::EditEntityFieldRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::EditEntityFieldResponse>,
            tonic::Status,
        >;
        /// 通用编辑实体属性，MAP数据结构
        async fn edit_entity_map_field(
            &self,
            request: tonic::Request<::manage_define::cashmere::EditEntityMapFieldRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::EditEntityMapFieldResponse>,
            tonic::Status,
        >;
        async fn edit_entity_map_field_remove_key(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::EditEntityMapFieldRemoveKeyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::manage_define::cashmere::EditEntityMapFieldRemoveKeyResponse,
            >,
            tonic::Status,
        >;
        /// 通用编辑实体属性，List数据结构
        async fn edit_entity_array_field_add_items(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::EditEntityArrayFieldAddItemsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::manage_define::cashmere::EditEntityArrayFieldAddItemsResponse,
            >,
            tonic::Status,
        >;
        async fn edit_entity_array_field_remove_items(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::EditEntityArrayFieldRemoveItemsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::manage_define::cashmere::EditEntityArrayFieldRemoveItemsResponse,
            >,
            tonic::Status,
        >;
        /// 名字
        async fn rename(
            &self,
            request: tonic::Request<::manage_define::cashmere::RenameRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::RenameResponse>,
            tonic::Status,
        >;
        async fn new_language_name(
            &self,
            request: tonic::Request<::manage_define::cashmere::NewLanguageNameRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::NewLanguageNameResponse>,
            tonic::Status,
        >;
        /// 国家
        async fn new_country_code(
            &self,
            request: tonic::Request<::manage_define::cashmere::NewCountryCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::NewCountryCodeResponse>,
            tonic::Status,
        >;
        async fn get_country_codes(
            &self,
            request: tonic::Request<::manage_define::cashmere::GetCountryCodesRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::GetCountryCodesResponse>,
            tonic::Status,
        >;
        /// 语言编码
        async fn new_language_code(
            &self,
            request: tonic::Request<::manage_define::cashmere::NewLanguageCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::NewLanguageCodeResponse>,
            tonic::Status,
        >;
        async fn get_language_codes(
            &self,
            request: tonic::Request<::manage_define::cashmere::GetLanguageCodesRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::GetLanguageCodesResponse>,
            tonic::Status,
        >;
        /// 手机区号
        async fn new_phone_area_code(
            &self,
            request: tonic::Request<::manage_define::cashmere::NewPhoneAreaCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::NewPhoneAreaCodeResponse>,
            tonic::Status,
        >;
        async fn get_phone_area_codes(
            &self,
            request: tonic::Request<::manage_define::cashmere::GetPhoneAreaCodesRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::GetPhoneAreaCodesResponse>,
            tonic::Status,
        >;
        /// 组
        async fn new_group(
            &self,
            request: tonic::Request<::manage_define::cashmere::NewGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<::manage_define::cashmere::NewGroupResponse>,
            tonic::Status,
        >;
        /// 数据
        async fn get_data_server_configs(
            &self,
            request: tonic::Request<
                ::data_module::protocols::GetDataServerConfigsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::GetDataServerConfigsResponse>,
            tonic::Status,
        >;
        async fn new_data(
            &self,
            request: tonic::Request<::data_module::protocols::NewDataRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::NewDataResponse>,
            tonic::Status,
        >;
        async fn list_entity_data(
            &self,
            request: tonic::Request<::data_module::protocols::ListEntityDataRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::ListEntityDataResponse>,
            tonic::Status,
        >;
        async fn get_data_info(
            &self,
            request: tonic::Request<::data_module::protocols::GetDataInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::GetDataInfoResponse>,
            tonic::Status,
        >;
        async fn mark_data_removed(
            &self,
            request: tonic::Request<::data_module::protocols::MarkDataRemovedRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::MarkDataRemovedResponse>,
            tonic::Status,
        >;
        /// 规格
        async fn new_specs(
            &self,
            request: tonic::Request<::data_module::protocols::NewSpecsRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::NewSpecsResponse>,
            tonic::Status,
        >;
        async fn list_specs(
            &self,
            request: tonic::Request<::data_module::protocols::ListSpecsRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::ListSpecsResponse>,
            tonic::Status,
        >;
        async fn list_specs_prefabs(
            &self,
            request: tonic::Request<::data_module::protocols::ListSpecsPrefabsRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::ListSpecsPrefabsResponse>,
            tonic::Status,
        >;
        /// 数据阶段
        async fn new_stage(
            &self,
            request: tonic::Request<::data_module::protocols::NewStageRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::NewStageResponse>,
            tonic::Status,
        >;
        async fn list_stages(
            &self,
            request: tonic::Request<::data_module::protocols::ListStagesRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::ListStagesResponse>,
            tonic::Status,
        >;
        /// 预制件
        async fn new_prefab(
            &self,
            request: tonic::Request<::data_module::protocols::NewPrefabRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::NewPrefabResponse>,
            tonic::Status,
        >;
        /// 阶段版本
        async fn add_stage_version(
            &self,
            request: tonic::Request<::data_module::protocols::AddStageVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::AddStageVersionResponse>,
            tonic::Status,
        >;
        async fn list_stage_versions(
            &self,
            request: tonic::Request<::data_module::protocols::ListStageVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::ListStageVersionsResponse>,
            tonic::Status,
        >;
        async fn set_stage_current_version(
            &self,
            request: tonic::Request<
                ::data_module::protocols::SetStageCurrentVersionRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::SetStageCurrentVersionResponse>,
            tonic::Status,
        >;
        async fn remove_stage_version(
            &self,
            request: tonic::Request<::data_module::protocols::RemoveStageVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::RemoveStageVersionResponse>,
            tonic::Status,
        >;
        /// 文件操作
        async fn add_file_to_version(
            &self,
            request: tonic::Request<::data_module::protocols::AddFileToVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::AddFileToVersionResponse>,
            tonic::Status,
        >;
        async fn add_file_set_to_version(
            &self,
            request: tonic::Request<::data_module::protocols::AddFileSetToVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::AddFileSetToVersionResponse>,
            tonic::Status,
        >;
        async fn add_file_sequence_to_version(
            &self,
            request: tonic::Request<
                ::data_module::protocols::AddFileSequenceToVersionRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::AddFileSequenceToVersionResponse>,
            tonic::Status,
        >;
        /// 只对文件和文件集有效，文件序列不支持
        async fn remove_files_from_version(
            &self,
            request: tonic::Request<
                ::data_module::protocols::RemoveFilesFromVersionRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::RemoveFilesFromVersionResponse>,
            tonic::Status,
        >;
        async fn list_version_folder(
            &self,
            request: tonic::Request<::data_module::protocols::ListVersionFolderRequest>,
        ) -> std::result::Result<
            tonic::Response<::data_module::protocols::ListVersionFolderResponse>,
            tonic::Status,
        >;
        /// 手动删除版本文件夹下的无关文件或文件夹
        async fn delete_version_folder_entries(
            &self,
            request: tonic::Request<
                ::data_module::protocols::DeleteVersionFolderEntriesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::data_module::protocols::DeleteVersionFolderEntriesResponse,
            >,
            tonic::Status,
        >;
        /// Server streaming response type for the UploadFile method.
        type UploadFileStream: futures_core::Stream<
                Item = std::result::Result<
                    ::data_module::protocols::UploadFileResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn upload_file(
            &self,
            request: tonic::Request<
                tonic::Streaming<::data_module::protocols::UploadFileRequest>,
            >,
        ) -> std::result::Result<tonic::Response<Self::UploadFileStream>, tonic::Status>;
        /// Server streaming response type for the DownloadFile method.
        type DownloadFileStream: futures_core::Stream<
                Item = std::result::Result<
                    ::data_module::protocols::DownloadFileResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn download_file(
            &self,
            request: tonic::Request<
                tonic::Streaming<::data_module::protocols::DownloadFileRequest>,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::DownloadFileStream>,
            tonic::Status,
        >;
        /// 事件
        async fn register_event_type(
            &self,
            request: tonic::Request<::event_module::protocols::RegisterEventTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<::event_module::protocols::RegisterEventTypeResponse>,
            tonic::Status,
        >;
        /// rpc DeregisterEventType(.event.cashmere.DeregisterEventTypeRequest) returns (.event.cashmere.DeregisterEventTypeResponse);
        async fn register_event_emitter(
            &self,
            request: tonic::Request<
                ::event_module::protocols::RegisterEventEmitterRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::event_module::protocols::RegisterEventEmitterResponse>,
            tonic::Status,
        >;
        async fn register_event_listener(
            &self,
            request: tonic::Request<
                ::event_module::protocols::RegisterEventListenerRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::event_module::protocols::RegisterEventListenerResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the EmitEvent method.
        type EmitEventStream: futures_core::Stream<
                Item = std::result::Result<
                    ::event_module::protocols::EmitEventResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn emit_event(
            &self,
            request: tonic::Request<::event_module::protocols::EmitEventRequest>,
        ) -> std::result::Result<tonic::Response<Self::EmitEventStream>, tonic::Status>;
        /// Server streaming response type for the ListenEventType method.
        type ListenEventTypeStream: futures_core::Stream<
                Item = std::result::Result<
                    ::event_module::protocols::ListenEventTypeResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn listen_event_type(
            &self,
            request: tonic::Request<::event_module::protocols::ListenEventTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ListenEventTypeStream>,
            tonic::Status,
        >;
        /// 项目
        async fn new_project(
            &self,
            request: tonic::Request<::knitter_module::protocols::NewProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::NewProjectResponse>,
            tonic::Status,
        >;
        async fn associate_asset_collections_to_project(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::AssociateAssetCollectionsToProjectRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::knitter_module::protocols::AssociateAssetCollectionsToProjectResponse,
            >,
            tonic::Status,
        >;
        async fn deassociate_asset_collections_from_project(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::DeassociateAssetCollectionsFromProjectRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::knitter_module::protocols::DeassociateAssetCollectionsFromProjectResponse,
            >,
            tonic::Status,
        >;
        async fn associate_set_collections_to_project(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::AssociateSetCollectionsToProjectRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::knitter_module::protocols::AssociateSetCollectionsToProjectResponse,
            >,
            tonic::Status,
        >;
        async fn deassociate_set_collections_from_project(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::DeassociateSetCollectionsFromProjectRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::knitter_module::protocols::DeassociateSetCollectionsFromProjectResponse,
            >,
            tonic::Status,
        >;
        async fn get_project_associated_asset_collections(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::GetProjectAssociatedAssetCollectionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::knitter_module::protocols::GetProjectAssociatedAssetCollectionsResponse,
            >,
            tonic::Status,
        >;
        async fn get_project_associated_set_collections(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::GetProjectAssociatedSetCollectionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::knitter_module::protocols::GetProjectAssociatedSetCollectionsResponse,
            >,
            tonic::Status,
        >;
        async fn change_project_status(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::ChangeProjectStatusRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::ChangeProjectStatusResponse>,
            tonic::Status,
        >;
        /// 资产集
        async fn new_asset_collection(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::NewAssetCollectionRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::NewAssetCollectionResponse>,
            tonic::Status,
        >;
        async fn get_asset_collection_asset_total_count(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::GetAssetCollectionAssetTotalCountRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::knitter_module::protocols::GetAssetCollectionAssetTotalCountResponse,
            >,
            tonic::Status,
        >;
        async fn get_asset_collection_assembly_total_count(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::GetAssetCollectionAssemblyTotalCountRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::knitter_module::protocols::GetAssetCollectionAssemblyTotalCountResponse,
            >,
            tonic::Status,
        >;
        async fn get_asset_collection_assets_page(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::GetAssetCollectionAssetsPageRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::knitter_module::protocols::GetAssetCollectionAssetsPageResponse,
            >,
            tonic::Status,
        >;
        async fn get_asset_collection_assemblies_page(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::GetAssetCollectionAssembliesPageRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::knitter_module::protocols::GetAssetCollectionAssembliesPageResponse,
            >,
            tonic::Status,
        >;
        /// 资产
        async fn new_asset(
            &self,
            request: tonic::Request<::knitter_module::protocols::NewAssetRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::NewAssetResponse>,
            tonic::Status,
        >;
        async fn get_referenced_assets(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::GetReferencedAssetsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::GetReferencedAssetsResponse>,
            tonic::Status,
        >;
        async fn mark_asset_status(
            &self,
            request: tonic::Request<::knitter_module::protocols::MarkAssetStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::MarkAssetStatusResponse>,
            tonic::Status,
        >;
        /// 组合
        async fn new_assembly(
            &self,
            request: tonic::Request<::knitter_module::protocols::NewAssemblyRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::NewAssemblyResponse>,
            tonic::Status,
        >;
        /// 集
        async fn new_epic(
            &self,
            request: tonic::Request<::knitter_module::protocols::NewEpicRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::NewEpicResponse>,
            tonic::Status,
        >;
        async fn get_epic_sequences(
            &self,
            request: tonic::Request<::knitter_module::protocols::GetEpicSequencesRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::GetEpicSequencesResponse>,
            tonic::Status,
        >;
        /// 章节
        async fn new_sequence(
            &self,
            request: tonic::Request<::knitter_module::protocols::NewSequenceRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::NewSequenceResponse>,
            tonic::Status,
        >;
        async fn get_sequence_cuts(
            &self,
            request: tonic::Request<::knitter_module::protocols::GetSequenceCutsRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::GetSequenceCutsResponse>,
            tonic::Status,
        >;
        /// 镜头
        async fn new_cut(
            &self,
            request: tonic::Request<::knitter_module::protocols::NewCutRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::NewCutResponse>,
            tonic::Status,
        >;
        async fn get_cut_referenced_assets(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::GetCutReferencedAssetsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::GetCutReferencedAssetsResponse>,
            tonic::Status,
        >;
        async fn mark_cut_status(
            &self,
            request: tonic::Request<::knitter_module::protocols::MarkCutStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::MarkCutStatusResponse>,
            tonic::Status,
        >;
        /// 景集合
        async fn new_set_collection(
            &self,
            request: tonic::Request<::knitter_module::protocols::NewSetCollectionRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::NewSetCollectionResponse>,
            tonic::Status,
        >;
        async fn get_set_collection_sets_page(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::GetSetCollectionSetsPageRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::knitter_module::protocols::GetSetCollectionSetsPageResponse,
            >,
            tonic::Status,
        >;
        async fn get_set_collection_set_total_count(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::GetSetCollectionSetTotalCountRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::knitter_module::protocols::GetSetCollectionSetTotalCountResponse,
            >,
            tonic::Status,
        >;
        /// 景
        async fn new_set(
            &self,
            request: tonic::Request<::knitter_module::protocols::NewSetRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::NewSetResponse>,
            tonic::Status,
        >;
        async fn mark_set_satus(
            &self,
            request: tonic::Request<::knitter_module::protocols::MarkSetStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::MarkSetStatusResponse>,
            tonic::Status,
        >;
        /// 引用
        async fn add_references(
            &self,
            request: tonic::Request<::knitter_module::protocols::AddReferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::AddReferencesResponse>,
            tonic::Status,
        >;
        async fn remove_references(
            &self,
            request: tonic::Request<::knitter_module::protocols::RemoveReferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::RemoveReferencesResponse>,
            tonic::Status,
        >;
        async fn list_references(
            &self,
            request: tonic::Request<::knitter_module::protocols::ListReferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::ListReferencesResponse>,
            tonic::Status,
        >;
        async fn change_reference(
            &self,
            request: tonic::Request<
                ::knitter_module::protocols::ChangeReferencePrefabRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<::knitter_module::protocols::ChangeReferencePrefabResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct KnitterGrpcServer<T: KnitterGrpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
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
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
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
        ) -> Poll<std::result::Result<(), Self::Error>> {
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_manages(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetManagesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_manage_entry_count(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetManageEntryCountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_manage_schema(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetManageSchemaSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).mark_schema_field_removed(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkSchemaFieldRemovedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).change_manage_read_rule(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeManageReadRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).change_manage_write_rule(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeManageWriteRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).change_collection_read_rule(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeCollectionReadRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).change_collection_write_rule(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeCollectionWriteRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).change_field_read_rule(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeFieldReadRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).change_field_write_rule(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeFieldWriteRuleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_entity(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEntitySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_entities(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEntitiesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_entities_page(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEntitiesPageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/MarkEntityRemoved" => {
                    #[allow(non_camel_case_types)]
                    struct MarkEntityRemovedSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::MarkEntityRemovedRequest,
                    > for MarkEntityRemovedSvc<T> {
                        type Response = ::manage_define::cashmere::MarkEntityRemovedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::MarkEntityRemovedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).mark_entity_removed(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkEntityRemovedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/RecoverRemovedEntity" => {
                    #[allow(non_camel_case_types)]
                    struct RecoverRemovedEntitySvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::RecoverRemovedEntityRequest,
                    > for RecoverRemovedEntitySvc<T> {
                        type Response = ::manage_define::cashmere::RecoverRemovedEntityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::RecoverRemovedEntityRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).recover_removed_entity(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RecoverRemovedEntitySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetRemovedEntitiesPage" => {
                    #[allow(non_camel_case_types)]
                    struct GetRemovedEntitiesPageSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::GetRemovedEntitiesPageRequest,
                    > for GetRemovedEntitiesPageSvc<T> {
                        type Response = ::manage_define::cashmere::GetRemovedEntitiesPageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::GetRemovedEntitiesPageRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_removed_entities_page(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRemovedEntitiesPageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).edit_entity_field(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditEntityFieldSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).edit_entity_map_field(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditEntityMapFieldSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).edit_entity_map_field_remove_key(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditEntityMapFieldRemoveKeySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/EditEntityArrayFieldAddItems" => {
                    #[allow(non_camel_case_types)]
                    struct EditEntityArrayFieldAddItemsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::EditEntityArrayFieldAddItemsRequest,
                    > for EditEntityArrayFieldAddItemsSvc<T> {
                        type Response = ::manage_define::cashmere::EditEntityArrayFieldAddItemsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::EditEntityArrayFieldAddItemsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).edit_entity_array_field_add_items(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditEntityArrayFieldAddItemsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/EditEntityArrayFieldRemoveItems" => {
                    #[allow(non_camel_case_types)]
                    struct EditEntityArrayFieldRemoveItemsSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::EditEntityArrayFieldRemoveItemsRequest,
                    > for EditEntityArrayFieldRemoveItemsSvc<T> {
                        type Response = ::manage_define::cashmere::EditEntityArrayFieldRemoveItemsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::EditEntityArrayFieldRemoveItemsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).edit_entity_array_field_remove_items(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditEntityArrayFieldRemoveItemsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).rename(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RenameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_language_name(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewLanguageNameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewCountryCode" => {
                    #[allow(non_camel_case_types)]
                    struct NewCountryCodeSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::NewCountryCodeRequest,
                    > for NewCountryCodeSvc<T> {
                        type Response = ::manage_define::cashmere::NewCountryCodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::NewCountryCodeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_country_code(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewCountryCodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetCountryCodes" => {
                    #[allow(non_camel_case_types)]
                    struct GetCountryCodesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::GetCountryCodesRequest,
                    > for GetCountryCodesSvc<T> {
                        type Response = ::manage_define::cashmere::GetCountryCodesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::GetCountryCodesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_country_codes(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCountryCodesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_language_code(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewLanguageCodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetLanguageCodes" => {
                    #[allow(non_camel_case_types)]
                    struct GetLanguageCodesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::GetLanguageCodesRequest,
                    > for GetLanguageCodesSvc<T> {
                        type Response = ::manage_define::cashmere::GetLanguageCodesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::GetLanguageCodesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_language_codes(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLanguageCodesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewPhoneAreaCode" => {
                    #[allow(non_camel_case_types)]
                    struct NewPhoneAreaCodeSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::NewPhoneAreaCodeRequest,
                    > for NewPhoneAreaCodeSvc<T> {
                        type Response = ::manage_define::cashmere::NewPhoneAreaCodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::NewPhoneAreaCodeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_phone_area_code(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewPhoneAreaCodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetPhoneAreaCodes" => {
                    #[allow(non_camel_case_types)]
                    struct GetPhoneAreaCodesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::GetPhoneAreaCodesRequest,
                    > for GetPhoneAreaCodesSvc<T> {
                        type Response = ::manage_define::cashmere::GetPhoneAreaCodesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::GetPhoneAreaCodesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_phone_area_codes(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPhoneAreaCodesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).new_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetDataServerConfigs" => {
                    #[allow(non_camel_case_types)]
                    struct GetDataServerConfigsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::GetDataServerConfigsRequest,
                    > for GetDataServerConfigsSvc<T> {
                        type Response = ::data_module::protocols::GetDataServerConfigsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::GetDataServerConfigsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_data_server_configs(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDataServerConfigsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                        ::data_module::protocols::NewDataRequest,
                    > for NewDataSvc<T> {
                        type Response = ::data_module::protocols::NewDataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::NewDataRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).new_data(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ListEntityData" => {
                    #[allow(non_camel_case_types)]
                    struct ListEntityDataSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::ListEntityDataRequest,
                    > for ListEntityDataSvc<T> {
                        type Response = ::data_module::protocols::ListEntityDataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::ListEntityDataRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_entity_data(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListEntityDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetDataInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetDataInfoSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::GetDataInfoRequest,
                    > for GetDataInfoSvc<T> {
                        type Response = ::data_module::protocols::GetDataInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::GetDataInfoRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_data_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDataInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
                        ::data_module::protocols::MarkDataRemovedRequest,
                    > for MarkDataRemovedSvc<T> {
                        type Response = ::data_module::protocols::MarkDataRemovedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::MarkDataRemovedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).mark_data_removed(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkDataRemovedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewSpecs" => {
                    #[allow(non_camel_case_types)]
                    struct NewSpecsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::NewSpecsRequest,
                    > for NewSpecsSvc<T> {
                        type Response = ::data_module::protocols::NewSpecsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::NewSpecsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).new_specs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewSpecsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ListSpecs" => {
                    #[allow(non_camel_case_types)]
                    struct ListSpecsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::ListSpecsRequest,
                    > for ListSpecsSvc<T> {
                        type Response = ::data_module::protocols::ListSpecsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::ListSpecsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_specs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSpecsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ListSpecsPrefabs" => {
                    #[allow(non_camel_case_types)]
                    struct ListSpecsPrefabsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::ListSpecsPrefabsRequest,
                    > for ListSpecsPrefabsSvc<T> {
                        type Response = ::data_module::protocols::ListSpecsPrefabsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::ListSpecsPrefabsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_specs_prefabs(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSpecsPrefabsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewStage" => {
                    #[allow(non_camel_case_types)]
                    struct NewStageSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::NewStageRequest,
                    > for NewStageSvc<T> {
                        type Response = ::data_module::protocols::NewStageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::NewStageRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).new_stage(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewStageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ListStages" => {
                    #[allow(non_camel_case_types)]
                    struct ListStagesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::ListStagesRequest,
                    > for ListStagesSvc<T> {
                        type Response = ::data_module::protocols::ListStagesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::ListStagesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_stages(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListStagesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewPrefab" => {
                    #[allow(non_camel_case_types)]
                    struct NewPrefabSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::NewPrefabRequest,
                    > for NewPrefabSvc<T> {
                        type Response = ::data_module::protocols::NewPrefabResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::NewPrefabRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).new_prefab(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewPrefabSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/AddStageVersion" => {
                    #[allow(non_camel_case_types)]
                    struct AddStageVersionSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::AddStageVersionRequest,
                    > for AddStageVersionSvc<T> {
                        type Response = ::data_module::protocols::AddStageVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::AddStageVersionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_stage_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddStageVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ListStageVersions" => {
                    #[allow(non_camel_case_types)]
                    struct ListStageVersionsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::ListStageVersionsRequest,
                    > for ListStageVersionsSvc<T> {
                        type Response = ::data_module::protocols::ListStageVersionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::ListStageVersionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_stage_versions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListStageVersionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/SetStageCurrentVersion" => {
                    #[allow(non_camel_case_types)]
                    struct SetStageCurrentVersionSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::SetStageCurrentVersionRequest,
                    > for SetStageCurrentVersionSvc<T> {
                        type Response = ::data_module::protocols::SetStageCurrentVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::SetStageCurrentVersionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_stage_current_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetStageCurrentVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/RemoveStageVersion" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveStageVersionSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::RemoveStageVersionRequest,
                    > for RemoveStageVersionSvc<T> {
                        type Response = ::data_module::protocols::RemoveStageVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::RemoveStageVersionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_stage_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveStageVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/AddFileToVersion" => {
                    #[allow(non_camel_case_types)]
                    struct AddFileToVersionSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::AddFileToVersionRequest,
                    > for AddFileToVersionSvc<T> {
                        type Response = ::data_module::protocols::AddFileToVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::AddFileToVersionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_file_to_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddFileToVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/AddFileSetToVersion" => {
                    #[allow(non_camel_case_types)]
                    struct AddFileSetToVersionSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::AddFileSetToVersionRequest,
                    > for AddFileSetToVersionSvc<T> {
                        type Response = ::data_module::protocols::AddFileSetToVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::AddFileSetToVersionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_file_set_to_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddFileSetToVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/AddFileSequenceToVersion" => {
                    #[allow(non_camel_case_types)]
                    struct AddFileSequenceToVersionSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::AddFileSequenceToVersionRequest,
                    > for AddFileSequenceToVersionSvc<T> {
                        type Response = ::data_module::protocols::AddFileSequenceToVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::AddFileSequenceToVersionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_file_sequence_to_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddFileSequenceToVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/RemoveFilesFromVersion" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveFilesFromVersionSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::RemoveFilesFromVersionRequest,
                    > for RemoveFilesFromVersionSvc<T> {
                        type Response = ::data_module::protocols::RemoveFilesFromVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::RemoveFilesFromVersionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_files_from_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveFilesFromVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ListVersionFolder" => {
                    #[allow(non_camel_case_types)]
                    struct ListVersionFolderSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::ListVersionFolderRequest,
                    > for ListVersionFolderSvc<T> {
                        type Response = ::data_module::protocols::ListVersionFolderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::ListVersionFolderRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_version_folder(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListVersionFolderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/DeleteVersionFolderEntries" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteVersionFolderEntriesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::data_module::protocols::DeleteVersionFolderEntriesRequest,
                    > for DeleteVersionFolderEntriesSvc<T> {
                        type Response = ::data_module::protocols::DeleteVersionFolderEntriesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::data_module::protocols::DeleteVersionFolderEntriesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_version_folder_entries(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteVersionFolderEntriesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/UploadFile" => {
                    #[allow(non_camel_case_types)]
                    struct UploadFileSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::StreamingService<
                        ::data_module::protocols::UploadFileRequest,
                    > for UploadFileSvc<T> {
                        type Response = ::data_module::protocols::UploadFileResponse;
                        type ResponseStream = T::UploadFileStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    ::data_module::protocols::UploadFileRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).upload_file(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UploadFileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/DownloadFile" => {
                    #[allow(non_camel_case_types)]
                    struct DownloadFileSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::StreamingService<
                        ::data_module::protocols::DownloadFileRequest,
                    > for DownloadFileSvc<T> {
                        type Response = ::data_module::protocols::DownloadFileResponse;
                        type ResponseStream = T::DownloadFileStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    ::data_module::protocols::DownloadFileRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).download_file(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DownloadFileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/RegisterEventType" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterEventTypeSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::event_module::protocols::RegisterEventTypeRequest,
                    > for RegisterEventTypeSvc<T> {
                        type Response = ::event_module::protocols::RegisterEventTypeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::event_module::protocols::RegisterEventTypeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).register_event_type(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterEventTypeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/RegisterEventEmitter" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterEventEmitterSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::event_module::protocols::RegisterEventEmitterRequest,
                    > for RegisterEventEmitterSvc<T> {
                        type Response = ::event_module::protocols::RegisterEventEmitterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::event_module::protocols::RegisterEventEmitterRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).register_event_emitter(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterEventEmitterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/RegisterEventListener" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterEventListenerSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::event_module::protocols::RegisterEventListenerRequest,
                    > for RegisterEventListenerSvc<T> {
                        type Response = ::event_module::protocols::RegisterEventListenerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::event_module::protocols::RegisterEventListenerRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).register_event_listener(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterEventListenerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/EmitEvent" => {
                    #[allow(non_camel_case_types)]
                    struct EmitEventSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::ServerStreamingService<
                        ::event_module::protocols::EmitEventRequest,
                    > for EmitEventSvc<T> {
                        type Response = ::event_module::protocols::EmitEventResponse;
                        type ResponseStream = T::EmitEventStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::event_module::protocols::EmitEventRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).emit_event(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EmitEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ListenEventType" => {
                    #[allow(non_camel_case_types)]
                    struct ListenEventTypeSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::ServerStreamingService<
                        ::event_module::protocols::ListenEventTypeRequest,
                    > for ListenEventTypeSvc<T> {
                        type Response = ::event_module::protocols::ListenEventTypeResponse;
                        type ResponseStream = T::ListenEventTypeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::event_module::protocols::ListenEventTypeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).listen_event_type(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListenEventTypeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewProject" => {
                    #[allow(non_camel_case_types)]
                    struct NewProjectSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::NewProjectRequest,
                    > for NewProjectSvc<T> {
                        type Response = ::knitter_module::protocols::NewProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::NewProjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).new_project(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/AssociateAssetCollectionsToProject" => {
                    #[allow(non_camel_case_types)]
                    struct AssociateAssetCollectionsToProjectSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::AssociateAssetCollectionsToProjectRequest,
                    > for AssociateAssetCollectionsToProjectSvc<T> {
                        type Response = ::knitter_module::protocols::AssociateAssetCollectionsToProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::AssociateAssetCollectionsToProjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .associate_asset_collections_to_project(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssociateAssetCollectionsToProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/DeassociateAssetCollectionsFromProject" => {
                    #[allow(non_camel_case_types)]
                    struct DeassociateAssetCollectionsFromProjectSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::DeassociateAssetCollectionsFromProjectRequest,
                    > for DeassociateAssetCollectionsFromProjectSvc<T> {
                        type Response = ::knitter_module::protocols::DeassociateAssetCollectionsFromProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::DeassociateAssetCollectionsFromProjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .deassociate_asset_collections_from_project(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeassociateAssetCollectionsFromProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/AssociateSetCollectionsToProject" => {
                    #[allow(non_camel_case_types)]
                    struct AssociateSetCollectionsToProjectSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::AssociateSetCollectionsToProjectRequest,
                    > for AssociateSetCollectionsToProjectSvc<T> {
                        type Response = ::knitter_module::protocols::AssociateSetCollectionsToProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::AssociateSetCollectionsToProjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).associate_set_collections_to_project(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssociateSetCollectionsToProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/DeassociateSetCollectionsFromProject" => {
                    #[allow(non_camel_case_types)]
                    struct DeassociateSetCollectionsFromProjectSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::DeassociateSetCollectionsFromProjectRequest,
                    > for DeassociateSetCollectionsFromProjectSvc<T> {
                        type Response = ::knitter_module::protocols::DeassociateSetCollectionsFromProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::DeassociateSetCollectionsFromProjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .deassociate_set_collections_from_project(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeassociateSetCollectionsFromProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetProjectAssociatedAssetCollections" => {
                    #[allow(non_camel_case_types)]
                    struct GetProjectAssociatedAssetCollectionsSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::GetProjectAssociatedAssetCollectionsRequest,
                    > for GetProjectAssociatedAssetCollectionsSvc<T> {
                        type Response = ::knitter_module::protocols::GetProjectAssociatedAssetCollectionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::GetProjectAssociatedAssetCollectionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .get_project_associated_asset_collections(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProjectAssociatedAssetCollectionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetProjectAssociatedSetCollections" => {
                    #[allow(non_camel_case_types)]
                    struct GetProjectAssociatedSetCollectionsSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::GetProjectAssociatedSetCollectionsRequest,
                    > for GetProjectAssociatedSetCollectionsSvc<T> {
                        type Response = ::knitter_module::protocols::GetProjectAssociatedSetCollectionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::GetProjectAssociatedSetCollectionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .get_project_associated_set_collections(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProjectAssociatedSetCollectionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ChangeProjectStatus" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeProjectStatusSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::ChangeProjectStatusRequest,
                    > for ChangeProjectStatusSvc<T> {
                        type Response = ::knitter_module::protocols::ChangeProjectStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::ChangeProjectStatusRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).change_project_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeProjectStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewAssetCollection" => {
                    #[allow(non_camel_case_types)]
                    struct NewAssetCollectionSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::NewAssetCollectionRequest,
                    > for NewAssetCollectionSvc<T> {
                        type Response = ::knitter_module::protocols::NewAssetCollectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::NewAssetCollectionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_asset_collection(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewAssetCollectionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetAssetCollectionAssetTotalCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetCollectionAssetTotalCountSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::GetAssetCollectionAssetTotalCountRequest,
                    > for GetAssetCollectionAssetTotalCountSvc<T> {
                        type Response = ::knitter_module::protocols::GetAssetCollectionAssetTotalCountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::GetAssetCollectionAssetTotalCountRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .get_asset_collection_asset_total_count(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetCollectionAssetTotalCountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetAssetCollectionAssemblyTotalCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetCollectionAssemblyTotalCountSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::GetAssetCollectionAssemblyTotalCountRequest,
                    > for GetAssetCollectionAssemblyTotalCountSvc<T> {
                        type Response = ::knitter_module::protocols::GetAssetCollectionAssemblyTotalCountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::GetAssetCollectionAssemblyTotalCountRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .get_asset_collection_assembly_total_count(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetCollectionAssemblyTotalCountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetAssetCollectionAssetsPage" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetCollectionAssetsPageSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::GetAssetCollectionAssetsPageRequest,
                    > for GetAssetCollectionAssetsPageSvc<T> {
                        type Response = ::knitter_module::protocols::GetAssetCollectionAssetsPageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::GetAssetCollectionAssetsPageRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_asset_collection_assets_page(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetCollectionAssetsPageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetAssetCollectionAssembliesPage" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetCollectionAssembliesPageSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::GetAssetCollectionAssembliesPageRequest,
                    > for GetAssetCollectionAssembliesPageSvc<T> {
                        type Response = ::knitter_module::protocols::GetAssetCollectionAssembliesPageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::GetAssetCollectionAssembliesPageRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_asset_collection_assemblies_page(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetCollectionAssembliesPageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewAsset" => {
                    #[allow(non_camel_case_types)]
                    struct NewAssetSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::NewAssetRequest,
                    > for NewAssetSvc<T> {
                        type Response = ::knitter_module::protocols::NewAssetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::NewAssetRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).new_asset(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewAssetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetReferencedAssets" => {
                    #[allow(non_camel_case_types)]
                    struct GetReferencedAssetsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::GetReferencedAssetsRequest,
                    > for GetReferencedAssetsSvc<T> {
                        type Response = ::knitter_module::protocols::GetReferencedAssetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::GetReferencedAssetsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_referenced_assets(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetReferencedAssetsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/MarkAssetStatus" => {
                    #[allow(non_camel_case_types)]
                    struct MarkAssetStatusSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::MarkAssetStatusRequest,
                    > for MarkAssetStatusSvc<T> {
                        type Response = ::knitter_module::protocols::MarkAssetStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::MarkAssetStatusRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).mark_asset_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkAssetStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewAssembly" => {
                    #[allow(non_camel_case_types)]
                    struct NewAssemblySvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::NewAssemblyRequest,
                    > for NewAssemblySvc<T> {
                        type Response = ::knitter_module::protocols::NewAssemblyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::NewAssemblyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_assembly(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewAssemblySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewEpic" => {
                    #[allow(non_camel_case_types)]
                    struct NewEpicSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::NewEpicRequest,
                    > for NewEpicSvc<T> {
                        type Response = ::knitter_module::protocols::NewEpicResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::NewEpicRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).new_epic(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewEpicSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetEpicSequences" => {
                    #[allow(non_camel_case_types)]
                    struct GetEpicSequencesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::GetEpicSequencesRequest,
                    > for GetEpicSequencesSvc<T> {
                        type Response = ::knitter_module::protocols::GetEpicSequencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::GetEpicSequencesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_epic_sequences(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEpicSequencesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewSequence" => {
                    #[allow(non_camel_case_types)]
                    struct NewSequenceSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::NewSequenceRequest,
                    > for NewSequenceSvc<T> {
                        type Response = ::knitter_module::protocols::NewSequenceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::NewSequenceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_sequence(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewSequenceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetSequenceCuts" => {
                    #[allow(non_camel_case_types)]
                    struct GetSequenceCutsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::GetSequenceCutsRequest,
                    > for GetSequenceCutsSvc<T> {
                        type Response = ::knitter_module::protocols::GetSequenceCutsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::GetSequenceCutsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_sequence_cuts(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSequenceCutsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewCut" => {
                    #[allow(non_camel_case_types)]
                    struct NewCutSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::NewCutRequest,
                    > for NewCutSvc<T> {
                        type Response = ::knitter_module::protocols::NewCutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::NewCutRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).new_cut(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewCutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetCutReferencedAssets" => {
                    #[allow(non_camel_case_types)]
                    struct GetCutReferencedAssetsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::GetCutReferencedAssetsRequest,
                    > for GetCutReferencedAssetsSvc<T> {
                        type Response = ::knitter_module::protocols::GetCutReferencedAssetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::GetCutReferencedAssetsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_cut_referenced_assets(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCutReferencedAssetsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/MarkCutStatus" => {
                    #[allow(non_camel_case_types)]
                    struct MarkCutStatusSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::MarkCutStatusRequest,
                    > for MarkCutStatusSvc<T> {
                        type Response = ::knitter_module::protocols::MarkCutStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::MarkCutStatusRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).mark_cut_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkCutStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewSetCollection" => {
                    #[allow(non_camel_case_types)]
                    struct NewSetCollectionSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::NewSetCollectionRequest,
                    > for NewSetCollectionSvc<T> {
                        type Response = ::knitter_module::protocols::NewSetCollectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::NewSetCollectionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_set_collection(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewSetCollectionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetSetCollectionSetsPage" => {
                    #[allow(non_camel_case_types)]
                    struct GetSetCollectionSetsPageSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::GetSetCollectionSetsPageRequest,
                    > for GetSetCollectionSetsPageSvc<T> {
                        type Response = ::knitter_module::protocols::GetSetCollectionSetsPageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::GetSetCollectionSetsPageRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_set_collection_sets_page(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSetCollectionSetsPageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/GetSetCollectionSetTotalCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetSetCollectionSetTotalCountSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::GetSetCollectionSetTotalCountRequest,
                    > for GetSetCollectionSetTotalCountSvc<T> {
                        type Response = ::knitter_module::protocols::GetSetCollectionSetTotalCountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::GetSetCollectionSetTotalCountRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_set_collection_set_total_count(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSetCollectionSetTotalCountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/NewSet" => {
                    #[allow(non_camel_case_types)]
                    struct NewSetSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::NewSetRequest,
                    > for NewSetSvc<T> {
                        type Response = ::knitter_module::protocols::NewSetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::NewSetRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).new_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewSetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/MarkSetSatus" => {
                    #[allow(non_camel_case_types)]
                    struct MarkSetSatusSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::MarkSetStatusRequest,
                    > for MarkSetSatusSvc<T> {
                        type Response = ::knitter_module::protocols::MarkSetStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::MarkSetStatusRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).mark_set_satus(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkSetSatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/AddReferences" => {
                    #[allow(non_camel_case_types)]
                    struct AddReferencesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::AddReferencesRequest,
                    > for AddReferencesSvc<T> {
                        type Response = ::knitter_module::protocols::AddReferencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::AddReferencesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_references(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddReferencesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/RemoveReferences" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveReferencesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::RemoveReferencesRequest,
                    > for RemoveReferencesSvc<T> {
                        type Response = ::knitter_module::protocols::RemoveReferencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::RemoveReferencesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_references(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveReferencesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ListReferences" => {
                    #[allow(non_camel_case_types)]
                    struct ListReferencesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::ListReferencesRequest,
                    > for ListReferencesSvc<T> {
                        type Response = ::knitter_module::protocols::ListReferencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::ListReferencesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_references(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListReferencesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.knitter.KnitterGrpc/ChangeReference" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeReferenceSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::knitter_module::protocols::ChangeReferencePrefabRequest,
                    > for ChangeReferenceSvc<T> {
                        type Response = ::knitter_module::protocols::ChangeReferencePrefabResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::knitter_module::protocols::ChangeReferencePrefabRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).change_reference(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeReferenceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
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
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: KnitterGrpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: KnitterGrpc> tonic::server::NamedService for KnitterGrpcServer<T> {
        const NAME: &'static str = "io.knitter.KnitterGrpc";
    }
}
