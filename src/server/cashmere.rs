/// 名
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Name {
    #[prost(string, tag = "1")]
    pub language: std::string::String,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
/// 名属性，语言：语言名
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NameField {
    #[prost(map = "string, string", tag = "1")]
    pub name_field: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// 重命名
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameRequest {
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
    #[prost(string, tag = "2")]
    pub entity_id: std::string::String,
    #[prost(string, tag = "3")]
    pub language: std::string::String,
    #[prost(string, tag = "4")]
    pub new_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 新语言名
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewLanguageNameRequest {
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
    #[prost(string, tag = "2")]
    pub entity_id: std::string::String,
    #[prost(string, tag = "3")]
    pub language: std::string::String,
    #[prost(string, tag = "4")]
    pub new_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewLanguageNameResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 新建单位组织
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOrganizationRequest {
    #[prost(string, tag = "1")]
    pub location: std::string::String,
    #[prost(string, tag = "2")]
    pub language: std::string::String,
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOrganizationResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 添加管理员
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAdministratorToOrganizationRequest {
    #[prost(string, tag = "1")]
    pub organization_id: std::string::String,
    #[prost(string, tag = "2")]
    pub admin_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAdministratorToOrganizationResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 移除管理员
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAdministratorFromOrganizationRequest {
    #[prost(string, tag = "1")]
    pub organization_id: std::string::String,
    #[prost(string, tag = "2")]
    pub admin_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAdministratorFromOrganizationResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 添加雇员
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEmployeeToOrganizationRequest {
    #[prost(string, tag = "1")]
    pub organization_id: std::string::String,
    #[prost(string, tag = "2")]
    pub employee_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEmployeeToOrganizationResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 移除雇员
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEmployeeFromOrganizationRequest {
    #[prost(string, tag = "1")]
    pub organization_id: std::string::String,
    #[prost(string, tag = "2")]
    pub employee_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEmployeeFromOrganizationResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 新建单位部门
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDepartmentRequest {
    #[prost(string, tag = "1")]
    pub organization_id: std::string::String,
    #[prost(string, tag = "2")]
    pub language: std::string::String,
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDepartmentResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 添加部门管理元员
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAdministratorToDepartmentRequest {
    #[prost(string, tag = "1")]
    pub department_id: std::string::String,
    #[prost(string, tag = "2")]
    pub admin_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAdministratorToDepartmentResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 移除管理员
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAdministratorFromDepartmentRequest {
    #[prost(string, tag = "1")]
    pub department_id: std::string::String,
    #[prost(string, tag = "2")]
    pub admin_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAdministratorFromDepartmentResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 添加部门员工
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEmployeeToDepartmentRequest {
    #[prost(string, tag = "1")]
    pub department_id: std::string::String,
    #[prost(string, tag = "2")]
    pub employee_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEmployeeToDepartmentResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 移除部门员工
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEmployeeFromDepartmentRequest {
    #[prost(string, tag = "1")]
    pub department_id: std::string::String,
    #[prost(string, tag = "2")]
    pub employee_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEmployeeFromDepartmentResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 新班级
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewClassRequest {
    #[prost(string, tag = "1")]
    pub organization_id: std::string::String,
    #[prost(string, tag = "2")]
    pub language: std::string::String,
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewClassResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 添加班主任
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetClassTeacherOfClassRequest {
    #[prost(string, tag = "1")]
    pub class_id: std::string::String,
    #[prost(string, tag = "2")]
    pub class_teacher_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetClassTeacherOfClassResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 添加老师
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTeacherToClassRequest {
    #[prost(string, tag = "1")]
    pub class_id: std::string::String,
    #[prost(string, tag = "2")]
    pub teacher_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTeacherToClassResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 移除老师
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTeacherFromClassRequest {
    #[prost(string, tag = "1")]
    pub class_id: std::string::String,
    #[prost(string, tag = "2")]
    pub teacher_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTeacherFromClassResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 添加学生
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddStudentToClassRequest {
    #[prost(string, tag = "1")]
    pub class_id: std::string::String,
    #[prost(string, tag = "2")]
    pub student_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddStudentToClassResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 移除学生
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveStudentFromClassRequest {
    #[prost(string, tag = "1")]
    pub class_id: std::string::String,
    #[prost(string, tag = "2")]
    pub student_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveStudentFromClassResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    /// bson document {id, name, writeable_fields, readonly_fields}
    #[prost(bytes, tag = "1")]
    pub data: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEntityRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(bytes, tag = "2")]
    pub data: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEntityResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: std::string::String,
    #[prost(bytes, tag = "3")]
    pub data: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEntityRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEntityResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEntityTemplateRequest {
    /// 模板对应管理
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
    /// 属性:值 列表
    #[prost(bytes, repeated, tag = "2")]
    pub fields: ::std::vec::Vec<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEntityTemplateResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityTemplateRequest {
    /// 模板编号
    #[prost(string, tag = "1")]
    pub template_id: std::string::String,
    /// 属性:值 列表
    #[prost(bytes, repeated, tag = "2")]
    pub fields: ::std::vec::Vec<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityTemplateResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEntityTemplateRequest {
    #[prost(string, tag = "1")]
    pub template_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEntityTemplateResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 映像请求
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageViewRequest {
    #[prost(string, tag = "1")]
    pub manage_name: std::string::String,
}
/// 映像返回
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageViewResponse {
    #[prost(string, tag = "1")]
    pub view_token: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(bytes, tag = "2")]
    pub name: std::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub data_type: std::string::String,
    #[prost(bool, tag = "4")]
    pub removed: bool,
}
/// 取得管理列表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManagesRequest {}
/// 取得管理描写
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageSchemaRequest {
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageSchemaResponse {
    #[prost(bytes, repeated, tag = "1")]
    pub schema: ::std::vec::Vec<std::vec::Vec<u8>>,
}
/// 添加管理属性
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSchemaFieldRequest {
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub field: ::std::option::Option<Field>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSchemaFieldResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 移除管理属性
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSchemaFieldRequest {
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
    #[prost(int32, tag = "2")]
    pub field_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSchemaFieldResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 编辑管理属性
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditSchemaFieldNameRequest {
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
    #[prost(int32, tag = "2")]
    pub field_id: i32,
    #[prost(string, tag = "3")]
    pub language: std::string::String,
    #[prost(string, tag = "4")]
    pub new_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditSchemaFieldNameResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(int32, tag = "1")]
    pub event_id: i32,
    #[prost(bytes, tag = "2")]
    pub context: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventQueue {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventHandle {
    #[prost(int32, tag = "1")]
    pub event_id: i32,
    #[prost(int32, tag = "2")]
    pub queue_id: i32,
    #[prost(int32, tag = "3")]
    pub manage_id: i32,
    #[prost(string, tag = "4")]
    pub name: std::string::String,
}
/// 新建事件
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventRequest {
    #[prost(int32, tag = "2")]
    pub manage_id: i32,
    #[prost(string, tag = "1")]
    pub event_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 编辑 事件表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEventRequest {
    #[prost(int32, tag = "1")]
    pub event_id: i32,
    #[prost(bytes, tag = "2")]
    pub new_values: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEventResponse {
    #[prost(bool, tag = "1")]
    pub failed: bool,
}
/// 添加目标队列
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEventTargetQueueRequest {
    #[prost(int32, tag = "1")]
    pub event_id: i32,
    #[prost(message, optional, tag = "3")]
    pub target: ::std::option::Option<EventQueue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEventTargetQueueResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 新建事件队列
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventQueueRequest {
    #[prost(int32, tag = "2")]
    pub manage_id: i32,
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventQueueResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 新建事件处理
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventHandleRequest {
    #[prost(int32, tag = "2")]
    pub event_id: i32,
    #[prost(int32, tag = "3")]
    pub queue_id: i32,
    #[prost(string, tag = "4")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventHandleResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 触发事件
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitEventRequest {
    #[prost(message, optional, tag = "1")]
    pub event: ::std::option::Option<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitEventResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 连接事件队列
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkEventQueueRequest {
    #[prost(int32, tag = "1")]
    pub queue_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkRequest {
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 添加
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPhaseForWorkRequest {
    #[prost(string, tag = "1")]
    pub work_id: std::string::String,
    #[prost(string, tag = "2")]
    pub phase_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPhaseForWorkResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkNodeForProcedureGraphRequest {
    #[prost(string, tag = "1")]
    pub procedure_id: std::string::String,
    #[prost(string, tag = "2")]
    pub node_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkNodeForProcedureGraphResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkNodeLinkRequest {
    #[prost(string, tag = "1")]
    pub procedure_id: std::string::String,
    #[prost(string, tag = "2")]
    pub start_node_id: std::string::String,
    #[prost(string, tag = "3")]
    pub out_slot: std::string::String,
    #[prost(string, tag = "4")]
    pub end_node_id: std::string::String,
    #[prost(string, tag = "5")]
    pub in_slot: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkNodeLinkResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveWorkNodeLinkRequest {
    #[prost(string, tag = "1")]
    pub procedure_id: std::string::String,
    #[prost(string, tag = "2")]
    pub start_node_id: std::string::String,
    #[prost(string, tag = "3")]
    pub out_slot: std::string::String,
    #[prost(string, tag = "4")]
    pub end_node_id: std::string::String,
    #[prost(string, tag = "5")]
    pub in_slot: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveWorkNodeLinkResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignWorkNodeToWorkerRequest {
    #[prost(string, tag = "1")]
    pub work_node_id: std::string::String,
    #[prost(string, tag = "2")]
    pub worker_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignWorkNodeToWorkerResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTaskForWorkNodeRequest {
    #[prost(string, tag = "1")]
    pub work_node_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTaskForWorkNodeResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataForTaskRequest {
    #[prost(string, tag = "1")]
    pub task_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataForTaskResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateDataToTaskRequest {
    #[prost(string, tag = "1")]
    pub data_id: std::string::String,
    #[prost(string, tag = "2")]
    pub task_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateDataToTaskResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitTaskRequest {
    #[prost(string, tag = "1")]
    pub task_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitTaskResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkTaskStatusRequest {
    #[prost(string, tag = "1")]
    pub task_id: std::string::String,
    #[prost(string, tag = "2")]
    pub status_set_id: std::string::String,
    #[prost(int32, tag = "3")]
    pub status_index: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkTaskStatusResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkPhaseRequest {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkPhaseResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditWorkPhaseRequest {
    #[prost(string, tag = "1")]
    pub phase_id: std::string::String,
    #[prost(bytes, tag = "2")]
    pub new_phase: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditWorkPhaseResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataSlotForWorkNodeRequest {
    #[prost(string, tag = "1")]
    pub node_id: std::string::String,
    #[prost(enumeration = "SlotType", tag = "2")]
    pub slot_type: i32,
    #[prost(string, tag = "3")]
    pub slot_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataSlotForWorkNodeResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataSlotForWorkNodeRequest {
    #[prost(string, tag = "1")]
    pub node_id: std::string::String,
    #[prost(string, tag = "2")]
    pub slot_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataSlotForWorkNodeResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDataToDataSlotRequest {
    #[prost(string, tag = "1")]
    pub node_id: std::string::String,
    #[prost(string, tag = "2")]
    pub slot_name: std::string::String,
    #[prost(string, tag = "3")]
    pub data_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDataToDataSlotResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SlotType {
    /// 参考数据
    RefrenceData = 0,
    /// 依赖
    DepedentData = 1,
    /// 工作输出
    WorkData = 2,
    /// 输出
    OutData = 3,
}
/// 编码参照
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDomainRequest {
    /// 门类编码
    #[prost(string, tag = "1")]
    pub section_code: std::string::String,
    /// 大类编码
    #[prost(string, tag = "2")]
    pub division_code: std::string::String,
    ///中类编码
    #[prost(string, tag = "3")]
    pub group_code: std::string::String,
    /// 小类编码
    #[prost(string, tag = "4")]
    pub class_code: std::string::String,
    ///描述
    #[prost(string, tag = "5")]
    pub descriptions: std::string::String,
    #[prost(string, tag = "6")]
    pub language: std::string::String,
    #[prost(string, tag = "7")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDomainResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPointRequest {
    #[prost(string, tag = "1")]
    pub domain_id: std::string::String,
    #[prost(string, tag = "2")]
    pub language: std::string::String,
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPointResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenamePointRequest {
    #[prost(string, tag = "1")]
    pub point_id: std::string::String,
    #[prost(string, tag = "2")]
    pub language: std::string::String,
    #[prost(string, tag = "3")]
    pub new_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenamePointResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendPointNameRequest {
    #[prost(string, tag = "1")]
    pub point_id: std::string::String,
    #[prost(string, tag = "2")]
    pub language: std::string::String,
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendPointNameResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewGraphRequest {
    #[prost(string, tag = "1")]
    pub domain_id: std::string::String,
    #[prost(enumeration = "IndividualLevel", tag = "2")]
    pub level: i32,
    #[prost(string, tag = "3")]
    pub individual_id: std::string::String,
    #[prost(string, tag = "4")]
    pub language: std::string::String,
    #[prost(string, tag = "5")]
    pub name: std::string::String,
    #[prost(enumeration = "ComposeType", tag = "6")]
    pub compose_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewGraphResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameGraphRequest {
    #[prost(string, tag = "1")]
    pub graph_id: std::string::String,
    #[prost(string, tag = "2")]
    pub language: std::string::String,
    #[prost(string, tag = "3")]
    pub new_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameGraphResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEdgeToGraphRequest {
    #[prost(string, tag = "1")]
    pub graph_id: std::string::String,
    #[prost(string, tag = "2")]
    pub start_id: std::string::String,
    #[prost(string, tag = "3")]
    pub end_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEdgeToGraphResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEdgeFromGraphRequest {
    #[prost(string, tag = "1")]
    pub graph_id: std::string::String,
    #[prost(string, tag = "2")]
    pub start_id: std::string::String,
    #[prost(string, tag = "3")]
    pub end_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEdgeFromGraphResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagToEdgeRequest {
    #[prost(string, tag = "1")]
    pub graph_id: std::string::String,
    #[prost(string, tag = "2")]
    pub start_id: std::string::String,
    #[prost(string, tag = "3")]
    pub end_id: std::string::String,
    #[prost(string, tag = "4")]
    pub tag_type: std::string::String,
    #[prost(double, tag = "5")]
    pub value: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagToEdgeResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IndividualLevel {
    Orgnization = 0,
    Department = 1,
    Person = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComposeType {
    Star = 0,
    Mind = 1,
    UpDown = 2,
    DownUp = 3,
    LeftRight = 4,
    RightLeft = 5,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTagTypeRequest {
    #[prost(string, tag = "1")]
    pub domain_id: std::string::String,
    #[prost(string, tag = "2")]
    pub language: std::string::String,
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    #[prost(string, tag = "4")]
    pub description: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTagTypeResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDataFieldToTagTypeRequest {
    #[prost(string, tag = "1")]
    pub tag_type_id: std::string::String,
    #[prost(string, tag = "2")]
    pub language: std::string::String,
    #[prost(string, tag = "3")]
    pub field_name: std::string::String,
    #[prost(enumeration = "TagDataType", tag = "4")]
    pub data_type: i32,
    #[prost(bytes, tag = "5")]
    pub default_value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDataFieldToTagTypeResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataFieldFromTagTypeRequest {
    #[prost(string, tag = "1")]
    pub tag_type_id: std::string::String,
    #[prost(string, tag = "2")]
    pub language: std::string::String,
    #[prost(string, tag = "3")]
    pub field_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataFieldFromTagTypeResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditDataFieldForTagTypeRequest {
    #[prost(string, tag = "1")]
    pub tag_type_id: std::string::String,
    #[prost(string, tag = "2")]
    pub language: std::string::String,
    #[prost(string, tag = "3")]
    pub old_field_name: std::string::String,
    #[prost(string, tag = "4")]
    pub new_field_name: std::string::String,
    #[prost(enumeration = "TagDataType", tag = "5")]
    pub new_data_type: i32,
    #[prost(bytes, tag = "6")]
    pub new_default_value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditDataFieldForTagTypeResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TagDataType {
    ///选项
    Option = 0,
    /// 文字
    Text = 1,
    /// 时长
    Dueration = 2,
    ///日期
    Date = 3,
    /// 日期时间
    DateTime = 4,
    /// 量, 带有单位
    Quantity = 5,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(float, tag = "2")]
    pub ammount: f32,
}
/// 添加标签
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagRequest {
    #[prost(string, tag = "1")]
    pub tag_map_id: std::string::String,
    #[prost(string, tag = "2")]
    pub point_id: std::string::String,
    #[prost(string, tag = "3")]
    pub tag_type_id: std::string::String,
    #[prost(bytes, tag = "4")]
    pub tag_value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 设置标签值
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTagValueRequest {
    #[prost(string, tag = "1")]
    pub tag_map_id: std::string::String,
    #[prost(string, tag = "2")]
    pub point_id: std::string::String,
    #[prost(string, tag = "3")]
    pub tag_type_id: std::string::String,
    #[prost(bytes, tag = "4")]
    pub tag_value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTagValueResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 移除标签
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTagRequest {
    #[prost(string, tag = "1")]
    pub tag_map_id: std::string::String,
    #[prost(string, tag = "2")]
    pub point_id: std::string::String,
    #[prost(string, tag = "3")]
    pub tag_type_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTagResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagMapRequest {
    #[prost(string, tag = "1")]
    pub roadmap_id: std::string::String,
    #[prost(string, tag = "2")]
    pub language: std::string::String,
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagMapResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 新路线图
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewRoadmapRequest {
    #[prost(string, tag = "1")]
    pub graph_id: std::string::String,
    #[prost(string, tag = "3")]
    pub language: std::string::String,
    #[prost(string, tag = "4")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewRoadmapResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 设置公开
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoadmapPublicRequest {
    #[prost(string, tag = "1")]
    pub roadmap_id: std::string::String,
    #[prost(bool, tag = "2")]
    pub public: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoadmapPublicResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 添加访问者
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMapVisitorRequest {
    #[prost(string, tag = "1")]
    pub roadmap_id: std::string::String,
    #[prost(enumeration = "RoadmapVisType", tag = "2")]
    pub visitor_type: i32,
    #[prost(string, tag = "3")]
    pub visitor_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMapVisitorResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 添加标签类型
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagTypeRequest {
    #[prost(string, tag = "1")]
    pub roadmap_id: std::string::String,
    #[prost(string, tag = "2")]
    pub tag_type_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagTypeResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 移除标签类型
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTagTypeRequest {
    #[prost(string, tag = "1")]
    pub roadmap_id: std::string::String,
    #[prost(string, tag = "2")]
    pub tag_type_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTagTypeResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 路线图可访问类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoadmapVisType {
    VisTypePerson = 0,
    VisTypeClass = 1,
    VisTypeGroup = 2,
    VisTypeDepartment = 3,
    VisTypeOrganization = 4,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCommentRequest {
    #[prost(string, tag = "1")]
    pub target_manage_id: std::string::String,
    #[prost(string, tag = "2")]
    pub target_entity_id: std::string::String,
    #[prost(string, tag = "3")]
    pub contents: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCommentResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditCommentRequest {
    #[prost(string, tag = "1")]
    pub comment_id: std::string::String,
    #[prost(string, tag = "2")]
    pub new_contents: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditCommentResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCommentRequest {
    #[prost(string, tag = "1")]
    pub target_manage_id: std::string::String,
    #[prost(string, tag = "2")]
    pub target_entity_id: std::string::String,
    #[prost(string, tag = "3")]
    pub comment_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCommentResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewQuestionRequest {
    #[prost(string, tag = "1")]
    pub graph_id: std::string::String,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    #[prost(string, tag = "3")]
    pub contents: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewQuestionResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditQuestionRequest {
    #[prost(string, tag = "2")]
    pub question_id: std::string::String,
    #[prost(string, tag = "3")]
    pub new_name: std::string::String,
    #[prost(string, tag = "4")]
    pub new_contents: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditQuestionResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveQuestionRequest {
    #[prost(string, tag = "1")]
    pub graph_id: std::string::String,
    #[prost(string, tag = "2")]
    pub question_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveQuestionResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAnswerRequest {
    #[prost(string, tag = "1")]
    pub question_id: std::string::String,
    #[prost(string, tag = "2")]
    pub contents: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAnswerResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditAnswerRequest {
    #[prost(string, tag = "1")]
    pub answer_id: std::string::String,
    #[prost(string, tag = "2")]
    pub new_contents: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditAnswerResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAnswerRequest {
    #[prost(string, tag = "1")]
    pub question_id: std::string::String,
    #[prost(string, tag = "2")]
    pub answer_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAnswerResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod cashmere_grpc_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct CashmereGrpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CashmereGrpcClient<tonic::transport::Channel> {
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
    impl<T> CashmereGrpcClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
    }
    impl<T: Clone> Clone for CashmereGrpcClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CashmereGrpcClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CashmereGrpcClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod cashmere_grpc_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CashmereGrpcServer."]
    #[async_trait]
    pub trait CashmereGrpc: Send + Sync + 'static {}
    #[derive(Debug)]
    pub struct CashmereGrpcServer<T: CashmereGrpc> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: CashmereGrpc> CashmereGrpcServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for CashmereGrpcServer<T>
    where
        T: CashmereGrpc,
        B: HttpBody + Send + Sync + 'static,
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
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: CashmereGrpc> Clone for CashmereGrpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: CashmereGrpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CashmereGrpc> tonic::transport::NamedService for CashmereGrpcServer<T> {
        const NAME: &'static str = "cashmere.CashmereGrpc";
    }
}
