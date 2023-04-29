/*
Author: 闫刚 (yes7rose@sina.com)
mod.rs (c) 2021
Desc: 服务模块
Created:  2021-01-17T04:31:08.729Z
Modified: !date!
*/

use manage_define::cashmere::*;
use service_common_handles::view_rules_service_handles::*;
use service_common_handles::ResponseStream;
use service_common_handles::{
    area_service_handles::HandleEditArea,
    area_service_handles::HandleNewArea,
    country_service_handles::HandleNewCountry,
    data_service_handles::*,
    entity_service_handles::*,
    group_service_handles::HandleNewGroup,
    language_code_handles::{HandleEditLanguageCode, HandleNewLanguageCode},
    manage_service_handle::*,
    name_service_handles::{HandleNewLanguageName, HandleRename},
    prefab_service_handles::*,
    specses_service_handles::*,
    stage_service_handles::*,
};
use std::sync::Arc;
use tonic::{Request, Response, Status, Streaming};

use crate::services::protocol::knitter_grpc_server::KnitterGrpc;
use crate::services::protocol::*;

pub mod assembly_servcice_handles;
pub mod asset_collection_servcice_handles;
pub mod assets_servcice_handles;
pub mod cut_servcice_handles;
pub mod epic_servcice_handles;
pub mod project_servcice_handles;
pub mod reference_service_handles;
pub mod sequence_servcice_handles;
pub mod set_collection_service_handles;
pub mod set_servcice_handles;

mod init;

pub mod protocol {
    include!("./io.knitter.rs");
}

/// 管理服务
#[derive(Default)]
pub struct KnitterServer;

// 组
impl HandleNewGroup for KnitterServer {}

// 国家
impl HandleNewCountry for KnitterServer {}

// 地区
impl HandleNewArea for KnitterServer {}

impl HandleEditArea for KnitterServer {}

// 语言
impl HandleNewLanguageCode for KnitterServer {}

impl HandleEditLanguageCode for KnitterServer {}

// 管理
impl HandleGetManages for KnitterServer {}

impl HandleGetManageSchema for KnitterServer {}

impl HandleMarkSchemaFieldRemoved for KnitterServer {}

impl HandleGetManageEntryCount for KnitterServer {}

// 可见性权限管理
impl HandleChangeManageReadrule for KnitterServer {}

impl HandleChangeManageWriteRule for KnitterServer {}

impl HandleChangeCollectionReadrule for KnitterServer {}

impl HandleChangeCollectionWriteRule for KnitterServer {}

impl HandleChangeFieldReadrule for KnitterServer {}

impl HandleChangeFieldWriteRule for KnitterServer {}

// 实体
impl HandleGetEntity for KnitterServer {}
impl HandleGetEntities for KnitterServer {}
impl HandleGetEntitiesPage for KnitterServer {}
impl HandleEditEntityField for KnitterServer {}
impl HandleEditEntityArrayFieldAddItems for KnitterServer {}
impl HandleEditEntityArrayFieldRemoveItems for KnitterServer {}
impl HandleEditEntityMapField for KnitterServer {}
impl HandleEditEntityMapFieldRemoveKey for KnitterServer {}

impl HandleMarkEntityRemoved for KnitterServer {}
impl HandleRecoverRemovedEntity for KnitterServer {}
impl HandleGetRemovedEntitiesPage for KnitterServer {}

// 名字
impl HandleRename for KnitterServer {}
impl HandleNewLanguageName for KnitterServer {}

// 数据
impl HandleGetDataServerConfigs for KnitterServer {}
impl HandleNewData for KnitterServer {}
impl HandleListEntityData for KnitterServer {}
impl HandleUploadFile for KnitterServer {}
impl HandleDownloadFile for KnitterServer {}

impl HandleNewSpecs for KnitterServer {}
impl HandleListSpecs for KnitterServer {}
impl HandleListSpecsPrefabs for KnitterServer {}

