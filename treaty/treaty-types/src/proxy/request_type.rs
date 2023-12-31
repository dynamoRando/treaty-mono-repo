use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;

#[derive(IntoPrimitive, Debug, Clone, TryFromPrimitive)]
#[repr(u16)]
pub enum RequestType {
    Unknown,
    Auth,
    CreateUserDatabase,
    EnableCooperativeFeatures,
    ReadAtHost,
    ReadAtPart,
    WriteAtHost,
    WriteAtPart,
    CooperativeWriteAtHost,
    HasTable,
    SetLogicalStoragePolicy,
    GetLogicalStoragePolicy,
    GenerateContract,
    AddParticipant,
    SendParticipantContract,
    ViewPendingContracts,
    AcceptPendingContract,
    RejectPendingContract,
    GenerateHostInfo,
    ViewHostInfo,
    ChangeHostStatus,
    TryAuthAtParticipant,
    ChangeUpdatesFromHostBehavior,
    ChangeDeletesFromHostBehavior,
    ChangeUpdatesToHostBehavior,
    ChangeDeletesToHostBehavior,
    GetDataHashAtHost,
    GetDataHashAtPart,
    GetReadRowIds,
    GetDataLogTableStatus,
    SetDataLogTableStatus,
    GetPendingActions,
    AcceptPendingAction,
    GetDatabases,
    GetParticipants,
    GetActiveContract,
    GetUpdatesFromHostBehavior,
    GetUpdatesToHostBehavior,
    GetDeletesFromHostBehavior,
    GetDeletesToHostBehavior,
    GetCooperativeHosts,
    GetSettings,
    GetLogsByLastNumber,
}
