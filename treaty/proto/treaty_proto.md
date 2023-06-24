# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [treaty.proto](#treaty-proto)
    - [AcceptPendingActionReply](#treaty_proto-AcceptPendingActionReply)
    - [AcceptPendingActionRequest](#treaty_proto-AcceptPendingActionRequest)
    - [AcceptPendingContractReply](#treaty_proto-AcceptPendingContractReply)
    - [AcceptPendingContractRequest](#treaty_proto-AcceptPendingContractRequest)
    - [AddParticipantReply](#treaty_proto-AddParticipantReply)
    - [AddParticipantRequest](#treaty_proto-AddParticipantRequest)
    - [AuthRequest](#treaty_proto-AuthRequest)
    - [AuthResult](#treaty_proto-AuthResult)
    - [ChangeDeletesFromHostBehaviorReply](#treaty_proto-ChangeDeletesFromHostBehaviorReply)
    - [ChangeDeletesFromHostBehaviorRequest](#treaty_proto-ChangeDeletesFromHostBehaviorRequest)
    - [ChangeDeletesToHostBehaviorReply](#treaty_proto-ChangeDeletesToHostBehaviorReply)
    - [ChangeDeletesToHostBehaviorRequest](#treaty_proto-ChangeDeletesToHostBehaviorRequest)
    - [ChangeHostStatusReply](#treaty_proto-ChangeHostStatusReply)
    - [ChangeHostStatusRequest](#treaty_proto-ChangeHostStatusRequest)
    - [ChangeUpdatesFromHostBehaviorRequest](#treaty_proto-ChangeUpdatesFromHostBehaviorRequest)
    - [ChangeUpdatesToHostBehaviorReply](#treaty_proto-ChangeUpdatesToHostBehaviorReply)
    - [ChangeUpdatesToHostBehaviorRequest](#treaty_proto-ChangeUpdatesToHostBehaviorRequest)
    - [ChangesUpdatesFromHostBehaviorReply](#treaty_proto-ChangesUpdatesFromHostBehaviorReply)
    - [ColumnSchema](#treaty_proto-ColumnSchema)
    - [Contract](#treaty_proto-Contract)
    - [CreatePartialDatabaseRequest](#treaty_proto-CreatePartialDatabaseRequest)
    - [CreatePartialDatabaseResult](#treaty_proto-CreatePartialDatabaseResult)
    - [CreateTableRequest](#treaty_proto-CreateTableRequest)
    - [CreateTableResult](#treaty_proto-CreateTableResult)
    - [CreateUserDatabaseReply](#treaty_proto-CreateUserDatabaseReply)
    - [CreateUserDatabaseRequest](#treaty_proto-CreateUserDatabaseRequest)
    - [DatabaseSchema](#treaty_proto-DatabaseSchema)
    - [DeleteDataRequest](#treaty_proto-DeleteDataRequest)
    - [DeleteDataResult](#treaty_proto-DeleteDataResult)
    - [EnableCoooperativeFeaturesReply](#treaty_proto-EnableCoooperativeFeaturesReply)
    - [EnableCoooperativeFeaturesRequest](#treaty_proto-EnableCoooperativeFeaturesRequest)
    - [ExecuteCooperativeWriteReply](#treaty_proto-ExecuteCooperativeWriteReply)
    - [ExecuteCooperativeWriteRequest](#treaty_proto-ExecuteCooperativeWriteRequest)
    - [ExecuteReadReply](#treaty_proto-ExecuteReadReply)
    - [ExecuteReadRequest](#treaty_proto-ExecuteReadRequest)
    - [ExecuteWriteReply](#treaty_proto-ExecuteWriteReply)
    - [ExecuteWriteRequest](#treaty_proto-ExecuteWriteRequest)
    - [GenerateContractReply](#treaty_proto-GenerateContractReply)
    - [GenerateContractRequest](#treaty_proto-GenerateContractRequest)
    - [GenerateHostInfoReply](#treaty_proto-GenerateHostInfoReply)
    - [GenerateHostInfoRequest](#treaty_proto-GenerateHostInfoRequest)
    - [GetActiveContractReply](#treaty_proto-GetActiveContractReply)
    - [GetActiveContractRequest](#treaty_proto-GetActiveContractRequest)
    - [GetCooperativeHostsReply](#treaty_proto-GetCooperativeHostsReply)
    - [GetCooperativeHostsRequest](#treaty_proto-GetCooperativeHostsRequest)
    - [GetDataHashReply](#treaty_proto-GetDataHashReply)
    - [GetDataHashRequest](#treaty_proto-GetDataHashRequest)
    - [GetDataLogTableStatusReply](#treaty_proto-GetDataLogTableStatusReply)
    - [GetDataLogTableStatusRequest](#treaty_proto-GetDataLogTableStatusRequest)
    - [GetDatabasesReply](#treaty_proto-GetDatabasesReply)
    - [GetDatabasesRequest](#treaty_proto-GetDatabasesRequest)
    - [GetDeletesFromHostBehaviorReply](#treaty_proto-GetDeletesFromHostBehaviorReply)
    - [GetDeletesFromHostBehaviorRequest](#treaty_proto-GetDeletesFromHostBehaviorRequest)
    - [GetDeletesToHostBehaviorReply](#treaty_proto-GetDeletesToHostBehaviorReply)
    - [GetDeletesToHostBehaviorRequest](#treaty_proto-GetDeletesToHostBehaviorRequest)
    - [GetLogicalStoragePolicyReply](#treaty_proto-GetLogicalStoragePolicyReply)
    - [GetLogicalStoragePolicyRequest](#treaty_proto-GetLogicalStoragePolicyRequest)
    - [GetLogsByLastNumberReply](#treaty_proto-GetLogsByLastNumberReply)
    - [GetLogsByLastNumberRequest](#treaty_proto-GetLogsByLastNumberRequest)
    - [GetParticipantsReply](#treaty_proto-GetParticipantsReply)
    - [GetParticipantsRequest](#treaty_proto-GetParticipantsRequest)
    - [GetPendingActionsReply](#treaty_proto-GetPendingActionsReply)
    - [GetPendingActionsRequest](#treaty_proto-GetPendingActionsRequest)
    - [GetReadRowIdsReply](#treaty_proto-GetReadRowIdsReply)
    - [GetReadRowIdsRequest](#treaty_proto-GetReadRowIdsRequest)
    - [GetRowFromPartialDatabaseRequest](#treaty_proto-GetRowFromPartialDatabaseRequest)
    - [GetRowFromPartialDatabaseResult](#treaty_proto-GetRowFromPartialDatabaseResult)
    - [GetSettingsReply](#treaty_proto-GetSettingsReply)
    - [GetSettingsRequest](#treaty_proto-GetSettingsRequest)
    - [GetUpdatesFromHostBehaviorReply](#treaty_proto-GetUpdatesFromHostBehaviorReply)
    - [GetUpdatesFromHostBehaviorRequest](#treaty_proto-GetUpdatesFromHostBehaviorRequest)
    - [GetUpdatesToHostBehaviorReply](#treaty_proto-GetUpdatesToHostBehaviorReply)
    - [GetUpdatesToHostBehaviorRequest](#treaty_proto-GetUpdatesToHostBehaviorRequest)
    - [HasTableReply](#treaty_proto-HasTableReply)
    - [HasTableRequest](#treaty_proto-HasTableRequest)
    - [Host](#treaty_proto-Host)
    - [HostInfoReply](#treaty_proto-HostInfoReply)
    - [HostInfoStatus](#treaty_proto-HostInfoStatus)
    - [HostNetwork](#treaty_proto-HostNetwork)
    - [InsertDataRequest](#treaty_proto-InsertDataRequest)
    - [InsertDataResult](#treaty_proto-InsertDataResult)
    - [NotifyHostOfRemovedRowRequest](#treaty_proto-NotifyHostOfRemovedRowRequest)
    - [NotifyHostOfRemovedRowResult](#treaty_proto-NotifyHostOfRemovedRowResult)
    - [Participant](#treaty_proto-Participant)
    - [ParticipantAcceptsContractRequest](#treaty_proto-ParticipantAcceptsContractRequest)
    - [ParticipantAcceptsContractResult](#treaty_proto-ParticipantAcceptsContractResult)
    - [ParticipantStatus](#treaty_proto-ParticipantStatus)
    - [PendingStatement](#treaty_proto-PendingStatement)
    - [RejectPendingContractReply](#treaty_proto-RejectPendingContractReply)
    - [RejectPendingContractRequest](#treaty_proto-RejectPendingContractRequest)
    - [RevokeReply](#treaty_proto-RevokeReply)
    - [Row](#treaty_proto-Row)
    - [RowInfo](#treaty_proto-RowInfo)
    - [RowParticipantAddress](#treaty_proto-RowParticipantAddress)
    - [RowRemoteMetadata](#treaty_proto-RowRemoteMetadata)
    - [RowValue](#treaty_proto-RowValue)
    - [SaveContractRequest](#treaty_proto-SaveContractRequest)
    - [SaveContractResult](#treaty_proto-SaveContractResult)
    - [SendParticipantContractReply](#treaty_proto-SendParticipantContractReply)
    - [SendParticipantContractRequest](#treaty_proto-SendParticipantContractRequest)
    - [SetDataLogTableStatusReply](#treaty_proto-SetDataLogTableStatusReply)
    - [SetDataLogTableStatusRequest](#treaty_proto-SetDataLogTableStatusRequest)
    - [SetLogicalStoragePolicyReply](#treaty_proto-SetLogicalStoragePolicyReply)
    - [SetLogicalStoragePolicyRequest](#treaty_proto-SetLogicalStoragePolicyRequest)
    - [StatementResultset](#treaty_proto-StatementResultset)
    - [TableSchema](#treaty_proto-TableSchema)
    - [Telemetry](#treaty_proto-Telemetry)
    - [TestReply](#treaty_proto-TestReply)
    - [TestRequest](#treaty_proto-TestRequest)
    - [TokenReply](#treaty_proto-TokenReply)
    - [TransactionInfo](#treaty_proto-TransactionInfo)
    - [TreatyError](#treaty_proto-TreatyError)
    - [TreatyLogEntry](#treaty_proto-TreatyLogEntry)
    - [TreatyWarning](#treaty_proto-TreatyWarning)
    - [TryAuthAtParticipantRequest](#treaty_proto-TryAuthAtParticipantRequest)
    - [TryAuthAtPartipantReply](#treaty_proto-TryAuthAtPartipantReply)
    - [TryAuthRequest](#treaty_proto-TryAuthRequest)
    - [TryAuthResult](#treaty_proto-TryAuthResult)
    - [UpdateDataRequest](#treaty_proto-UpdateDataRequest)
    - [UpdateDataResult](#treaty_proto-UpdateDataResult)
    - [UpdateRowDataHashForHostRequest](#treaty_proto-UpdateRowDataHashForHostRequest)
    - [UpdateRowDataHashForHostResult](#treaty_proto-UpdateRowDataHashForHostResult)
    - [VersionReply](#treaty_proto-VersionReply)
    - [Versions](#treaty_proto-Versions)
    - [ViewPendingContractsReply](#treaty_proto-ViewPendingContractsReply)
    - [ViewPendingContractsRequest](#treaty_proto-ViewPendingContractsRequest)
  
    - [DataService](#treaty_proto-DataService)
    - [UserService](#treaty_proto-UserService)
  
- [Scalar Value Types](#scalar-value-types)



<a name="treaty-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## treaty.proto



<a name="treaty_proto-AcceptPendingActionReply"></a>

### AcceptPendingActionReply
Replies with the result of accepting a pending action at a Participant if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the acceptance of the action is successful. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to accept the action. |






<a name="treaty_proto-AcceptPendingActionRequest"></a>

### AcceptPendingActionRequest
Requests to accept a pending action at a Participant if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |
| row_id | [uint32](#uint32) |  | The row id. |






<a name="treaty_proto-AcceptPendingContractReply"></a>

### AcceptPendingContractReply
Replies with the result of accepting a pending contract, if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the request was successful. |
| message | [string](#string) |  | A message with any additional information. This field may be blank. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to accept the pending contract. |






<a name="treaty_proto-AcceptPendingContractRequest"></a>

### AcceptPendingContractRequest
Requests Treaty to accept the pending contract from the specified Host, if authenticated. 
This will send a message back to the host that we are ready to accept data 
and will create additional meta-data structures to support the contract. 
For more information, see the manual.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| host_alias | [string](#string) |  | The host that has sent us the pending contract. |






<a name="treaty_proto-AddParticipantReply"></a>

### AddParticipantReply
Replies with the result of adding a Participant if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If adding the Participant was successful. |
| message | [string](#string) |  | A message describing any additional details if needed. This field can be empty. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to add the Participant. |






<a name="treaty_proto-AddParticipantRequest"></a>

### AddParticipantRequest
Request to add the Participant to the specified database if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| alias | [string](#string) |  | An alias for this participant. |
| ip4_address | [string](#string) |  | The IP address for this Participant, in IP 4 format. |
| port | [uint32](#uint32) |  | The database port number for this Participant. |
| http_addr | [string](#string) |  | The HTTP address for this Participant. |
| http_port | [uint32](#uint32) |  | The HTTP port for this Participant. |
| id | [string](#string) | optional | The Host Id for this participant. This field is optional. This is used if Treaty is being hosted by a `treaty-proxy` instance, where multiple Treaty instances are hosted and you need to uniquely identify which Treaty instance you are attempting to talk to. |






<a name="treaty_proto-AuthRequest"></a>

### AuthRequest
Credentials to authenticate against Treaty.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_name | [string](#string) |  | The name of the user. |
| pw | [string](#string) |  | The pw of the user. |
| pw_hash | [bytes](#bytes) |  | A hash of the pw of the user. |
| token | [bytes](#bytes) |  | A generated token of the pw of the user. |
| jwt | [string](#string) |  | A Json Web Token in place of credentials. |
| id | [string](#string) | optional | An optional Host Id of the Treaty instance. This is used when talking to a `treaty-proxy` instance. |






<a name="treaty_proto-AuthResult"></a>

### AuthResult
A message describing the results of an authentication attempt.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| is_authenticated | [bool](#bool) |  | If authenticated. |
| message | [string](#string) | optional | An optional message for any additional information. |






<a name="treaty_proto-ChangeDeletesFromHostBehaviorReply"></a>

### ChangeDeletesFromHostBehaviorReply
Replies with the result of changing the &#34;DeletesFromHost&#34; behavior if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the request was successful or not. |
| message | [string](#string) |  | A message if any additional information is available. This value can be empty. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to change the behavior. |






<a name="treaty_proto-ChangeDeletesFromHostBehaviorRequest"></a>

### ChangeDeletesFromHostBehaviorRequest
Request to change the &#34;DeletesFromHost&#34; behavior if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |
| behavior | [uint32](#uint32) |  | The behavior to change to. This value is defined in the /treaty/treaty-types/enums.rs file. |






<a name="treaty_proto-ChangeDeletesToHostBehaviorReply"></a>

### ChangeDeletesToHostBehaviorReply
Replies with the result of the request to change the &#34;DeletesToHost&#34; behavior if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the request was successful. |
| message | [string](#string) |  | A message if any additional information is needed. This value can be empty. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to get the requested data hash. |






<a name="treaty_proto-ChangeDeletesToHostBehaviorRequest"></a>

### ChangeDeletesToHostBehaviorRequest
Request to change the &#34;DeletesToHost&#34; behavior if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |
| behavior | [uint32](#uint32) |  | The &#34;DeletesToHost&#34; before setting. This value is defined in the /treaty/treaty-types/enums.rs file. |






<a name="treaty_proto-ChangeHostStatusReply"></a>

### ChangeHostStatusReply
Replies with the result of changing the host status if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the request was successful. |
| status | [uint32](#uint32) |  | The status the value was changed to. This echoes what was sent. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to change the host status. |






<a name="treaty_proto-ChangeHostStatusRequest"></a>

### ChangeHostStatusRequest
Request to change the status of a Host to ALLOW/DENY if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| host_alias | [string](#string) |  | The host alias. |
| host_id | [string](#string) |  | The host id. |
| status | [uint32](#uint32) |  | The status to change for the host. This value is defined in the /treaty/treaty-types/enums.rs file. |






<a name="treaty_proto-ChangeUpdatesFromHostBehaviorRequest"></a>

### ChangeUpdatesFromHostBehaviorRequest
Request to change the &#34;UpdatesFromHost&#34; behavior if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |
| behavior | [uint32](#uint32) |  | The behavior to change to. This value is defined in the /treaty/treaty-types/enums.rs file. |






<a name="treaty_proto-ChangeUpdatesToHostBehaviorReply"></a>

### ChangeUpdatesToHostBehaviorReply
Replies with the result of attempting to change the &#34;UpdatesToHost&#34; behavior if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the request was successful. |
| message | [string](#string) |  | A message if any additional information is needed. This value can be empty. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to set the behavior. |






<a name="treaty_proto-ChangeUpdatesToHostBehaviorRequest"></a>

### ChangeUpdatesToHostBehaviorRequest
A request to change the &#34;UpdatesToHostBehavior&#34; if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |
| behavior | [uint32](#uint32) |  | The behavior to change to. This value is defined in the /treaty/treaty-types/enums.rs file. |






<a name="treaty_proto-ChangesUpdatesFromHostBehaviorReply"></a>

### ChangesUpdatesFromHostBehaviorReply
Replies with the result of changing the &#34;UpdatesFromHost&#34; behavior if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the request was successful or not. |
| message | [string](#string) |  | A message with any additional information. This value can be empty. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to change the behavior. |






<a name="treaty_proto-ColumnSchema"></a>

### ColumnSchema
A message for describing schema information of a column in a database table.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| column_name | [string](#string) |  | The column name. |
| column_type | [uint32](#uint32) |  | The column type. This value is defined in the /treaty/treaty-types/enums.rs file. |
| column_length | [uint32](#uint32) |  | The max or fixed length of the column, if applicable. |
| is_nullable | [bool](#bool) |  | If the column is nullable or not. |
| ordinal | [uint32](#uint32) |  | The ordinal value of the column, i.e. the order in which the column appears in the table. |
| table_id | [string](#string) |  | Empty string in a request, populated in a response with the table GUID the column is attached to. |
| column_id | [string](#string) |  | Empty string in a request, populated in a response with the column GUID value. |
| is_primary_key | [bool](#bool) |  | If the column is the primary key of the table. If this is part of a list of columns, it is implied to be a composite primary key. |






<a name="treaty_proto-Contract"></a>

### Contract



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| contract_guid | [string](#string) |  | the unique contract id |
| description | [string](#string) |  | a description of the rights in the contract |
| schema | [DatabaseSchema](#treaty_proto-DatabaseSchema) |  | the schema of the entire database |
| contract_version | [string](#string) |  | a GUID representing the version of the contract |
| host_info | [Host](#treaty_proto-Host) |  | The host for the contract. |
| status | [uint32](#uint32) |  | the status of the contract, if applicable |






<a name="treaty_proto-CreatePartialDatabaseRequest"></a>

### CreatePartialDatabaseRequest
A message for creating a partial database.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| Telemetry | [Telemetry](#treaty_proto-Telemetry) |  | Additional debugging information. |
| database_name | [string](#string) |  | The database name. |






<a name="treaty_proto-CreatePartialDatabaseResult"></a>

### CreatePartialDatabaseResult
A message describing the results of a CreateDatabaseRequest.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the partial database creation was successful. |
| database_name | [string](#string) |  | The name of the database. |
| databaseId | [string](#string) |  | The id of the database. |
| is_error | [bool](#bool) |  | If there was an error creating the database. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error describing what happened during the request, if applicable. |






<a name="treaty_proto-CreateTableRequest"></a>

### CreateTableRequest
A message for creating a table in a database.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The user requesting the table creation. |
| database_name | [string](#string) |  | The database in which to create the table. |
| database_guid | [string](#string) |  | The database GUID in which to create the table. |
| table_name | [string](#string) |  | The name of the table to create. |
| columns | [ColumnSchema](#treaty_proto-ColumnSchema) | repeated | A list of columns for the table. |






<a name="treaty_proto-CreateTableResult"></a>

### CreateTableResult
A message for describing the result of a CreateTableRequest.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the table was created. |
| database_name | [string](#string) |  | The name of the database the table was created in. |
| result_message | [string](#string) |  | Any additional information if needed. This field can be blank. |
| database_id | [string](#string) |  | The database id the table was created in. |
| table_name | [string](#string) |  | The table name that was created. This should line up with the request made and is intended for confirmation. |
| table_id | [string](#string) |  | The table id that was created. |
| is_error | [bool](#bool) |  | If the request failed in any manner. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error detailing if the request failed in any manner. |






<a name="treaty_proto-CreateUserDatabaseReply"></a>

### CreateUserDatabaseReply
Replies with the result of creating a database if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_created | [bool](#bool) |  | If the database was created. |
| message | [string](#string) |  | A message describing any details if needed. This field can be blank. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to create the requested database. |






<a name="treaty_proto-CreateUserDatabaseRequest"></a>

### CreateUserDatabaseRequest
Requests to create a database if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |






<a name="treaty_proto-DatabaseSchema"></a>

### DatabaseSchema
A message for describing the schema of a database.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| database_name | [string](#string) |  | The database name. |
| database_id | [string](#string) |  | The database id. |
| tables | [TableSchema](#treaty_proto-TableSchema) | repeated | The tables of the database. |
| database_type | [uint32](#uint32) |  | The type of database: Sqlite, Postgres, or MySQL. This value is defined in the /treaty/treaty-types/enums.rs file. |
| treaty_database_type | [uint32](#uint32) |  | The type of Treaty database; i.e. A Host, Partial, or internal Treaty system database. This value is defined in the /treaty/treaty-types/enums.rs file. |
| cooperation_enabled | [bool](#bool) |  | If the database has cooperative features. |
| has_participants | [bool](#bool) |  | If the database has any participants. |






<a name="treaty_proto-DeleteDataRequest"></a>

### DeleteDataRequest
A request for Treaty to execute the provided DELETE statement. 
❗ Warning: At the moment, Treaty can only handle simple UPDATE statements. 
For more information, see the manual.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication requestl |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |
| cmd | [string](#string) |  | The actual DELETE statement. Note: This DELETE statement needs to match the field specified prior. |
| where_clause | [string](#string) |  | ❗ The WHERE clause of the delete statement. This field needs to match the WHERE clause if there is one in the prior field. Otherwise, this field can be left blank. |






<a name="treaty_proto-DeleteDataResult"></a>

### DeleteDataResult
Describes the result of Treaty executing the specified DELETE statement.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the command was successfully executed. |
| message | [string](#string) |  | A message providing further details if needed. This field can be blank. |
| rows | [RowInfo](#treaty_proto-RowInfo) | repeated | A message describing details of the rows impacted. |
| is_error | [bool](#bool) |  | Denotes if there was an error executing the DELETE statement. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error describing details if needed. |






<a name="treaty_proto-EnableCoooperativeFeaturesReply"></a>

### EnableCoooperativeFeaturesReply
Replies with the result of enabling cooperative features, if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If enabling cooperative features was successful. |
| message | [string](#string) |  | A message containing any additional details. This field may be blank. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to enable cooperative features on the specified database. |






<a name="treaty_proto-EnableCoooperativeFeaturesRequest"></a>

### EnableCoooperativeFeaturesRequest
Requests that Treaty enable cooperative features for a database, if authentiated. 
This modifies the database with additional structures to support adding Participants and other related actions. 
For more information, see the manual.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name to enable cooperative features. |






<a name="treaty_proto-ExecuteCooperativeWriteReply"></a>

### ExecuteCooperativeWriteReply
Replies with the result of Cooperative Write if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the result was successful. |
| total_rows_affected | [uint32](#uint32) |  | The total number of rows affected by the INSERT/UPDATE/DELETE statement. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to execute the Cooperative Write. |






<a name="treaty_proto-ExecuteCooperativeWriteRequest"></a>

### ExecuteCooperativeWriteRequest
Requests Treaty to execute the specified INSERT/UPDATE/DELETE statement both at the 
Host and the Participant if authenticated. 
This attempts to execute the action at the Participant and if successful
will keep metadata about the action at the Host. 
For more information, see the README.md or the manual.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentaction request. |
| database_name | [string](#string) |  | The name of the database. |
| sql_statement | [string](#string) |  | The INSERT/UPDATE/DELETE statement to execute at the Participant. |
| database_type | [uint32](#uint32) |  | The type of database: Sqlite, MySQL, Postgres. This value is defined in the /treaty/treaty-types/enums.rs file. |
| alias | [string](#string) |  | The participant alias this statement is for. |
| participant_id | [string](#string) |  | The participant id this statement is for. |
| where_clause | [string](#string) |  | The WHERE clause of the INSERT/UPDATE/STATEMENT. For technical reasons this needs to be the same as in the &#34;sql_statement&#34; field if applicable. This field can be empty. |






<a name="treaty_proto-ExecuteReadReply"></a>

### ExecuteReadReply
Replies with the result of the SELECT statement if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| total_resultsets | [uint64](#uint64) |  | The total number of result-sets. |
| results | [StatementResultset](#treaty_proto-StatementResultset) | repeated | The results of the query. |
| is_error | [bool](#bool) |  | Denotes if there was an error executing the query. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to execute the SELECT statement provided. |






<a name="treaty_proto-ExecuteReadRequest"></a>

### ExecuteReadRequest
Requests to execute the specified SELECT statement if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| sql_statement | [string](#string) |  | The SELECT SQL statement. |
| database_type | [uint32](#uint32) |  | The datababase type (Sqlite, MySQL, Postgres) |






<a name="treaty_proto-ExecuteWriteReply"></a>

### ExecuteWriteReply
Replies with the results of the provided INSERT/UPDATE/DELETE statement if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the statement executed without error. |
| total_rows_affected | [uint32](#uint32) |  | The total number of rows the statement affected, if applicable. |
| is_error | [bool](#bool) |  | Denotes if there was an error executing the statement. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was uanble to execute the INSERT/UPDATE/DELETE statement provided. |






<a name="treaty_proto-ExecuteWriteRequest"></a>

### ExecuteWriteRequest
Requests to execute the provided INSERT/UPDATE/DELETE statement if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| sql_statement | [string](#string) |  | The INSERT/UPDATE/DELETE statement to execute. |
| database_type | [uint32](#uint32) |  | The database type (Sqlite, MySQL, Postgres). |
| where_clause | [string](#string) |  | The WHERE clause of the statement, if applicable. ℹ️ Note: If the &#34;sql_statement&#34; includes a WHERE clause, duplicate the contents here. Otherwise, leave the string empty. This is needed because of a limitation with Treaty&#39;s implementation of Antlr. In the future, hopefully this field will not be needed. |






<a name="treaty_proto-GenerateContractReply"></a>

### GenerateContractReply
Replies with the status of generating a contract for the specified database if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If contract generation was successful. |
| message | [string](#string) |  | A message providing any additional details. This value can be empty. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to generate the contract. |






<a name="treaty_proto-GenerateContractRequest"></a>

### GenerateContractRequest
A request to generate a contract for the specified database. 
If there is already a database contract for this database, it will be marked as inactive and a new one
will be generated. 
❗ Note: You will need to ensure that each table in your database has a Logical Storage Policy set before 
generating a contract, otherwise this call will fail.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| host_name | [string](#string) |  | A host name to identify this Treaty instance to others. |
| description | [string](#string) |  | A general description for the contract. This will be made visible to Participants. |
| database_name | [string](#string) |  | The name of the database this contract is for. |
| remote_delete_behavior | [uint32](#uint32) |  | The Remote Delete Behavior for this Host for this contract. This value is defined in the /treaty/treaty-types/enums.rs file. |






<a name="treaty_proto-GenerateHostInfoReply"></a>

### GenerateHostInfoReply
Replies with the result of attempting to generate host information if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If creating host information was successful or not. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to generate host information. |






<a name="treaty_proto-GenerateHostInfoRequest"></a>

### GenerateHostInfoRequest
Request to generate host information for Treaty. 
This will assign our Treaty instance with the specified host name and an auto 
generated UUID if not already set. Otherwise, it will update the host name and token.

❗ WARNING: If calling this request, this will re-generate the host token which is used to identify this 
Treaty instance to other Treaty instances.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| host_name | [string](#string) |  | The friendly host name to use. |






<a name="treaty_proto-GetActiveContractReply"></a>

### GetActiveContractReply
Replies with the active contract for the specified database if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| contract | [Contract](#treaty_proto-Contract) |  | The active database contract. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to return the Active Contract for the specified database. |






<a name="treaty_proto-GetActiveContractRequest"></a>

### GetActiveContractRequest
Requests the current Active Contract for the specified database if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |






<a name="treaty_proto-GetCooperativeHostsReply"></a>

### GetCooperativeHostsReply
Responds with a list of hosts that this Treaty instance is cooperating with if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| hosts | [HostInfoStatus](#treaty_proto-HostInfoStatus) | repeated | The list of hosts this Treaty instance is cooperating with. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to return the list of Hosts. |






<a name="treaty_proto-GetCooperativeHostsRequest"></a>

### GetCooperativeHostsRequest
Requests a list of hosts that this Treaty instance is cooperating with if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |






<a name="treaty_proto-GetDataHashReply"></a>

### GetDataHashReply
Returns the requested data hash if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentiation result. |
| dataHash | [uint64](#uint64) |  | The requested data hash. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to get the requested data hash. |






<a name="treaty_proto-GetDataHashRequest"></a>

### GetDataHashRequest
Requests the saved data hash for the specified row id if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |
| row_id | [uint32](#uint32) |  | The row id. |






<a name="treaty_proto-GetDataLogTableStatusReply"></a>

### GetDataLogTableStatusReply
Replies with the status of data logging for the specified table if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| use_data_log | [bool](#bool) |  | If data logging was configured or not. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to get the status of data logging. |






<a name="treaty_proto-GetDataLogTableStatusRequest"></a>

### GetDataLogTableStatusRequest
Requests the status of data logging for the specified table if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |






<a name="treaty_proto-GetDatabasesReply"></a>

### GetDatabasesReply
Replies with the list of databses at Tretay if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| databases | [DatabaseSchema](#treaty_proto-DatabaseSchema) | repeated | The databases hosted at this Treaty instance. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to return the list of databases. |






<a name="treaty_proto-GetDatabasesRequest"></a>

### GetDatabasesRequest
Requests a list of the databases at Treaty if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  |  |






<a name="treaty_proto-GetDeletesFromHostBehaviorReply"></a>

### GetDeletesFromHostBehaviorReply
Responds with the current &#34;DeletesFromHost&#34; behavior for the specified database and table if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| behavior | [uint32](#uint32) | optional | The current behavior for the requested database and table. This value is defined in the /treaty/treaty-types/enum.rs file. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to return the current behavior. |






<a name="treaty_proto-GetDeletesFromHostBehaviorRequest"></a>

### GetDeletesFromHostBehaviorRequest
Requests the current &#34;DeletesFromHost&#34; behavior for the specified database and table if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |






<a name="treaty_proto-GetDeletesToHostBehaviorReply"></a>

### GetDeletesToHostBehaviorReply
Responds with the current &#34;DeletesToHost&#34; behavior if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| behavior | [uint32](#uint32) | optional | The current behavior for the requested database and table. This value is defined in the /treaty/treaty-types/enums.rs file. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to return the current behavior. |






<a name="treaty_proto-GetDeletesToHostBehaviorRequest"></a>

### GetDeletesToHostBehaviorRequest
Requests the current &#34;DeletesToHost&#34; behavior for the specified database and table if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |






<a name="treaty_proto-GetLogicalStoragePolicyReply"></a>

### GetLogicalStoragePolicyReply
Replies with the current Logical Storage Policy for the specified table if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| policy_mode | [uint32](#uint32) |  | The current Logical Storage policy for the requested table. This value is defined in the /treaty/treaty-types/enums.rs file. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to get the Logical Storage Policy for the specified table. |






<a name="treaty_proto-GetLogicalStoragePolicyRequest"></a>

### GetLogicalStoragePolicyRequest
Request the current Logical Storage Policy for the specified table if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |






<a name="treaty_proto-GetLogsByLastNumberReply"></a>

### GetLogsByLastNumberReply
Responds with the total requested number of logs if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| logs | [TreatyLogEntry](#treaty_proto-TreatyLogEntry) | repeated | A list of log entries. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to fetch logs. |






<a name="treaty_proto-GetLogsByLastNumberRequest"></a>

### GetLogsByLastNumberRequest
Requests Treaty to return the last X number of logs if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | Authentication. |
| number_of_logs | [uint32](#uint32) |  | The last number of logs to fetch. |






<a name="treaty_proto-GetParticipantsReply"></a>

### GetParticipantsReply
Replies with the list of Participants for the specified database if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| participants | [ParticipantStatus](#treaty_proto-ParticipantStatus) | repeated | A list of participants for the specified database. |
| is_error | [bool](#bool) |  | If the request has an error. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to return the list of participants for the specified database. |






<a name="treaty_proto-GetParticipantsRequest"></a>

### GetParticipantsRequest
Requests a list of participants for the specified database if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |






<a name="treaty_proto-GetPendingActionsReply"></a>

### GetPendingActionsReply
Replies with a list of pending actions (statements) if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| pending_statements | [PendingStatement](#treaty_proto-PendingStatement) | repeated | A list of pending statements to be executed. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to get the list of pending actions. |






<a name="treaty_proto-GetPendingActionsRequest"></a>

### GetPendingActionsRequest
Requests a list of pending actions at a Participant if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |
| action | [string](#string) |  | The type of action we are interested in (UPDATE or DELETE) |






<a name="treaty_proto-GetReadRowIdsReply"></a>

### GetReadRowIdsReply
Replies with a list of row ids for the request if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| row_ids | [uint32](#uint32) | repeated | The list of row ids. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to get the list of affected row ids. |






<a name="treaty_proto-GetReadRowIdsRequest"></a>

### GetReadRowIdsRequest
Requests the row ids for the specified WHERE clause if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |
| where_clause | [string](#string) |  | The WHERE clause. |






<a name="treaty_proto-GetRowFromPartialDatabaseRequest"></a>

### GetRowFromPartialDatabaseRequest
A request to get a specified row from a partial database if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| row_address | [RowParticipantAddress](#treaty_proto-RowParticipantAddress) |  | The row which to get. |
| Telemetry | [Telemetry](#treaty_proto-Telemetry) |  | Additional details for debugging purposes. |






<a name="treaty_proto-GetRowFromPartialDatabaseResult"></a>

### GetRowFromPartialDatabaseResult
A response containing the specified row requested, if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the request was successful. |
| result_message | [string](#string) |  | Any additional details if needed. This field can be blank. |
| row | [Row](#treaty_proto-Row) |  | The actual row requested. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to get the specified row. |






<a name="treaty_proto-GetSettingsReply"></a>

### GetSettingsReply
Responds with the current Settings if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| settings_json | [string](#string) | optional | The settings in JSON format. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to return the settings. |






<a name="treaty_proto-GetSettingsRequest"></a>

### GetSettingsRequest
Requests the current settings from Treaty if authenticated. These are the values from the &#34;Settings.toml&#34; file.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | Authentication. |






<a name="treaty_proto-GetUpdatesFromHostBehaviorReply"></a>

### GetUpdatesFromHostBehaviorReply
Responds with the current &#34;UpdatesFromHost&#34; behavior for the specified database and table if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| behavior | [uint32](#uint32) | optional | The current behavior for the requested database and table. This value is defined in the /treaty/treaty-types/enum.rs file. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to return the current behavior. |






<a name="treaty_proto-GetUpdatesFromHostBehaviorRequest"></a>

### GetUpdatesFromHostBehaviorRequest
Requests the current &#34;UpdatesFromHost&#34; behavior for the specified database and table if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |






<a name="treaty_proto-GetUpdatesToHostBehaviorReply"></a>

### GetUpdatesToHostBehaviorReply
Responds with the current &#34;UpdatesToHost&#34; behavior for the specified database and table if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| behavior | [uint32](#uint32) | optional | The current behavior for the for the requested database and table. This value is defined in the /treaty/treaty-types/enum.rs file. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to return the current behavior. |






<a name="treaty_proto-GetUpdatesToHostBehaviorRequest"></a>

### GetUpdatesToHostBehaviorRequest
Requests the current &#34;UpdatesToHost&#34; behavior for the specified database and table if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |






<a name="treaty_proto-HasTableReply"></a>

### HasTableReply
Replies if the specified table exists if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| has_table | [bool](#bool) |  | If the table exists or not. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was uanble to determine if the specified table exists or not. |






<a name="treaty_proto-HasTableRequest"></a>

### HasTableRequest
Requests to find out if the specified table exists if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |






<a name="treaty_proto-Host"></a>

### Host
A Host in Treaty.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| host_guid | [string](#string) |  | The public GUID/UUID the Host identifies itself with. |
| host_name | [string](#string) |  | A friendly name for the host. |
| token | [bytes](#bytes) |  | A token used for authentication. |
| network | [HostNetwork](#treaty_proto-HostNetwork) | optional | Network settings for the Host. |






<a name="treaty_proto-HostInfoReply"></a>

### HostInfoReply
Replies with the current Host Info at this Treaty instance if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| host_info | [Host](#treaty_proto-Host) | optional | The host information. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to return the current host information. |






<a name="treaty_proto-HostInfoStatus"></a>

### HostInfoStatus
A message describing the latest status of a Host.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| host | [Host](#treaty_proto-Host) |  | Host information. |
| last_communcation_utc | [string](#string) |  | The last time a message was seen from this host in UTC (RFC3339) |
| status | [uint32](#uint32) |  | The current HostStatus. This value is defined in the /treaty/treaty-types/enums.rs file. |






<a name="treaty_proto-HostNetwork"></a>

### HostNetwork
A Host&#39;s network settings.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ip4_address | [string](#string) | optional | The IP4 Address. |
| ip6_address | [string](#string) | optional | The IP6 address. |
| database_port_number | [uint32](#uint32) | optional | The database port number. |
| http_addr | [string](#string) | optional | The HTTP address. |
| http_port | [uint32](#uint32) | optional | The HTTP port. |






<a name="treaty_proto-InsertDataRequest"></a>

### InsertDataRequest
A request for Treaty to execute the specified INSERT statement if authenticated. 
❗ Warning: At the moment, Treaty can only handle simple INSERT statements for a single row. 
For more information, see the manual.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |
| cmd | [string](#string) |  | The actual INSERT statement. Note: while this is duplicative, at the moment the contents of this INSERT statement must match the database and table name. |






<a name="treaty_proto-InsertDataResult"></a>

### InsertDataResult
A result of executing an INSERT statement against a partial database if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the result was successful. |
| data_hash | [uint64](#uint64) |  | A hash of the data inserted. |
| message | [string](#string) |  | An additional message if needed. This field can be blank. |
| row_id | [uint32](#uint32) |  | The row id of the record inserted. |
| is_error | [bool](#bool) |  | If there was an error executing the INSERT statement. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error detailing if the request failed. |






<a name="treaty_proto-NotifyHostOfRemovedRowRequest"></a>

### NotifyHostOfRemovedRowRequest
Request to notify the upstream Host that a row has been deleted.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| Telemetry | [Telemetry](#treaty_proto-Telemetry) |  | Debugging information about the sender of this message. |
| host_info | [Host](#treaty_proto-Host) |  | The host information. From an upstream Host&#39;s perspective, this is the Participant. |
| database_name | [string](#string) |  | The database name. |
| database_id | [string](#string) |  | The database id. |
| table_name | [string](#string) |  | The table name. |
| table_id | [uint32](#uint32) |  | The table id. |
| row_id | [uint32](#uint32) |  | The row id. |






<a name="treaty_proto-NotifyHostOfRemovedRowResult"></a>

### NotifyHostOfRemovedRowResult
The result of notifying the upstream Host that a row has been deleted.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the notification was successful. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was not able to notify the upstream Host. |






<a name="treaty_proto-Participant"></a>

### Participant
A message representing information about a Participant in the system.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| participant_guid | [string](#string) |  | The public GUID/UUID that a Participant identifies itself with. |
| alias | [string](#string) |  | A friendly alias. |
| ip4_address | [string](#string) |  | The IP4 address. |
| ip6_address | [string](#string) |  | The IP6 address. |
| database_port_number | [uint32](#uint32) |  | The database port number. |
| token | [bytes](#bytes) |  | A token used for authentication. |
| internal_participant_guid | [string](#string) |  | An internal generated GUID/UUID for the Participant. |
| http_addr | [string](#string) |  | The HTTP address. |
| http_port | [uint32](#uint32) |  | The HTTP port number. |






<a name="treaty_proto-ParticipantAcceptsContractRequest"></a>

### ParticipantAcceptsContractRequest
Request to accept a particular contract.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| participant | [Participant](#treaty_proto-Participant) |  | The participant accepting the contract. This is used as a way to identify the Participant. |
| contract_guid | [string](#string) |  | The GUID/UUID of the contract. |
| contract_version_guid | [string](#string) |  | The GUID/UUID version of the contract. Contracts can be updated, and so with each change of a contract the version must be changed. |
| database_name | [string](#string) |  | The database name. |
| Telemetry | [Telemetry](#treaty_proto-Telemetry) |  | Any additional debugging information. |
| id | [string](#string) | optional | The host id of the targeted Treaty instance. This is usually used if `treaty` is being hosted by a `treaty-proxy` instance. |






<a name="treaty_proto-ParticipantAcceptsContractResult"></a>

### ParticipantAcceptsContractResult
Describes the result of notifying that a Participant has accepted a contract.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| contract_acceptance_is_acknowledged | [bool](#bool) |  | If the result is acknowledged. |
| is_error | [bool](#bool) |  | If there was an error notifying acceptance of the contract. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error describing details for the action, if appliable. |






<a name="treaty_proto-ParticipantStatus"></a>

### ParticipantStatus
The status of a Participant at a Host.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| participant | [Participant](#treaty_proto-Participant) |  | The participant details. |
| contract_status | [uint32](#uint32) |  | The contract status. This value is defined in the /treaty/treaty-types/enums.rs file. |






<a name="treaty_proto-PendingStatement"></a>

### PendingStatement
A statement that is queued to be executed at a Treaty instance.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| row_id | [uint32](#uint32) |  | The row id being affected |
| statement | [string](#string) |  | The UPDATE or DELETE statement |
| requested_ts_utc | [string](#string) |  | The time in UTC the request was made |
| host_id | [string](#string) |  | The host id requesting the action |
| action | [string](#string) |  | The actual SQL statement being executed |






<a name="treaty_proto-RejectPendingContractReply"></a>

### RejectPendingContractReply
Replies with the result of rejecting a pending contract if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the rejection was successful. |
| message | [string](#string) |  | A message with any additional information. This field may be blank. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to reject the pending contract. |






<a name="treaty_proto-RejectPendingContractRequest"></a>

### RejectPendingContractRequest
Requests that Treaty reject the pending contract from the specified host, if authenticated. 
This sends a message back to the Host that we are not interested in this contract. No database changes are made. 
For more information, see the manual.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| host_alias | [string](#string) |  | The alias of the host. |






<a name="treaty_proto-RevokeReply"></a>

### RevokeReply
Replies with the result of attempting to revoke the current Json Web Token.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| is_successful | [bool](#bool) |  |  |






<a name="treaty_proto-Row"></a>

### Row
An object for representing a row in a table. Used for returning data.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |
| row_id | [uint32](#uint32) |  | The row id. |
| values | [RowValue](#treaty_proto-RowValue) | repeated | A list of values held by the row. |
| is_remoteable | [bool](#bool) |  | Deprecated. This will be deleted. |
| remote_metadata | [RowRemoteMetadata](#treaty_proto-RowRemoteMetadata) |  | A description about the row such as if the data is out of sync between a Host and a Participant. |
| hash | [bytes](#bytes) |  | A hash of the row&#39;s data. |






<a name="treaty_proto-RowInfo"></a>

### RowInfo
A message describing the details of a row in a partial database.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| database_name | [string](#string) |  | The name of the database the row is in. |
| table_name | [string](#string) |  | The table name the row is in. |
| row_id | [uint32](#uint32) |  | The row id. |
| data_hash | [uint64](#uint64) |  | The data hash of the row. |






<a name="treaty_proto-RowParticipantAddress"></a>

### RowParticipantAddress
A message for identifying the location of a row in a partial database.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |
| row_id | [uint32](#uint32) |  | The row id. |






<a name="treaty_proto-RowRemoteMetadata"></a>

### RowRemoteMetadata
Describes the data status of the host in relation to the Participant.
Example the data hash between the host and the participant do not match
or if the row was deleted at the participant, but the reference at the host is not.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| is_remote_out_of_sync_with_host | [bool](#bool) |  |  |
| is_hash_out_of_sync_with_host | [bool](#bool) |  |  |
| is_remote_deleted | [bool](#bool) |  |  |
| is_local_deleted | [bool](#bool) |  |  |






<a name="treaty_proto-RowValue"></a>

### RowValue
An object for storing values for a row in a table. Used for returning data.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| column | [ColumnSchema](#treaty_proto-ColumnSchema) |  | The column of the value. |
| is_null_value | [bool](#bool) |  | If the value is NULL. |
| value | [bytes](#bytes) |  | The binary value. |
| string_value | [string](#string) |  | A string representation of the value. |






<a name="treaty_proto-SaveContractRequest"></a>

### SaveContractRequest
A message from a host to a participant to save a contract.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| contract | [Contract](#treaty_proto-Contract) |  | A contract to save. |
| Telemetry | [Telemetry](#treaty_proto-Telemetry) |  | Any additional debugging details. |
| id | [string](#string) | optional | The host id of the targeted Treaty instance. This is usually used if `treaty` is being hosted by a `treaty-proxy` instance. |






<a name="treaty_proto-SaveContractResult"></a>

### SaveContractResult
A message describing the results of saving a contract.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| is_saved | [bool](#bool) |  | If the contract was saved. |
| contract_status | [uint32](#uint32) |  | A message confirming the Participant&#39;s status of the contract (Accepted/Rejected/Pending) |
| participant_info | [Participant](#treaty_proto-Participant) | optional | If the Participant wishes to confirm their information back to the Host. This is useful If the Host and the Participant are out of sync with the contract status. |
| is_error | [bool](#bool) |  | If there was an error saving the contract. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | Any details if Treaty was unable to save the contract. |






<a name="treaty_proto-SendParticipantContractReply"></a>

### SendParticipantContractReply
Replies with the result of sending the active contract to the participant if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_sent | [bool](#bool) |  | If the contract was sent. |
| contract_status | [uint32](#uint32) |  | The current status of the contract at the participant. This is an echo of what the Participant thinks the contract status is. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to send the active contract to the Participant. |






<a name="treaty_proto-SendParticipantContractRequest"></a>

### SendParticipantContractRequest
Requests to send the active database contract ot the specified participant if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The name of the database. |
| participant_alias | [string](#string) |  | The alias of the participant. |






<a name="treaty_proto-SetDataLogTableStatusReply"></a>

### SetDataLogTableStatusReply
Replies with the result of configuring a data log if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the request was successful or not. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to set the requested status of data logging. |






<a name="treaty_proto-SetDataLogTableStatusRequest"></a>

### SetDataLogTableStatusRequest
Requests that a data log be enabled for the specified table if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |
| use_data_log | [bool](#bool) |  | If a data log should be enabled. |






<a name="treaty_proto-SetLogicalStoragePolicyReply"></a>

### SetLogicalStoragePolicyReply
Replies with the result of setting the Logical Storage Policy for the specified table if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the request was successful. |
| message | [string](#string) |  | A message providing any additional information. This value can be empty. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to generate the contract. |






<a name="treaty_proto-SetLogicalStoragePolicyRequest"></a>

### SetLogicalStoragePolicyRequest
Requests Treaty to set the specified Logical Storage Policy for the specified table if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |
| policy_mode | [uint32](#uint32) |  | The policy to set the table to. This value is defined in the /treaty/treaty-types/enums.rs file. |






<a name="treaty_proto-StatementResultset"></a>

### StatementResultset
A message representing the results of a SQL query.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| number_of_rows_affected | [uint64](#uint64) |  | The total number of rows affected, if applicable. |
| rows | [Row](#treaty_proto-Row) | repeated | A list of Row items. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to provide results. |
| warning | [TreatyWarning](#treaty_proto-TreatyWarning) | optional | A warning if there is a data mis-match. |






<a name="treaty_proto-TableSchema"></a>

### TableSchema
A message for describing the schema information of a table in a database.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| table_name | [string](#string) |  | The table name. |
| table_id | [string](#string) |  | The table id. |
| database_name | [string](#string) |  | The database name this table belongs to. |
| databaseId | [string](#string) |  | The database id this table belongs to. |
| columns | [ColumnSchema](#treaty_proto-ColumnSchema) | repeated | The columns of the table. |
| logical_storage_policy | [uint32](#uint32) |  | The Logical Storage Policy for this table. This value is defined in the /treaty/treaty-types/enums.rs file. For more information, see the manual. |






<a name="treaty_proto-Telemetry"></a>

### Telemetry
A message for general information.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| is_little_endian | [bool](#bool) |  | Endian-ness of the Treaty instance. |
| message_addresses | [string](#string) | repeated | A list of IP addresses for this sender. |
| message_generated_time_utc | [string](#string) |  | The time the message was generated in UTC (RFC3339) |
| message_guid | [string](#string) |  | A unique ID for this message. |






<a name="treaty_proto-TestReply"></a>

### TestReply
A message for basic online testing.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| reply_time_utc | [string](#string) |  | The time the reply was generated in UTC (RFC3339) |
| reply_echo_message | [string](#string) |  | The message to echo back. |
| treaty_version | [string](#string) |  | The sender&#39;s Treaty version. |






<a name="treaty_proto-TestRequest"></a>

### TestRequest
A message for basic online testing.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| request_time_utc | [string](#string) |  | The time the request was sent in UTC (RFC3339) |
| request_origin_url | [string](#string) |  | The origin URL, if applicable. |
| request_origin_ip4 | [string](#string) |  | The origin IP4 address. |
| request_origin_ip6 | [string](#string) |  | The oring IP6 address. |
| request_port_number | [uint32](#uint32) |  | The origin port number. |
| request_echo_message | [string](#string) |  | A test message that should be echo&#39;d back. |






<a name="treaty_proto-TokenReply"></a>

### TokenReply
Replies with an issued Json Web Token if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| is_successful | [bool](#bool) |  |  |
| expiration_utc | [string](#string) |  |  |
| jwt | [string](#string) |  |  |






<a name="treaty_proto-TransactionInfo"></a>

### TransactionInfo
🗑️ This should not be used and will be deleted in the future.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transaction_batch_id | [string](#string) |  |  |
| transaction_mode | [uint32](#uint32) |  |  |






<a name="treaty_proto-TreatyError"></a>

### TreatyError
A message describing an error in the system.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| message | [string](#string) |  | A description of what went wrong. |
| help | [string](#string) | optional | An optional description of how to fix the error. |
| number | [uint32](#uint32) |  | Not used: A numerical value tied to the error. |






<a name="treaty_proto-TreatyLogEntry"></a>

### TreatyLogEntry
A log entry within Treaty.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dt | [string](#string) |  | The local datetime of the log entry. |
| dt_utc | [string](#string) |  | The UTC datetime of the log entry. |
| level | [string](#string) |  | The logging level. In order of severity: Error, Warn, Info, Debug, Trace. |
| message | [string](#string) |  | The actual log message. |






<a name="treaty_proto-TreatyWarning"></a>

### TreatyWarning
A message describing a potential problem in the system.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| message | [string](#string) |  | A description of a problem. |
| help | [string](#string) | optional | An optional description of how to fix the error. |
| number | [uint32](#uint32) |  | Not used: A numerical value tied to the error. |






<a name="treaty_proto-TryAuthAtParticipantRequest"></a>

### TryAuthAtParticipantRequest
A request to attempt to authenticate at the specified Participant. This tests to make sure that we 
have not been rejected by the specified participant.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| participant_id | [string](#string) |  | The participant id. |
| participant_alias | [string](#string) |  | The participant alias. |
| db_name | [string](#string) |  | The database name. |






<a name="treaty_proto-TryAuthAtPartipantReply"></a>

### TryAuthAtPartipantReply
Replies with the result of attempting to autenticate at the specified Participant.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the result was successful. |
| message | [string](#string) |  | A message with any additional information. This value can be empty. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to attempt authentication. |






<a name="treaty_proto-TryAuthRequest"></a>

### TryAuthRequest
A request to validate that we have access to this Treaty instance.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |






<a name="treaty_proto-TryAuthResult"></a>

### TryAuthResult
Replies with the authentication result.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |






<a name="treaty_proto-UpdateDataRequest"></a>

### UpdateDataRequest
A request for Treaty to execute the specified UPDATE statement if authentiated. 
❗ Warning: At the moment, Treaty can only handle simple UPDATE statements. 
For more information, see the manual.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| database_name | [string](#string) |  | The database name. |
| table_name | [string](#string) |  | The table name. |
| cmd | [string](#string) |  | The actual UPDATE statement. |
| where_clause | [string](#string) |  |  |






<a name="treaty_proto-UpdateDataResult"></a>

### UpdateDataResult
Replies with the result of executing the provided UPDATE statement if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the UPDATE statement was successful. |
| message | [string](#string) |  | A message describing any additional details. This field can be blank. |
| rows | [RowInfo](#treaty_proto-RowInfo) | repeated | A copy of the rows that were affected. |
| update_status | [uint32](#uint32) |  | The status of the actual update. Values are: 0 - unknown 1 - success (overwrite or overwrite with log) 2 - pending (queue for review) 3 - ignored (ignore) Note: These values are defined in the /treaty/treaty-types/enums.rs file. |
| is_error | [bool](#bool) |  | If there was an error executing the UPDATE statement. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | Any details if there was an error executing the UPDATE statement. |






<a name="treaty_proto-UpdateRowDataHashForHostRequest"></a>

### UpdateRowDataHashForHostRequest
Request from a Participant to a Host to update the data hash.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |
| Telemetry | [Telemetry](#treaty_proto-Telemetry) |  | Additional telemetry for debugging. |
| host_info | [Host](#treaty_proto-Host) |  | The host information this data hash came from (from the perspective of the Host, this is the Participant&#39;s information). |
| database_name | [string](#string) |  | The database name. |
| database_id | [string](#string) |  | The database id. |
| table_name | [string](#string) |  | The table name. |
| table_id | [uint32](#uint32) |  | The table id. |
| row_id | [uint32](#uint32) |  | The row id. |
| updated_hash_value | [uint64](#uint64) |  | The new hash value for the row. |
| is_deleted_at_participant | [bool](#bool) |  | If the row is deleted. |






<a name="treaty_proto-UpdateRowDataHashForHostResult"></a>

### UpdateRowDataHashForHostResult
Replies with the result of the update data hash request.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| is_successful | [bool](#bool) |  | If the message was successful. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if the updated data hash could not be sent. |






<a name="treaty_proto-VersionReply"></a>

### VersionReply
Replies with the current version of Treaty at this instance, if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| versions | [Versions](#treaty_proto-Versions) | optional | The version of Treaty. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to return the current version numbers. |






<a name="treaty_proto-Versions"></a>

### Versions
The version of Treaty.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| treaty | [string](#string) | optional |  |






<a name="treaty_proto-ViewPendingContractsReply"></a>

### ViewPendingContractsReply
Replies with a list of pending contracts if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication_result | [AuthResult](#treaty_proto-AuthResult) |  | The authentication result. |
| contracts | [Contract](#treaty_proto-Contract) | repeated | A list of contracts that are in a pending state. This list may be empty. |
| error | [TreatyError](#treaty_proto-TreatyError) | optional | An error if Treaty was unable to get the list of pending contracts. |






<a name="treaty_proto-ViewPendingContractsRequest"></a>

### ViewPendingContractsRequest
Requests to view a list of all contracts Treaty has that are in a Pending state, if authenticated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authentication | [AuthRequest](#treaty_proto-AuthRequest) |  | The authentication request. |





 

 

 


<a name="treaty_proto-DataService"></a>

### DataService
A service that a Treaty instance can talk to other Treaty instances.
Generally defaults to port 50052. See the &#34;Settings.toml&#34; file for configuration.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| IsOnline | [TestRequest](#treaty_proto-TestRequest) | [TestReply](#treaty_proto-TestReply) | A call to see if the service is available. |
| CreatePartialDatabase | [CreatePartialDatabaseRequest](#treaty_proto-CreatePartialDatabaseRequest) | [CreatePartialDatabaseResult](#treaty_proto-CreatePartialDatabaseResult) | Creates a partial database. |
| CreateTableInDatabase | [CreateTableRequest](#treaty_proto-CreateTableRequest) | [CreateTableResult](#treaty_proto-CreateTableResult) | Creates a table in a partial database. |
| InsertCommandIntoTable | [InsertDataRequest](#treaty_proto-InsertDataRequest) | [InsertDataResult](#treaty_proto-InsertDataResult) | Executes the provided INSERT statement against a partial database. |
| UpdateCommandIntoTable | [UpdateDataRequest](#treaty_proto-UpdateDataRequest) | [UpdateDataResult](#treaty_proto-UpdateDataResult) | Executes the provided UPDATE statement against a partial database. |
| DeleteCommandIntoTable | [DeleteDataRequest](#treaty_proto-DeleteDataRequest) | [DeleteDataResult](#treaty_proto-DeleteDataResult) | Executes the provided DELETE statement againts a partial database. |
| GetRowFromPartialDatabase | [GetRowFromPartialDatabaseRequest](#treaty_proto-GetRowFromPartialDatabaseRequest) | [GetRowFromPartialDatabaseResult](#treaty_proto-GetRowFromPartialDatabaseResult) | Requests a specific row from a partial database. |
| SaveContract | [SaveContractRequest](#treaty_proto-SaveContractRequest) | [SaveContractResult](#treaty_proto-SaveContractResult) | Request to save a Contract; usually to be later Accepted or Rejected. |
| AcceptContract | [ParticipantAcceptsContractRequest](#treaty_proto-ParticipantAcceptsContractRequest) | [ParticipantAcceptsContractResult](#treaty_proto-ParticipantAcceptsContractResult) | Notification that a Participant has accepted a contract. |
| UpdateRowDataHashForHost | [UpdateRowDataHashForHostRequest](#treaty_proto-UpdateRowDataHashForHostRequest) | [UpdateRowDataHashForHostResult](#treaty_proto-UpdateRowDataHashForHostResult) | Notification that a data hash has changed at a Participant. |
| NotifyHostOfRemovedRow | [NotifyHostOfRemovedRowRequest](#treaty_proto-NotifyHostOfRemovedRowRequest) | [NotifyHostOfRemovedRowResult](#treaty_proto-NotifyHostOfRemovedRowResult) | Notification that a row in a partial database has been removed at a Participant. |
| TryAuth | [TryAuthRequest](#treaty_proto-TryAuthRequest) | [TryAuthResult](#treaty_proto-TryAuthResult) | Check if we can authenticate at this Treaty instance. |


<a name="treaty_proto-UserService"></a>

### UserService
A service by which application developers can talk to a Treaty instance. 
Generally defaults to port 50051. See the &#34;Settings.toml&#34; file for configuration.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| IsOnline | [TestRequest](#treaty_proto-TestRequest) | [TestReply](#treaty_proto-TestReply) | Denotes if the instance is online. |
| CreateUserDatabase | [CreateUserDatabaseRequest](#treaty_proto-CreateUserDatabaseRequest) | [CreateUserDatabaseReply](#treaty_proto-CreateUserDatabaseReply) | Creates a database. |
| EnableCoooperativeFeatures | [EnableCoooperativeFeaturesRequest](#treaty_proto-EnableCoooperativeFeaturesRequest) | [EnableCoooperativeFeaturesReply](#treaty_proto-EnableCoooperativeFeaturesReply) | Instructs Treaty to create needed meta-data tables. |
| ExecuteReadAtHost | [ExecuteReadRequest](#treaty_proto-ExecuteReadRequest) | [ExecuteReadReply](#treaty_proto-ExecuteReadReply) | Executes the specified SELECT SQL query against a Host database. |
| ExecuteWriteAtHost | [ExecuteWriteRequest](#treaty_proto-ExecuteWriteRequest) | [ExecuteWriteReply](#treaty_proto-ExecuteWriteReply) | Executes the specified INSERT/UPDATE/DELETE SQL statement against a Host database. |
| ExecuteCooperativeWriteAtHost | [ExecuteCooperativeWriteRequest](#treaty_proto-ExecuteCooperativeWriteRequest) | [ExecuteCooperativeWriteReply](#treaty_proto-ExecuteCooperativeWriteReply) | Executes the specified INSERT/UPDATE/DELETE SQL statement at the Participant and saves the meta-data at the Host. |
| ExecuteReadAtParticipant | [ExecuteReadRequest](#treaty_proto-ExecuteReadRequest) | [ExecuteReadReply](#treaty_proto-ExecuteReadReply) | Executes the specified SELECT SQL query against a Partial database. |
| ExecuteWriteAtParticipant | [ExecuteWriteRequest](#treaty_proto-ExecuteWriteRequest) | [ExecuteWriteReply](#treaty_proto-ExecuteWriteReply) | Executes the specified INSERT/UPDATE/DELETE SQL statment against a Partial database. |
| HasTable | [HasTableRequest](#treaty_proto-HasTableRequest) | [HasTableReply](#treaty_proto-HasTableReply) | Checks if the specified table exists in the specified database. |
| SetLogicalStoragePolicy | [SetLogicalStoragePolicyRequest](#treaty_proto-SetLogicalStoragePolicyRequest) | [SetLogicalStoragePolicyReply](#treaty_proto-SetLogicalStoragePolicyReply) | Sets the Logical Storage Policy for the specified table in the specified database. |
| GetLogicalStoragePolicy | [GetLogicalStoragePolicyRequest](#treaty_proto-GetLogicalStoragePolicyRequest) | [GetLogicalStoragePolicyReply](#treaty_proto-GetLogicalStoragePolicyReply) | Gets the Logical Storage Policy for the specified table in the specified database. |
| GenerateContract | [GenerateContractRequest](#treaty_proto-GenerateContractRequest) | [GenerateContractReply](#treaty_proto-GenerateContractReply) | Generates a database contract for the specified database. ℹ️ INFORMATION: For this to work, you must set a Logical Storage Policy ahead of time on all database tables. See the manual for more information. |
| AddParticipant | [AddParticipantRequest](#treaty_proto-AddParticipantRequest) | [AddParticipantReply](#treaty_proto-AddParticipantReply) | Adds a participant with the specified attributes to the specified database. |
| SendParticipantContract | [SendParticipantContractRequest](#treaty_proto-SendParticipantContractRequest) | [SendParticipantContractReply](#treaty_proto-SendParticipantContractReply) | Sends a copy of the active database contract to the specified Participant. |
| ReviewPendingContracts | [ViewPendingContractsRequest](#treaty_proto-ViewPendingContractsRequest) | [ViewPendingContractsReply](#treaty_proto-ViewPendingContractsReply) | Gets a list of pending contracts at our Treaty instance. |
| AcceptPendingContract | [AcceptPendingContractRequest](#treaty_proto-AcceptPendingContractRequest) | [AcceptPendingContractReply](#treaty_proto-AcceptPendingContractReply) | Accepts the specified database contract. This creates the needed partial database and supporting database structures. |
| RejectPendingContract | [RejectPendingContractRequest](#treaty_proto-RejectPendingContractRequest) | [RejectPendingContractReply](#treaty_proto-RejectPendingContractReply) | Rejects the specified database contract. This informs the Host that we do not agree to cooperate. |
| GenerateHostInfo | [GenerateHostInfoRequest](#treaty_proto-GenerateHostInfoRequest) | [GenerateHostInfoReply](#treaty_proto-GenerateHostInfoReply) | Generates our host info with the specified host name. ❗ WARNING: Calling this may overwrite any existing authentication token you have used to identify your Treaty instance to others. See the manual for more information. |
| ChangeHostStatus | [ChangeHostStatusRequest](#treaty_proto-ChangeHostStatusRequest) | [ChangeHostStatusReply](#treaty_proto-ChangeHostStatusReply) | Change the status for the specified Host. This configures if a Host is allowed to talk to our Treaty instance. |
| TryAuthAtParticipant | [TryAuthAtParticipantRequest](#treaty_proto-TryAuthAtParticipantRequest) | [TryAuthAtPartipantReply](#treaty_proto-TryAuthAtPartipantReply) | Attempt authentication at the specified host. |
| ChangeUpdatesFromHostBehavior | [ChangeUpdatesFromHostBehaviorRequest](#treaty_proto-ChangeUpdatesFromHostBehaviorRequest) | [ChangesUpdatesFromHostBehaviorReply](#treaty_proto-ChangesUpdatesFromHostBehaviorReply) | Changes the UpdatesFromHost behavior. |
| ChangeDeletesFromHostBehavior | [ChangeDeletesFromHostBehaviorRequest](#treaty_proto-ChangeDeletesFromHostBehaviorRequest) | [ChangeDeletesFromHostBehaviorReply](#treaty_proto-ChangeDeletesFromHostBehaviorReply) | Changes the DeletesFromHost behavior. |
| ChangeUpdatesToHostBehavior | [ChangeUpdatesToHostBehaviorRequest](#treaty_proto-ChangeUpdatesToHostBehaviorRequest) | [ChangeUpdatesToHostBehaviorReply](#treaty_proto-ChangeUpdatesToHostBehaviorReply) | Changes the UpdatesToHost behavior. |
| ChangeDeletesToHostBehavior | [ChangeDeletesToHostBehaviorRequest](#treaty_proto-ChangeDeletesToHostBehaviorRequest) | [ChangeDeletesToHostBehaviorReply](#treaty_proto-ChangeDeletesToHostBehaviorReply) | Changes the DeletesToHost behavior. |
| GetDataHashAtHost | [GetDataHashRequest](#treaty_proto-GetDataHashRequest) | [GetDataHashReply](#treaty_proto-GetDataHashReply) | Gets the data hash at the specified Host database for the specified row. |
| GetDataHashAtParticipant | [GetDataHashRequest](#treaty_proto-GetDataHashRequest) | [GetDataHashReply](#treaty_proto-GetDataHashReply) | Gets the data hash at the specified Partial database for the specified row. |
| ReadRowIdAtParticipant | [GetReadRowIdsRequest](#treaty_proto-GetReadRowIdsRequest) | [GetReadRowIdsReply](#treaty_proto-GetReadRowIdsReply) | Gets the Row ID at the specified Partial database for the specified WHERE clause. |
| GetDataLogTableStatusAtParticipant | [GetDataLogTableStatusRequest](#treaty_proto-GetDataLogTableStatusRequest) | [GetDataLogTableStatusReply](#treaty_proto-GetDataLogTableStatusReply) | Gets the status of our Log table at the Partial database. |
| SetDataLogTableStatusAtParticipant | [SetDataLogTableStatusRequest](#treaty_proto-SetDataLogTableStatusRequest) | [SetDataLogTableStatusReply](#treaty_proto-SetDataLogTableStatusReply) | Sets the status of our Log table at the Partial database. |
| GetPendingActionsAtParticipant | [GetPendingActionsRequest](#treaty_proto-GetPendingActionsRequest) | [GetPendingActionsReply](#treaty_proto-GetPendingActionsReply) | Gets a list of pending actions at the Partial database. |
| AcceptPendingActionAtParticipant | [AcceptPendingActionRequest](#treaty_proto-AcceptPendingActionRequest) | [AcceptPendingActionReply](#treaty_proto-AcceptPendingActionReply) | Accepts the pending database action at the Partial database. |
| GetDatabases | [GetDatabasesRequest](#treaty_proto-GetDatabasesRequest) | [GetDatabasesReply](#treaty_proto-GetDatabasesReply) | Gets a list of databases at our Treaty instance. |
| GetParticipants | [GetParticipantsRequest](#treaty_proto-GetParticipantsRequest) | [GetParticipantsReply](#treaty_proto-GetParticipantsReply) | Gets a list of Participants at our Treaty instance. |
| GetActiveContract | [GetActiveContractRequest](#treaty_proto-GetActiveContractRequest) | [GetActiveContractReply](#treaty_proto-GetActiveContractReply) | Gets the active database contract for the specified database. |
| AuthForToken | [AuthRequest](#treaty_proto-AuthRequest) | [TokenReply](#treaty_proto-TokenReply) | Requests Treaty to generate a Json Web Token for the credentials provided. |
| RevokeToken | [AuthRequest](#treaty_proto-AuthRequest) | [RevokeReply](#treaty_proto-RevokeReply) | Requests Treaty to revoke the Json Web Token for the credentials provided. |
| GetHostInfo | [AuthRequest](#treaty_proto-AuthRequest) | [HostInfoReply](#treaty_proto-HostInfoReply) | Gets our Host Information. |
| GetVersions | [AuthRequest](#treaty_proto-AuthRequest) | [VersionReply](#treaty_proto-VersionReply) | Gets the versions of Treaty assemblies. |
| GetUpdatesFromHostBehavior | [GetUpdatesFromHostBehaviorRequest](#treaty_proto-GetUpdatesFromHostBehaviorRequest) | [GetUpdatesFromHostBehaviorReply](#treaty_proto-GetUpdatesFromHostBehaviorReply) | Gets the current configured UpdatesFromHostBehavior. |
| GetUpdatesToHostBehavior | [GetUpdatesToHostBehaviorRequest](#treaty_proto-GetUpdatesToHostBehaviorRequest) | [GetUpdatesToHostBehaviorReply](#treaty_proto-GetUpdatesToHostBehaviorReply) | Gets the current configured UpdatesToHostBehavior. |
| GetDeletesFromHostBehavior | [GetDeletesFromHostBehaviorRequest](#treaty_proto-GetDeletesFromHostBehaviorRequest) | [GetDeletesFromHostBehaviorReply](#treaty_proto-GetDeletesFromHostBehaviorReply) | Gets the current configured DeletesFromHostBehavior. |
| GetDeletesToHostBehavior | [GetDeletesToHostBehaviorRequest](#treaty_proto-GetDeletesToHostBehaviorRequest) | [GetDeletesToHostBehaviorReply](#treaty_proto-GetDeletesToHostBehaviorReply) | Gets the current configured DeletesToHostBehavior. |
| GetCooperativeHosts | [GetCooperativeHostsRequest](#treaty_proto-GetCooperativeHostsRequest) | [GetCooperativeHostsReply](#treaty_proto-GetCooperativeHostsReply) | Gets a list of Hosts that we are cooperating with. These are all the Treaty instances that we have accepted contracts from. |
| GetSettings | [GetSettingsRequest](#treaty_proto-GetSettingsRequest) | [GetSettingsReply](#treaty_proto-GetSettingsReply) | Gets the current configured settings from the Settings.toml file. |
| GetLogsByLastNumber | [GetLogsByLastNumberRequest](#treaty_proto-GetLogsByLastNumberRequest) | [GetLogsByLastNumberReply](#treaty_proto-GetLogsByLastNumberReply) | Gets the last X number of log entries. |

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