impl HandleNewStage for KnitterServer {}
impl HandleListStages for KnitterServer {}
impl HandleAddStageVersion for KnitterServer {}
impl HandleListStageVersions for KnitterServer {}
impl HandleSetStageCurrentVersion for KnitterServer {}
impl HandleRemoveStageVersion for KnitterServer {}
impl HandleAddFileToVersion for KnitterServer {}
impl HandleAddFileSetToVersion for KnitterServer {}
impl HandleAddFileSequenceToVersion for KnitterServer {}
impl HandleRemoveFilesFromVersion for KnitterServer {}

impl HandleListVersionFolder for KnitterServer {}
impl HandleDeleteVersionFolderEntries for KnitterServer {}

impl HandleGetDataInfo for KnitterServer {}

impl HandleNewPrefab for KnitterServer {}

#[tonic::async_trait]
impl KnitterGrpc for KnitterServer {
    async fn get_manages(
        &self,
        request: Request<GetManagesRequest>,
    ) -> Result<Response<GetManagesResponse>, Status> {
        self.handle_get_manages(request).await
    }

    async fn get_manage_entry_count(
        &self,
        request: Request<GetManageEntryCountRequest>,
    ) -> Result<Response<GetManageEntryCountResponse>, Status> {
        self.handle_get_manage_entry_count(request).await
    }

    async fn get_manage_schema(
        &self,
        request: Request<GetManageSchemaRequest>,
    ) -> Result<Response<GetManageSchemaResponse>, Status> {
        self.handle_get_manage_schema(request).await
    }

    async fn mark_schema_field_removed(
        &self,
        request: Request<MarkSchemaFieldRemovedRequest>,
    ) -> Result<Response<MarkSchemaFieldRemovedResponse>, Status> {
        self.handle_mark_schema_field_removed(request).await
    }

    async fn change_manage_read_rule(
        &self,
        request: Request<ChangeManageReadRuleRequest>,
    ) -> Result<Response<ChangeManageReadRuleResponse>, Status> {
        self.handle_change_manage_read_rule(request).await
    }

    async fn change_manage_write_rule(
        &self,
        request: Request<ChangeManageWriteRuleRequest>,
    ) -> Result<Response<ChangeManageWriteRuleResponse>, Status> {
        self.handle_change_manage_write_rule(request).await
    }

    async fn change_collection_read_rule(
        &self,
        request: Request<ChangeCollectionReadRuleRequest>,
    ) -> Result<Response<ChangeCollectionReadRuleResponse>, Status> {
        self.handle_change_collection_read_rule(request).await
    }

    async fn change_collection_write_rule(
        &self,
        request: Request<ChangeCollectionWriteRuleRequest>,
    ) -> Result<Response<ChangeCollectionWriteRuleResponse>, Status> {
        self.handle_change_collection_write_rule(request).await
    }

    async fn change_field_read_rule(
        &self,
        request: Request<ChangeFieldReadRuleRequest>,
    ) -> Result<Response<ChangeFieldReadRuleResponse>, Status> {
        self.handle_change_field_read_rule(request).await
    }

    async fn change_field_write_rule(
        &self,
        request: Request<ChangeFieldWriteRuleRequest>,
    ) -> Result<Response<ChangeFieldWriteRuleResponse>, Status> {
        self.handle_change_field_write_rule(request).await
    }

    async fn get_entity(
        &self,
        request: Request<GetEntityRequest>,
    ) -> Result<Response<GetEntityResponse>, Status> {
        self.handle_get_entity(request).await
    }

    async fn get_entities(
        &self,
        request: Request<GetEntitiesRequest>,
    ) -> Result<Response<GetEntitiesResponse>, Status> {
        self.handle_get_entities(request).await
    }

    async fn get_entities_page(
        &self,
        request: Request<GetEntitiesPageRequest>,
    ) -> Result<Response<GetEntitiesPageResponse>, Status> {
        self.handle_get_entities_page(request).await
    }

    async fn mark_entity_removed(
        &self,
        request: Request<MarkEntityRemovedRequest>,
    ) -> Result<Response<MarkEntityRemovedResponse>, Status> {
        self.handle_mark_entity_remved(request).await
    }

