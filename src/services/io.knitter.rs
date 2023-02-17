/// 新项目
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewProjectRequest {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="2")]
    pub inner_root_path: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub external_root_path: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub picture: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewProjectResponse {
    /// 成功返回项目id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 标记项目已经完成
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkProjectStatusRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkProjectStatusResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 关联资产集合到项目
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateAssetCollectionsToProjectRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub collection_ids: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateAssetCollectionsToProjectResponse {
    /// "ok" if succeed
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 关联布景集合到项目
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateSetCollectionsToProjectRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub collection_ids: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateSetCollectionsToProjectResponse {
    /// "ok" if succeed
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得关联库表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectAssociatedAssetCollectionsRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectAssociatedAssetCollectionsResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub asset_collections: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 取得项目景表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectAssociatedSetCollectionsRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub set_collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectAssociatedSetCollectionsResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub set_collections: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 取得项目集表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectEpicsRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectEpicsResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub epics: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 新资产集合
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAssetCollectionRequest {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="2")]
    pub inner_root_path: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub external_root_path: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub picture: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAssetCollectionResponse {
    /// 成功返回项目id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得资产数量
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssetTotalPagesCountRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssetTotalPagesCountResponse {
    #[prost(uint32, tag="1")]
    pub total_count: u32,
}
/// 取得组合件数量
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssemblyTotalPagesCountRequest {
    #[prost(string, tag="1")]
    pub colllection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssemblyTotalPagesCountResponse {
    #[prost(uint32, tag="1")]
    pub totao_count: u32,
}
/// 取得资产页
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssetsPageRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub page_index: u32,
    #[prost(uint32, tag="3")]
    pub total_pages_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssetsPageResponse {
    /// bson list
    #[prost(bytes="vec", repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 取得组合页
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssembliesPageRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub page_index: u32,
    #[prost(uint32, tag="3")]
    pub total_pages_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetCollectionAssembliesPageResponse {
    /// bson list
    #[prost(bytes="vec", repeated, tag="1")]
    pub assemblies: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 标记资产集状态
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkAssetCollectionStatusRequest {
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(enumeration="AssetCollectionStatus", tag="2")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkAssetCollectionStatusResponse {
    /// 修改成功返回当前集状态
    #[prost(enumeration="AssetCollectionStatus", tag="1")]
    pub status: i32,
}
/// 资产集合状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetCollectionStatus {
    AssetCollectionClosed = 0,
    AssetCollectionOpenning = 1,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reference {
    #[prost(enumeration="ReferenceType", tag="1")]
    pub reference_type: i32,
    /// 源
    #[prost(string, tag="2")]
    pub source_id: ::prost::alloc::string::String,
    /// 规格
    #[prost(string, tag="4")]
    pub specs_id: ::prost::alloc::string::String,
    /// 预制件
    #[prost(string, tag="3")]
    pub prefab_id: ::prost::alloc::string::String,
}
/// 引用
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddReferencesRequest {
    /// 主管理编号
    #[prost(int32, tag="1")]
    pub subject_manage_id: i32,
    /// 主实体
    #[prost(string, tag="2")]
    pub subject_entity_id: ::prost::alloc::string::String,
    /// 主引用字段编码
    #[prost(string, tag="3")]
    pub reference_field_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub references: ::prost::alloc::vec::Vec<Reference>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddReferencesResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 列出引用
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReferencesRequest {
    /// 主管理编号
    #[prost(int32, tag="1")]
    pub subject_manage_id: i32,
    /// 主实体
    #[prost(string, tag="2")]
    pub subject_entity_id: ::prost::alloc::string::String,
    /// 主引用字段编码
    #[prost(string, tag="3")]
    pub reference_field_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReferencesResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub references: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 移除引用
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveReferencesRequest {
    /// 主管理编号
    #[prost(int32, tag="1")]
    pub subject_manage_id: i32,
    /// 主实体
    #[prost(string, tag="2")]
    pub subject_entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub reference_field_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub references: ::prost::alloc::vec::Vec<Reference>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveReferencesResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 修改引用预制件
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeReferencePrefabRequest {
    /// 主管理编号
    #[prost(int32, tag="1")]
    pub subject_manage_id: i32,
    /// 主实体
    #[prost(string, tag="2")]
    pub subject_entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub reference_field_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub old_reference: ::core::option::Option<Reference>,
    #[prost(message, optional, tag="5")]
    pub new_reference: ::core::option::Option<Reference>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeReferencePrefabResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReferenceType {
    /// 资产
    RefAsset = 0,
    /// 组合
    RefAssembly = 1,
    /// 景
    RefSet = 2,
}
/// 新建资产
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAssetRequest {
    #[prost(string, tag="1")]
    pub asset_collection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAssetResponse {
    /// 成功返回新资产id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 标记状态
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkAssetStatusRequest {
    #[prost(string, tag="1")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(enumeration="AssetStatus", tag="2")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkAssetStatusResponse {
    /// 成功返回  "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得引用
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReferencedAssetsRequest {
    /// 主管理编号
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    /// 主实体
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub asset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReferencedAssetsResponse {
    /// 成功返回 "ok"
    #[prost(bytes="vec", repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 资产状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetStatus {
    AssetDone = 0,
    AssetSuspended = 1,
    AssetCanceled = 2,
}
/// 新建组装
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAssemblyRequest {
    #[prost(string, tag="1")]
    pub asset_collection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAssemblyResponse {
    /// 成功返回新资产组合id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 更新资产到组
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAssemblyRequest {
    #[prost(string, tag="1")]
    pub assembly_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub asset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAssemblyResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 引用资产
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferenceAssembliesRequest {
    /// 主管理编号
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    /// 主实体
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub reference_field_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub assembly_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferenceAssembliesResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 新建集
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEpicRequest {
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEpicResponse {
    /// 成功返回新id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得集的章节
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEpicSequencesRequest {
    #[prost(string, tag="1")]
    pub epic_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEpicSequencesResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub sequences: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 新建章节
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSequenceRequest {
    #[prost(string, tag="1")]
    pub epic_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSequenceResponse {
    /// 成功返回新id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得章节的镜头表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSequenceCutsRequest {
    #[prost(string, tag="1")]
    pub sequence_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSequenceCutsResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub cuts: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 新建镜头
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCutRequest {
    #[prost(string, tag="1")]
    pub sequence_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCutResponse {
    /// 成功返回新id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 引用资产
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CutReferenceAssetsRequest {
    #[prost(string, tag="1")]
    pub cut_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub asset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CutReferenceAssetsResponse {
    /// 成功返回  "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 引用景
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CutRefereceSetsRequest {
    #[prost(string, tag="1")]
    pub cut_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CutRefereceSetsResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 标记状态
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkCutStatusRequest {
    #[prost(string, tag="1")]
    pub cut_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkCutStatusResponse {
    /// 成功返回  "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得引用的资产
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCutReferencedAssetsRequest {
    #[prost(string, tag="1")]
    pub cut_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCutReferencedAssetsResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 新项目
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSetCollectionRequest {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="2")]
    pub inner_root_path: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub external_root_path: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub picture: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSetCollectionResponse {
    /// 成功返回id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得资产页
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSetCollectionAssetsPageRequest {
    #[prost(string, tag="1")]
    pub library_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub page_index: u32,
    #[prost(uint32, tag="3")]
    pub total_page_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSetCollectionAssetsPageResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 取得组合页
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSetCollectionAssembliesPageRequest {
    #[prost(string, tag="1")]
    pub library_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub page_index: u32,
    #[prost(uint32, tag="3")]
    pub total_page_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSetCollectionAssembliesPageResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 新建景
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSetRequest {
    #[prost(string, tag="1")]
    pub set_collection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSetResponse {
    /// 成功返回新id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 添加引用资产
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferencAssetsRequest {
    #[prost(string, tag="1")]
    pub set_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub asset_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferencAssetsResponse {
    /// 成功返回"ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 更新引用资产
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReferencedAssetsRequest {
    #[prost(string, tag="1")]
    pub set_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub asset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReferencedAssetsResponse {
    /// 成功返回"ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 引用，原则上只被章节引用
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferenceSetsRequest {
    /// 主管理编号
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    /// 主实体
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub set_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferenceSetsResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 标记状态
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkSetStatusRequest {
    #[prost(string, tag="1")]
    pub set_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkSetStatusResponse {
    /// 成功返回  "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSpecsRequest {
    #[prost(int32, tag="1")]
    pub owner_manage_id: i32,
    #[prost(string, tag="2")]
    pub owner_entity_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSpecsResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpecsRequest {
    #[prost(int32, tag="1")]
    pub owner_manage_id: i32,
    #[prost(string, tag="2")]
    pub owner_entity_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpecsResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub specses: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpecsPrefabsRequest {
    #[prost(string, tag="1")]
    pub specs_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpecsPrefabsResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub prefabs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPrefabRequest {
    #[prost(string, tag="1")]
    pub specs_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPrefabResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
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
        async fn mark_entity_removed(
            &self,
            request: tonic::Request<::manage_define::cashmere::MarkEntityRemovedRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::MarkEntityRemovedResponse>,
            tonic::Status,
        >;
        /// 通用编辑实体属性，非数据结构
        async fn edit_entity_field(
            &self,
            request: tonic::Request<::manage_define::cashmere::EditEntityFieldRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::EditEntityFieldResponse>,
            tonic::Status,
        >;
        /// 通用编辑实体属性，MAP数据结构
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
        /// 通用编辑实体属性，List数据结构
        async fn edit_entity_array_field_add_items(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::EditEntityArrayFieldAddItemsRequest,
            >,
        ) -> Result<
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
        ) -> Result<
            tonic::Response<
                ::manage_define::cashmere::EditEntityArrayFieldRemoveItemsResponse,
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
        /// 组
        async fn new_group(
            &self,
            request: tonic::Request<::manage_define::cashmere::NewGroupRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::NewGroupResponse>,
            tonic::Status,
        >;
        /// 数据
        async fn get_data_server_configs(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::GetDataServerConfigsRequest,
            >,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::GetDataServerConfigsResponse>,
            tonic::Status,
        >;
        async fn new_data(
            &self,
            request: tonic::Request<::manage_define::cashmere::NewDataRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::NewDataResponse>,
            tonic::Status,
        >;
        async fn list_data(
            &self,
            request: tonic::Request<::manage_define::cashmere::ListDataRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::ListDataResponse>,
            tonic::Status,
        >;
        async fn list_data_stages(
            &self,
            request: tonic::Request<::manage_define::cashmere::ListDataStagesRequest>,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::ListDataStagesResponse>,
            tonic::Status,
        >;
        async fn add_data_stage_version(
            &self,
            request: tonic::Request<
                ::manage_define::cashmere::AddDataStageVersionRequest,
            >,
        ) -> Result<
            tonic::Response<::manage_define::cashmere::AddDataStageVersionResponse>,
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
        /// 数据操作
        async fn file_data_upload_file(
            &self,
            request: tonic::Request<
                tonic::Streaming<::manage_define::cashmere::FileDataUploadFileRequest>,
            >,
        ) -> Result<tonic::Response<Self::FileDataUploadFileStream>, tonic::Status>;
        ///Server streaming response type for the FileDataDownloadFile method.
        type FileDataDownloadFileStream: futures_core::Stream<
                Item = Result<
                    ::manage_define::cashmere::FileDataDownloadFileResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn file_data_download_file(
            &self,
            request: tonic::Request<
                tonic::Streaming<::manage_define::cashmere::FileDataDownloadFileRequest>,
            >,
        ) -> Result<tonic::Response<Self::FileDataDownloadFileStream>, tonic::Status>;
        /// 项目
        async fn new_project(
            &self,
            request: tonic::Request<super::NewProjectRequest>,
        ) -> Result<tonic::Response<super::NewProjectResponse>, tonic::Status>;
        async fn associate_asset_collections_to_project(
            &self,
            request: tonic::Request<super::AssociateAssetCollectionsToProjectRequest>,
        ) -> Result<
            tonic::Response<super::AssociateAssetCollectionsToProjectResponse>,
            tonic::Status,
        >;
        async fn associate_set_collections_to_project(
            &self,
            request: tonic::Request<super::AssociateSetCollectionsToProjectRequest>,
        ) -> Result<
            tonic::Response<super::AssociateSetCollectionsToProjectResponse>,
            tonic::Status,
        >;
        async fn get_project_associated_asset_collections(
            &self,
            request: tonic::Request<super::GetProjectAssociatedAssetCollectionsRequest>,
        ) -> Result<
            tonic::Response<super::GetProjectAssociatedAssetCollectionsResponse>,
            tonic::Status,
        >;
        async fn get_project_associated_set_collections(
            &self,
            request: tonic::Request<super::GetProjectAssociatedSetCollectionsRequest>,
        ) -> Result<
            tonic::Response<super::GetProjectAssociatedSetCollectionsResponse>,
            tonic::Status,
        >;
        /// 资产集
        async fn new_asset_collection(
            &self,
            request: tonic::Request<super::NewAssetCollectionRequest>,
        ) -> Result<tonic::Response<super::NewAssetCollectionResponse>, tonic::Status>;
        async fn get_asset_collection_asset_total_pages_count(
            &self,
            request: tonic::Request<super::GetAssetCollectionAssetTotalPagesCountRequest>,
        ) -> Result<
            tonic::Response<super::GetAssetCollectionAssetTotalPagesCountResponse>,
            tonic::Status,
        >;
        async fn get_asset_collection_assembly_total_pages_count(
            &self,
            request: tonic::Request<
                super::GetAssetCollectionAssemblyTotalPagesCountRequest,
            >,
        ) -> Result<
            tonic::Response<super::GetAssetCollectionAssemblyTotalPagesCountResponse>,
            tonic::Status,
        >;
        async fn get_asset_collection_assets_page(
            &self,
            request: tonic::Request<super::GetAssetCollectionAssetsPageRequest>,
        ) -> Result<
            tonic::Response<super::GetAssetCollectionAssetsPageResponse>,
            tonic::Status,
        >;
        async fn get_asset_collection_assemblies_page(
            &self,
            request: tonic::Request<super::GetAssetCollectionAssembliesPageRequest>,
        ) -> Result<
            tonic::Response<super::GetAssetCollectionAssembliesPageResponse>,
            tonic::Status,
        >;
        async fn mark_asset_collection_status(
            &self,
            request: tonic::Request<super::MarkAssetCollectionStatusRequest>,
        ) -> Result<
            tonic::Response<super::MarkAssetCollectionStatusResponse>,
            tonic::Status,
        >;
        /// 资产
        async fn new_asset(
            &self,
            request: tonic::Request<super::NewAssetRequest>,
        ) -> Result<tonic::Response<super::NewAssetResponse>, tonic::Status>;
        async fn get_referenced_assets(
            &self,
            request: tonic::Request<super::GetReferencedAssetsRequest>,
        ) -> Result<tonic::Response<super::GetReferencedAssetsResponse>, tonic::Status>;
        async fn mark_asset_status(
            &self,
            request: tonic::Request<super::MarkAssetStatusRequest>,
        ) -> Result<tonic::Response<super::MarkAssetStatusResponse>, tonic::Status>;
        /// 组合
        async fn new_assembly(
            &self,
            request: tonic::Request<super::NewAssemblyRequest>,
        ) -> Result<tonic::Response<super::NewAssemblyResponse>, tonic::Status>;
        /// 集
        async fn new_epic(
            &self,
            request: tonic::Request<super::NewEpicRequest>,
        ) -> Result<tonic::Response<super::NewEpicResponse>, tonic::Status>;
        async fn get_epic_sequences(
            &self,
            request: tonic::Request<super::GetEpicSequencesRequest>,
        ) -> Result<tonic::Response<super::GetEpicSequencesResponse>, tonic::Status>;
        /// 章节
        async fn new_sequence(
            &self,
            request: tonic::Request<super::NewSequenceRequest>,
        ) -> Result<tonic::Response<super::NewSequenceResponse>, tonic::Status>;
        async fn get_sequence_cuts(
            &self,
            request: tonic::Request<super::GetSequenceCutsRequest>,
        ) -> Result<tonic::Response<super::GetSequenceCutsResponse>, tonic::Status>;
        /// 镜头
        async fn new_cut(
            &self,
            request: tonic::Request<super::NewCutRequest>,
        ) -> Result<tonic::Response<super::NewCutResponse>, tonic::Status>;
        async fn get_cut_referenced_assets(
            &self,
            request: tonic::Request<super::GetCutReferencedAssetsRequest>,
        ) -> Result<
            tonic::Response<super::GetCutReferencedAssetsResponse>,
            tonic::Status,
        >;
        async fn mark_cut_status(
            &self,
            request: tonic::Request<super::MarkCutStatusRequest>,
        ) -> Result<tonic::Response<super::MarkCutStatusResponse>, tonic::Status>;
        /// 景集合
        async fn new_set_collection(
            &self,
            request: tonic::Request<super::NewSetCollectionRequest>,
        ) -> Result<tonic::Response<super::NewSetCollectionResponse>, tonic::Status>;
        /// 景
        async fn new_set(
            &self,
            request: tonic::Request<super::NewSetRequest>,
        ) -> Result<tonic::Response<super::NewSetResponse>, tonic::Status>;
        async fn mark_set_satus(
            &self,
            request: tonic::Request<super::MarkSetStatusRequest>,
        ) -> Result<tonic::Response<super::MarkSetStatusResponse>, tonic::Status>;
        /// 规格
        async fn new_specs(
            &self,
            request: tonic::Request<super::NewSpecsRequest>,
        ) -> Result<tonic::Response<super::NewSpecsResponse>, tonic::Status>;
        async fn list_specs(
            &self,
            request: tonic::Request<super::ListSpecsRequest>,
        ) -> Result<tonic::Response<super::ListSpecsResponse>, tonic::Status>;
        async fn list_specs_prefabs(
            &self,
            request: tonic::Request<super::ListSpecsPrefabsRequest>,
        ) -> Result<tonic::Response<super::ListSpecsPrefabsResponse>, tonic::Status>;
        /// 预制件
        async fn new_prefab(
            &self,
            request: tonic::Request<super::NewPrefabRequest>,
        ) -> Result<tonic::Response<super::NewPrefabResponse>, tonic::Status>;
        /// 引用
        async fn add_references(
            &self,
            request: tonic::Request<super::AddReferencesRequest>,
        ) -> Result<tonic::Response<super::AddReferencesResponse>, tonic::Status>;
        async fn remove_references(
            &self,
            request: tonic::Request<super::RemoveReferencesRequest>,
        ) -> Result<tonic::Response<super::RemoveReferencesResponse>, tonic::Status>;
        async fn list_references(
            &self,
            request: tonic::Request<super::ListReferencesRequest>,
        ) -> Result<tonic::Response<super::ListReferencesResponse>, tonic::Status>;
        async fn change_reference(
            &self,
            request: tonic::Request<super::ChangeReferencePrefabRequest>,
        ) -> Result<
            tonic::Response<super::ChangeReferencePrefabResponse>,
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).mark_entity_removed(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkEntityRemovedSvc(inner);
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).edit_entity_array_field_add_items(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditEntityArrayFieldAddItemsSvc(inner);
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).edit_entity_array_field_remove_items(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditEntityArrayFieldRemoveItemsSvc(inner);
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
                "/io.knitter.KnitterGrpc/GetDataServerConfigs" => {
                    #[allow(non_camel_case_types)]
                    struct GetDataServerConfigsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::GetDataServerConfigsRequest,
                    > for GetDataServerConfigsSvc<T> {
                        type Response = ::manage_define::cashmere::GetDataServerConfigsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::GetDataServerConfigsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_data_server_configs(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDataServerConfigsSvc(inner);
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
                "/io.knitter.KnitterGrpc/ListData" => {
                    #[allow(non_camel_case_types)]
                    struct ListDataSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::ListDataRequest,
                    > for ListDataSvc<T> {
                        type Response = ::manage_define::cashmere::ListDataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::ListDataRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_data(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDataSvc(inner);
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
                "/io.knitter.KnitterGrpc/ListDataStages" => {
                    #[allow(non_camel_case_types)]
                    struct ListDataStagesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::ListDataStagesRequest,
                    > for ListDataStagesSvc<T> {
                        type Response = ::manage_define::cashmere::ListDataStagesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::ListDataStagesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_data_stages(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDataStagesSvc(inner);
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
                "/io.knitter.KnitterGrpc/AddDataStageVersion" => {
                    #[allow(non_camel_case_types)]
                    struct AddDataStageVersionSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        ::manage_define::cashmere::AddDataStageVersionRequest,
                    > for AddDataStageVersionSvc<T> {
                        type Response = ::manage_define::cashmere::AddDataStageVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::manage_define::cashmere::AddDataStageVersionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_data_stage_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddDataStageVersionSvc(inner);
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
                "/io.knitter.KnitterGrpc/FileDataDownloadFile" => {
                    #[allow(non_camel_case_types)]
                    struct FileDataDownloadFileSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::StreamingService<
                        ::manage_define::cashmere::FileDataDownloadFileRequest,
                    > for FileDataDownloadFileSvc<T> {
                        type Response = ::manage_define::cashmere::FileDataDownloadFileResponse;
                        type ResponseStream = T::FileDataDownloadFileStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    ::manage_define::cashmere::FileDataDownloadFileRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).file_data_download_file(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FileDataDownloadFileSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewProject" => {
                    #[allow(non_camel_case_types)]
                    struct NewProjectSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::NewProjectRequest>
                    for NewProjectSvc<T> {
                        type Response = super::NewProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_project(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewProjectSvc(inner);
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
                "/io.knitter.KnitterGrpc/AssociateAssetCollectionsToProject" => {
                    #[allow(non_camel_case_types)]
                    struct AssociateAssetCollectionsToProjectSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        super::AssociateAssetCollectionsToProjectRequest,
                    > for AssociateAssetCollectionsToProjectSvc<T> {
                        type Response = super::AssociateAssetCollectionsToProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AssociateAssetCollectionsToProjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
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
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssociateAssetCollectionsToProjectSvc(inner);
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
                "/io.knitter.KnitterGrpc/AssociateSetCollectionsToProject" => {
                    #[allow(non_camel_case_types)]
                    struct AssociateSetCollectionsToProjectSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        super::AssociateSetCollectionsToProjectRequest,
                    > for AssociateSetCollectionsToProjectSvc<T> {
                        type Response = super::AssociateSetCollectionsToProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AssociateSetCollectionsToProjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).associate_set_collections_to_project(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssociateSetCollectionsToProjectSvc(inner);
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
                "/io.knitter.KnitterGrpc/GetProjectAssociatedAssetCollections" => {
                    #[allow(non_camel_case_types)]
                    struct GetProjectAssociatedAssetCollectionsSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        super::GetProjectAssociatedAssetCollectionsRequest,
                    > for GetProjectAssociatedAssetCollectionsSvc<T> {
                        type Response = super::GetProjectAssociatedAssetCollectionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetProjectAssociatedAssetCollectionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
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
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProjectAssociatedAssetCollectionsSvc(inner);
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
                "/io.knitter.KnitterGrpc/GetProjectAssociatedSetCollections" => {
                    #[allow(non_camel_case_types)]
                    struct GetProjectAssociatedSetCollectionsSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        super::GetProjectAssociatedSetCollectionsRequest,
                    > for GetProjectAssociatedSetCollectionsSvc<T> {
                        type Response = super::GetProjectAssociatedSetCollectionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetProjectAssociatedSetCollectionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
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
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProjectAssociatedSetCollectionsSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewAssetCollection" => {
                    #[allow(non_camel_case_types)]
                    struct NewAssetCollectionSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::NewAssetCollectionRequest>
                    for NewAssetCollectionSvc<T> {
                        type Response = super::NewAssetCollectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewAssetCollectionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).new_asset_collection(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewAssetCollectionSvc(inner);
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
                "/io.knitter.KnitterGrpc/GetAssetCollectionAssetTotalPagesCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetCollectionAssetTotalPagesCountSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        super::GetAssetCollectionAssetTotalPagesCountRequest,
                    > for GetAssetCollectionAssetTotalPagesCountSvc<T> {
                        type Response = super::GetAssetCollectionAssetTotalPagesCountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetAssetCollectionAssetTotalPagesCountRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .get_asset_collection_asset_total_pages_count(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetCollectionAssetTotalPagesCountSvc(inner);
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
                "/io.knitter.KnitterGrpc/GetAssetCollectionAssemblyTotalPagesCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetCollectionAssemblyTotalPagesCountSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        super::GetAssetCollectionAssemblyTotalPagesCountRequest,
                    > for GetAssetCollectionAssemblyTotalPagesCountSvc<T> {
                        type Response = super::GetAssetCollectionAssemblyTotalPagesCountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetAssetCollectionAssemblyTotalPagesCountRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .get_asset_collection_assembly_total_pages_count(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetCollectionAssemblyTotalPagesCountSvc(inner);
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
                "/io.knitter.KnitterGrpc/GetAssetCollectionAssetsPage" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetCollectionAssetsPageSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        super::GetAssetCollectionAssetsPageRequest,
                    > for GetAssetCollectionAssetsPageSvc<T> {
                        type Response = super::GetAssetCollectionAssetsPageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetAssetCollectionAssetsPageRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_asset_collection_assets_page(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetCollectionAssetsPageSvc(inner);
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
                "/io.knitter.KnitterGrpc/GetAssetCollectionAssembliesPage" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetCollectionAssembliesPageSvc<T: KnitterGrpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        super::GetAssetCollectionAssembliesPageRequest,
                    > for GetAssetCollectionAssembliesPageSvc<T> {
                        type Response = super::GetAssetCollectionAssembliesPageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetAssetCollectionAssembliesPageRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_asset_collection_assemblies_page(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetCollectionAssembliesPageSvc(inner);
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
                "/io.knitter.KnitterGrpc/MarkAssetCollectionStatus" => {
                    #[allow(non_camel_case_types)]
                    struct MarkAssetCollectionStatusSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<
                        super::MarkAssetCollectionStatusRequest,
                    > for MarkAssetCollectionStatusSvc<T> {
                        type Response = super::MarkAssetCollectionStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MarkAssetCollectionStatusRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).mark_asset_collection_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkAssetCollectionStatusSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewAsset" => {
                    #[allow(non_camel_case_types)]
                    struct NewAssetSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::NewAssetRequest>
                    for NewAssetSvc<T> {
                        type Response = super::NewAssetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewAssetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_asset(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewAssetSvc(inner);
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
                "/io.knitter.KnitterGrpc/GetReferencedAssets" => {
                    #[allow(non_camel_case_types)]
                    struct GetReferencedAssetsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::GetReferencedAssetsRequest>
                    for GetReferencedAssetsSvc<T> {
                        type Response = super::GetReferencedAssetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetReferencedAssetsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_referenced_assets(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetReferencedAssetsSvc(inner);
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
                "/io.knitter.KnitterGrpc/MarkAssetStatus" => {
                    #[allow(non_camel_case_types)]
                    struct MarkAssetStatusSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::MarkAssetStatusRequest>
                    for MarkAssetStatusSvc<T> {
                        type Response = super::MarkAssetStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarkAssetStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).mark_asset_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkAssetStatusSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewAssembly" => {
                    #[allow(non_camel_case_types)]
                    struct NewAssemblySvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::NewAssemblyRequest>
                    for NewAssemblySvc<T> {
                        type Response = super::NewAssemblyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewAssemblyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).new_assembly(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewAssemblySvc(inner);
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
                "/io.knitter.KnitterGrpc/NewEpic" => {
                    #[allow(non_camel_case_types)]
                    struct NewEpicSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::NewEpicRequest>
                    for NewEpicSvc<T> {
                        type Response = super::NewEpicResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewEpicRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_epic(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewEpicSvc(inner);
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
                "/io.knitter.KnitterGrpc/GetEpicSequences" => {
                    #[allow(non_camel_case_types)]
                    struct GetEpicSequencesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::GetEpicSequencesRequest>
                    for GetEpicSequencesSvc<T> {
                        type Response = super::GetEpicSequencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEpicSequencesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_epic_sequences(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEpicSequencesSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewSequence" => {
                    #[allow(non_camel_case_types)]
                    struct NewSequenceSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::NewSequenceRequest>
                    for NewSequenceSvc<T> {
                        type Response = super::NewSequenceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewSequenceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).new_sequence(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewSequenceSvc(inner);
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
                "/io.knitter.KnitterGrpc/GetSequenceCuts" => {
                    #[allow(non_camel_case_types)]
                    struct GetSequenceCutsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::GetSequenceCutsRequest>
                    for GetSequenceCutsSvc<T> {
                        type Response = super::GetSequenceCutsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSequenceCutsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_sequence_cuts(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSequenceCutsSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewCut" => {
                    #[allow(non_camel_case_types)]
                    struct NewCutSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::NewCutRequest>
                    for NewCutSvc<T> {
                        type Response = super::NewCutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewCutRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_cut(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewCutSvc(inner);
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
                "/io.knitter.KnitterGrpc/GetCutReferencedAssets" => {
                    #[allow(non_camel_case_types)]
                    struct GetCutReferencedAssetsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::GetCutReferencedAssetsRequest>
                    for GetCutReferencedAssetsSvc<T> {
                        type Response = super::GetCutReferencedAssetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCutReferencedAssetsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_cut_referenced_assets(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCutReferencedAssetsSvc(inner);
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
                "/io.knitter.KnitterGrpc/MarkCutStatus" => {
                    #[allow(non_camel_case_types)]
                    struct MarkCutStatusSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::MarkCutStatusRequest>
                    for MarkCutStatusSvc<T> {
                        type Response = super::MarkCutStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarkCutStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).mark_cut_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkCutStatusSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewSetCollection" => {
                    #[allow(non_camel_case_types)]
                    struct NewSetCollectionSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::NewSetCollectionRequest>
                    for NewSetCollectionSvc<T> {
                        type Response = super::NewSetCollectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewSetCollectionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).new_set_collection(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewSetCollectionSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewSet" => {
                    #[allow(non_camel_case_types)]
                    struct NewSetSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::NewSetRequest>
                    for NewSetSvc<T> {
                        type Response = super::NewSetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewSetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewSetSvc(inner);
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
                "/io.knitter.KnitterGrpc/MarkSetSatus" => {
                    #[allow(non_camel_case_types)]
                    struct MarkSetSatusSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::MarkSetStatusRequest>
                    for MarkSetSatusSvc<T> {
                        type Response = super::MarkSetStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarkSetStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).mark_set_satus(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkSetSatusSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewSpecs" => {
                    #[allow(non_camel_case_types)]
                    struct NewSpecsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::NewSpecsRequest>
                    for NewSpecsSvc<T> {
                        type Response = super::NewSpecsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewSpecsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_specs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewSpecsSvc(inner);
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
                "/io.knitter.KnitterGrpc/ListSpecs" => {
                    #[allow(non_camel_case_types)]
                    struct ListSpecsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::ListSpecsRequest>
                    for ListSpecsSvc<T> {
                        type Response = super::ListSpecsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSpecsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_specs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSpecsSvc(inner);
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
                "/io.knitter.KnitterGrpc/ListSpecsPrefabs" => {
                    #[allow(non_camel_case_types)]
                    struct ListSpecsPrefabsSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::ListSpecsPrefabsRequest>
                    for ListSpecsPrefabsSvc<T> {
                        type Response = super::ListSpecsPrefabsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSpecsPrefabsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_specs_prefabs(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSpecsPrefabsSvc(inner);
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
                "/io.knitter.KnitterGrpc/NewPrefab" => {
                    #[allow(non_camel_case_types)]
                    struct NewPrefabSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::NewPrefabRequest>
                    for NewPrefabSvc<T> {
                        type Response = super::NewPrefabResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewPrefabRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_prefab(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewPrefabSvc(inner);
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
                "/io.knitter.KnitterGrpc/AddReferences" => {
                    #[allow(non_camel_case_types)]
                    struct AddReferencesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::AddReferencesRequest>
                    for AddReferencesSvc<T> {
                        type Response = super::AddReferencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddReferencesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_references(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddReferencesSvc(inner);
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
                "/io.knitter.KnitterGrpc/RemoveReferences" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveReferencesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::RemoveReferencesRequest>
                    for RemoveReferencesSvc<T> {
                        type Response = super::RemoveReferencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveReferencesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_references(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveReferencesSvc(inner);
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
                "/io.knitter.KnitterGrpc/ListReferences" => {
                    #[allow(non_camel_case_types)]
                    struct ListReferencesSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::ListReferencesRequest>
                    for ListReferencesSvc<T> {
                        type Response = super::ListReferencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListReferencesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_references(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListReferencesSvc(inner);
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
                "/io.knitter.KnitterGrpc/ChangeReference" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeReferenceSvc<T: KnitterGrpc>(pub Arc<T>);
                    impl<
                        T: KnitterGrpc,
                    > tonic::server::UnaryService<super::ChangeReferencePrefabRequest>
                    for ChangeReferenceSvc<T> {
                        type Response = super::ChangeReferencePrefabResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeReferencePrefabRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).change_reference(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeReferenceSvc(inner);
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