    async fn recover_removed_entity(
        &self,
        request: Request<RecoverRemovedEntityRequest>,
    ) -> Result<Response<RecoverRemovedEntityResponse>, Status> {
        self.handle_recover_removed_entity(request).await
    }

    async fn get_removed_entities_page(
        &self,
        request: Request<GetRemovedEntitiesPageRequest>,
    ) -> Result<Response<GetRemovedEntitiesPageResponse>, Status> {
        self.handle_get_removed_entities_page(request).await
    }

    async fn edit_entity_field(
        &self,
        request: Request<EditEntityFieldRequest>,
    ) -> Result<Response<EditEntityFieldResponse>, Status> {
        self.handle_edit_entity_field(request).await
    }

    async fn edit_entity_map_field(
        &self,
        request: Request<::manage_define::cashmere::EditEntityMapFieldRequest>,
    ) -> Result<Response<::manage_define::cashmere::EditEntityMapFieldResponse>, Status> {
        self.handle_edit_entity_map_field(request).await
    }

    async fn edit_entity_map_field_remove_key(
        &self,
        request: Request<EditEntityMapFieldRemoveKeyRequest>,
    ) -> Result<Response<EditEntityMapFieldRemoveKeyResponse>, Status> {
        self.handle_edit_entity_map_field_remove_key(request).await
    }

    async fn edit_entity_array_field_add_items(
        &self,
        request: Request<EditEntityArrayFieldAddItemsRequest>,
    ) -> Result<Response<EditEntityArrayFieldAddItemsResponse>, Status> {
        self.handle_edit_entity_array_field_add_items(request).await
    }

    async fn edit_entity_array_field_remove_items(
        &self,
        request: Request<EditEntityArrayFieldRemoveItemsRequest>,
    ) -> Result<Response<EditEntityArrayFieldRemoveItemsResponse>, Status> {
        self.handle_edit_entity_array_field_remove_items(request)
            .await
    }

    async fn rename(
        &self,
        request: Request<RenameRequest>,
    ) -> Result<Response<RenameResponse>, Status> {
        self.handle_rename(request).await
    }

    async fn new_language_name(
        &self,
        request: Request<NewLanguageNameRequest>,
    ) -> Result<Response<NewLanguageNameResponse>, Status> {
        self.handle_new_language_name(request).await
    }

    async fn new_country(
        &self,
        request: Request<NewCountryRequest>,
    ) -> Result<Response<NewCountryResponse>, Status> {
        self.handle_new_country(request).await
    }

    // 语言编码
    async fn new_language_code(
        &self,
        request: Request<NewLanguageCodeRequest>,
    ) -> Result<Response<NewLanguageCodeResponse>, Status> {
        self.handle_new_language_code(request).await
    }

    async fn new_group(
        &self,
        request: Request<NewGroupRequest>,
    ) -> Result<Response<NewGroupResponse>, Status> {
        self.handle_new_group(request).await
    }

    async fn get_data_server_configs(
        &self,
        request: Request<GetDataServerConfigsRequest>,
    ) -> Result<Response<GetDataServerConfigsResponse>, Status> {
        self.handle_get_data_server_configs(request).await
    }

    async fn new_data(
        &self,
        request: Request<NewDataRequest>,
    ) -> Result<Response<NewDataResponse>, Status> {
        self.handle_new_data(request).await
    }
    async fn list_entity_data(
        &self,
        request: Request<ListEntityDataRequest>,
    ) -> Result<Response<ListEntityDataResponse>, Status> {
        self.handle_list_entity_data(request).await
    }

    async fn get_data_info(
        &self,
        request: Request<GetDataInfoRequest>,
    ) -> Result<Response<GetDataInfoResponse>, Status> {
        self.handle_get_data_info(request).await
    }

    async fn mark_data_removed(
        &self,
        _request: Request<MarkDataRemovedRequest>,
    ) -> Result<Response<MarkDataRemovedResponse>, Status> {
        todo!()
    }

    async fn new_specs(
        &self,
        request: Request<NewSpecsRequest>,
    ) -> Result<Response<NewSpecsResponse>, Status> {
        self.handle_new_specs(request).await
    }

    async fn list_specs(
        &self,
        request: Request<ListSpecsRequest>,
    ) -> Result<Response<ListSpecsResponse>, Status> {
        self.handle_list_specs(request).await
    }

    async fn list_specs_prefabs(
        &self,
        request: Request<ListSpecsPrefabsRequest>,
    ) -> Result<Response<ListSpecsPrefabsResponse>, Status> {
        self.handle_list_specs_prefabs(request).await
    }
    async fn new_stage(
        &self,
        request: Request<NewStageRequest>,
    ) -> Result<Response<NewStageResponse>, Status> {
        self.handle_new_stage(request).await
    }

    async fn list_stages(
        &self,
        request: Request<ListStagesRequest>,
    ) -> Result<Response<ListStagesResponse>, Status> {
        self.handle_list_stages(request).await
    }

    async fn new_prefab(
        &self,
        request: Request<NewPrefabRequest>,
    ) -> Result<Response<NewPrefabResponse>, Status> {
        self.handle_new_prefab(request).await
    }

    async fn add_stage_version(
        &self,
        request: Request<AddStageVersionRequest>,
    ) -> Result<Response<AddStageVersionResponse>, Status> {
        self.handle_add_stage_version(request).await
    }

    async fn list_stage_versions(
        &self,
        request: Request<ListStageVersionsRequest>,
    ) -> Result<Response<ListStageVersionsResponse>, Status> {
        self.handle_list_stage_versions(request).await
    }

    async fn set_stage_current_version(
        &self,
        request: Request<SetStageCurrentVersionRequest>,
    ) -> Result<Response<SetStageCurrentVersionResponse>, Status> {
        self.handle_set_stage_current_version(request).await
    }

    async fn remove_stage_version(
        &self,
        request: Request<RemoveStageVersionRequest>,
    ) -> Result<Response<RemoveStageVersionResponse>, Status> {
        self.handle_remove_stage_version(request).await
    }

    async fn add_file_to_version(
        &self,
        request: Request<AddFileToVersionRequest>,
    ) -> Result<Response<AddFileToVersionResponse>, Status> {
        self.handle_add_file_to_version(request).await
    }

    async fn add_file_set_to_version(
        &self,
        request: Request<AddFileSetToVersionRequest>,
    ) -> Result<Response<AddFileSetToVersionResponse>, Status> {
        self.handle_add_file_set_to_version(request).await
    }

    async fn add_file_sequence_to_version(
        &self,
        request: Request<AddFileSequenceToVersionRequest>,
    ) -> Result<Response<AddFileSequenceToVersionResponse>, Status> {
        self.handle_add_file_sequence_to_version(request).await
    }

    async fn remove_files_from_version(
        &self,
        request: Request<RemoveFilesFromVersionRequest>,
    ) -> Result<Response<RemoveFilesFromVersionResponse>, Status> {
        self.handle_remove_files_from_version(request).await
    }

    async fn list_version_folder(
        &self,
        request: Request<ListVersionFolderRequest>,
    ) -> Result<Response<ListVersionFolderResponse>, Status> {
        self.handle_list_version_folder(request).await
    }

    async fn delete_version_folder_entries(
        &self,
        request: Request<DeleteVersionFolderEntriesRequest>,
    ) -> Result<Response<DeleteVersionFolderEntriesResponse>, Status> {
        self.handle_delete_version_folder_entries(request).await
    }

    // 必须要有这个声明
    type UploadFileStream = ResponseStream<UploadFileResponse>;

    // async fn mark_asset_collection_status(
    //     &self,
    //     request: Request<MarkAssetCollectionStatusRequest>,
    // ) -> Result<Response<MarkAssetCollectionStatusResponse>, Status> {
    //     todo!()
    // }

    async fn upload_file(
        &self,
        request: Request<Streaming<UploadFileRequest>>,
    ) -> Result<Response<Self::UploadFileStream>, Status> {
        self.handle_upload_file(request).await
    }

    // 必须要有这个声明
    type DownloadFileStream = ResponseStream<DownloadFileResponse>;

    async fn download_file(
        &self,
        request: Request<Streaming<DownloadFileRequest>>,
    ) -> Result<Response<Self::DownloadFileStream>, Status> {
        self.handle_download_file(request).await
    }

    async fn new_project(
        &self,
        request: Request<NewProjectRequest>,
    ) -> Result<Response<NewProjectResponse>, Status> {
        self.handle_new_project(request).await
    }

    async fn associate_asset_collections_to_project(
        &self,
        request: Request<AssociateAssetCollectionsToProjectRequest>,
    ) -> Result<Response<AssociateAssetCollectionsToProjectResponse>, Status> {
        self.handle_associate_asset_collections_to_project(request)
            .await
    }

    async fn deassociate_asset_collections_from_project(
        &self,
        request: Request<DeassociateAssetCollectionsFromProjectRequest>,
    ) -> Result<Response<DeassociateAssetCollectionsFromProjectResponse>, Status> {
        self.handle_deassociate_asset_collections_from_project(request)
            .await
    }

    async fn associate_set_collections_to_project(
        &self,
        request: Request<AssociateSetCollectionsToProjectRequest>,
    ) -> Result<Response<AssociateSetCollectionsToProjectResponse>, Status> {
        self.handle_associate_set_collections_to_project(request)
            .await
    }

    async fn deassociate_set_collections_from_project(
        &self,
        request: Request<DeassociateSetCollectionsFromProjectRequest>,
    ) -> Result<Response<DeassociateSetCollectionsFromProjectResponse>, Status> {
        self.handle_deassociate_set_collections_from_project(request)
            .await
    }

    async fn get_project_associated_asset_collections(
        &self,
        request: Request<GetProjectAssociatedAssetCollectionsRequest>,
    ) -> Result<Response<GetProjectAssociatedAssetCollectionsResponse>, Status> {
        self.handle_get_project_associated_asset_collections(request)
            .await
    }

    async fn get_project_associated_set_collections(
        &self,
        request: Request<GetProjectAssociatedSetCollectionsRequest>,
    ) -> Result<Response<GetProjectAssociatedSetCollectionsResponse>, Status> {
        self.handle_get_project_associated_set_collections(request)
            .await
    }

    async fn change_project_status(
        &self,
        request: Request<ChangeProjectStatusRequest>,
    ) -> Result<Response<ChangeProjectStatusResponse>, Status> {
        self.handle_change_project_status(request).await
    }

    async fn new_asset_collection(
        &self,
        request: Request<NewAssetCollectionRequest>,
    ) -> Result<Response<NewAssetCollectionResponse>, Status> {
        self.handle_new_asset_collection(request).await
    }

    async fn get_asset_collection_asset_total_count(
        &self,
        request: Request<GetAssetCollectionAssetTotalCountRequest>,
    ) -> Result<Response<GetAssetCollectionAssetTotalCountResponse>, Status> {
        self.handle_get_asset_collection_asset_total_count(request)
            .await
    }

    async fn get_asset_collection_assembly_total_count(
        &self,
        request: Request<GetAssetCollectionAssemblyTotalCountRequest>,
    ) -> Result<Response<GetAssetCollectionAssemblyTotalCountResponse>, Status> {
        self.handle_get_asset_collection_assembly_total_count(request)
            .await
    }

    async fn get_asset_collection_assets_page(
        &self,
        request: Request<GetAssetCollectionAssetsPageRequest>,
    ) -> Result<Response<GetAssetCollectionAssetsPageResponse>, Status> {
        self.handle_get_asset_collection_assets_page(request).await
    }

    async fn get_asset_collection_assemblies_page(
        &self,
        request: Request<GetAssetCollectionAssembliesPageRequest>,
    ) -> Result<Response<GetAssetCollectionAssembliesPageResponse>, Status> {
        self.handle_get_asset_collection_assemblies_page(request)
            .await
    }

    async fn new_asset(
        &self,
        request: Request<NewAssetRequest>,
    ) -> Result<Response<NewAssetResponse>, Status> {
        self.handle_new_asset(request).await
    }

    async fn get_referenced_assets(
        &self,
        request: Request<GetReferencedAssetsRequest>,
    ) -> Result<Response<GetReferencedAssetsResponse>, Status> {
        self.get_referenced_assets(request).await
    }

    async fn mark_asset_status(
        &self,
        request: Request<MarkAssetStatusRequest>,
    ) -> Result<Response<MarkAssetStatusResponse>, Status> {
        todo!()
    }

    async fn new_assembly(
        &self,
        request: Request<NewAssemblyRequest>,
    ) -> Result<Response<NewAssemblyResponse>, Status> {
        self.handle_new_assembly(request).await
    }

    async fn new_epic(
        &self,
        request: Request<NewEpicRequest>,
    ) -> Result<Response<NewEpicResponse>, Status> {
        self.handle_new_epic(request).await
    }

    async fn get_epic_sequences(
        &self,
        request: Request<GetEpicSequencesRequest>,
    ) -> Result<Response<GetEpicSequencesResponse>, Status> {
        self.handle_get_epic_sequences(request).await
    }

    async fn new_sequence(
        &self,
        request: Request<NewSequenceRequest>,
    ) -> Result<Response<NewSequenceResponse>, Status> {
        self.handle_new_sequence(request).await
    }

    async fn get_sequence_cuts(
        &self,
        request: Request<GetSequenceCutsRequest>,
    ) -> Result<Response<GetSequenceCutsResponse>, Status> {
        todo!()
    }

    async fn new_cut(
        &self,
        request: Request<NewCutRequest>,
    ) -> Result<Response<NewCutResponse>, Status> {
        self.handle_new_cut(request).await
    }

    async fn get_cut_referenced_assets(
        &self,
        request: Request<GetCutReferencedAssetsRequest>,
    ) -> Result<Response<GetCutReferencedAssetsResponse>, Status> {
        todo!()
    }

    async fn mark_cut_status(
        &self,
        request: Request<MarkCutStatusRequest>,
    ) -> Result<Response<MarkCutStatusResponse>, Status> {
        todo!()
    }

    async fn new_set_collection(
        &self,
        request: Request<NewSetCollectionRequest>,
    ) -> Result<Response<NewSetCollectionResponse>, Status> {
        self.handle_new_set_collection(request).await
    }

    async fn get_set_collection_sets_page(
        &self,
        request: Request<GetSetCollectionSetsPageRequest>,
    ) -> Result<Response<GetSetCollectionSetsPageResponse>, Status> {
        self.handle_get_set_collection_sets_page(request).await
    }

    async fn get_set_collection_set_total_count(
        &self,
        request: Request<GetSetCollectionSetTotalCountRequest>,
    ) -> Result<Response<GetSetCollectionSetTotalCountResponse>, Status> {
        self.handle_get_set_collection_set_total_count(request)
            .await
    }

    async fn new_set(
        &self,
        request: Request<NewSetRequest>,
    ) -> Result<Response<NewSetResponse>, Status> {
        self.handle_new_set(request).await
    }

    async fn mark_set_satus(
        &self,
        request: Request<MarkSetStatusRequest>,
    ) -> Result<Response<MarkSetStatusResponse>, Status> {
        todo!()
    }

    async fn add_references(
        &self,
        request: Request<AddReferencesRequest>,
    ) -> Result<Response<AddReferencesResponse>, Status> {
        self.handle_add_references(request).await
    }

    async fn remove_references(
        &self,
        request: Request<RemoveReferencesRequest>,
    ) -> Result<Response<RemoveReferencesResponse>, Status> {
        self.handle_remove_references(request).await
    }

    async fn list_references(
        &self,
        request: Request<ListReferencesRequest>,
    ) -> Result<Response<ListReferencesResponse>, Status> {
        self.handle_list_references(request).await
    }

    async fn change_reference(
        &self,
        request: Request<ChangeReferencePrefabRequest>,
    ) -> Result<Response<ChangeReferencePrefabResponse>, Status> {
        todo!()
    }
}
