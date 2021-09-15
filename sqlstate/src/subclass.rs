use sqlstate_macros::subclass;

#[subclass]
pub enum Warning {
    #[state("001")]
    CursorOperationConflict,
    #[state("002")]
    DisconnectError,
    #[state("003")]
    NullValueEliminatedInSetFunction,
    #[state("004")]
    StringDataRightTruncation,
    #[state("005")]
    InsufficientItemDescriptorAreas,
    #[state("006")]
    PrivilegeNotRevoked,
    #[state("007")]
    PrivilegeNotGranted,
    #[state("009")]
    SearchConditionTooLongForInformationSchema,
    #[state("00A")]
    QueryExpressionTooLongForInformationSchema,
    #[state("00B")]
    DefaultValueTooLongForInformationSchema,
    #[state("00C")]
    ResultSetsReturned,
    #[state("00D")]
    AdditionalResultSetsReturned,
    #[state("00E")]
    AttemptToReturnTooManyResultSets,
    #[state("00F")]
    StatementTooLongForInformationSchema,
    #[state("010")]
    ColumnCannotBeMapped,
    #[state("011")]
    SqlJavaPathTooLongForInformationSchema,
    #[state("012")]
    InvalidNumberOfConditions,
    #[state("02F")]
    ArrayDataRightTruncation,
}

#[subclass]
pub enum NoData {
    #[state("001")]
    NoAdditionalResultSetsReturned,
}

#[subclass]
pub enum DynamicSqlError {
    #[state("001")]
    UsingClauseDoesNotMatchDynamicParameterSpecifications,
    #[state("002")]
    UsingClauseDoesNotMatchTargetSpecifications,
    #[state("003")]
    CursorSpecificationCannotBeExecuted,
    #[state("004")]
    UsingClauseRequiredForDynamicParameters,
    #[state("005")]
    PreparedStatementNotACursorSpecification,
    #[state("006")]
    RestrictedDataTypeAttributeViolation,
    #[state("007")]
    UsingClauseRequiredForResultFields,
    #[state("008")]
    InvalidDescriptorCount,
    #[state("009")]
    InvalidDescriptorIndex,
    #[state("00B")]
    DataTypeTransformFunctionViolation,
    #[state("00C")]
    UndefinedDataValue,
    #[state("00D")]
    InvalidDataTarget,
    #[state("00E")]
    InvalidLevelValue,
    #[state("00F")]
    InvalidDatetimeIntervalCode,
}

#[subclass]
pub enum ConnectionException {
    #[state("001")]
    SqlClientUnableToEstablishSqlConnection,
    #[state("002")]
    ConnectionNameInUse,
    #[state("003")]
    ConnectionDoesNotExist,
    #[state("004")]
    SqlServerRejectedEstablishmentOfSqlConnection,
    #[state("006")]
    ConnectionFailure,
    #[state("007")]
    TransactionResolutionUnknown,
}

#[subclass]
pub enum FeatureNotSupported {
    #[state("001")]
    MultipleServerTransactions,
}

#[subclass]
pub enum LocatorException {
    #[state("001")]
    InvalidSpecification,
}

#[subclass]
pub enum SqlXmlMappingError {
    #[state("001")]
    UnmappableXmlName,
    #[state("002")]
    InvalidXmlCharacter,
}

#[subclass]
pub enum ProhibitedStatementDuringTriggerExecution {
    #[state("001")]
    ModifyTableModifiedByDataChangeDeltaTable,
}

#[subclass]
pub enum PassthroughSpecificCondition {
    #[state("001")]
    InvalidCursorOption,
    #[state("002")]
    InvalidCursorAllocation,
}

#[subclass]
pub enum DiagnosticsException {
    #[state("001")]
    MaximumNumberOfStackedDiagnosticsAreasExceeded,
    #[state("002")]
    StackedDiagnosticsAccessedWithoutActiveHandler,
}

#[subclass]
pub enum DataException {
    #[state("001")]
    StringDataRightTruncation,
    #[state("002")]
    NullValueNoIndicatorParameter,
    #[state("003")]
    NumericValueOutOfRange,
    #[state("004")]
    NullValueNotAllowed,
    #[state("005")]
    ErrorInAssignment,
    #[state("006")]
    InvalidIntervalFormat,
    #[state("007")]
    InvalidDatetimeFormat,
    #[state("008")]
    DatetimeFieldOverflow,
    #[state("009")]
    InvalidTimeZoneDisplacementValue,
    #[state("00B")]
    EscapeCharacterConflict,
    #[state("00C")]
    InvalidUseOfEscapeCharacter,
    #[state("00D")]
    InvalidEscapeOctet,
    #[state("00E")]
    NullValueInArrayTarget,
    #[state("00F")]
    ZeroLengthCharacterString,
    #[state("00G")]
    MostSpecificTypeMismatch,
    #[state("00H")]
    SequenceGeneratorLimitExceeded,
    #[state("00J")]
    NonidenticalNotationsWithTheSameName,
    #[state("00K")]
    NonidenticalUnparsedEntitiesWithTheSameName,
    #[state("00L")]
    NotAnXmlDocument,
    #[state("00M")]
    InvalidXmlDocument,
    #[state("00N")]
    InvalidXmlContent,
    #[state("00P")]
    IntervalValueOutOfRange,
    #[state("00Q")]
    MultisetValueOverflow,
    #[state("00R")]
    XmlValueOverflow,
    #[state("00S")]
    InvalidComment,
    #[state("00T")]
    InvalidProcessingInstruction,
    #[state("00U")]
    NotAnXQueryDocumentNode,
    #[state("00V")]
    InvalidXQueryContextItem,
    #[state("00W")]
    XQuerySerializationError,
    #[state("010")]
    InvalidIndicatorParameterValue,
    #[state("011")]
    SubstringError,
    #[state("012")]
    DivisionByZero,
    #[state("013")]
    InvalidPrecedingOrFollowingSizeInWindowFunction,
    #[state("014")]
    InvalidArgumentForNtileFunction,
    #[state("015")]
    IntervalFieldOverflow,
    #[state("016")]
    InvalidArgumentForNthValueFunction,
    #[state("017")]
    InvalidDataSpecifiedForDatalink,
    #[state("018")]
    InvalidCharacterValueForCast,
    #[state("019")]
    InvalidEscapeCharacter,
    #[state("01A")]
    NullArgumentPassedToDatalinkConstructor,
    #[state("01B")]
    InvalidRegularExpression,
    #[state("01C")]
    NullRowNotPermittedInTable,
    #[state("01D")]
    DatalinkValueExceedsMaximumLength,
    #[state("01E")]
    InvalidArgumentForNaturalLogarithm,
    #[state("01F")]
    InvalidArgumentForPowerFunction,
    #[state("01G")]
    InvalidArgumentForWidthBucketFunction,
    #[state("01H")]
    InvalidRowVersion,
    #[state("01J")]
    XQuerySequenceCannotBeValidated,
    #[state("01K")]
    XQueryDocumentNodeCannotBeValidated,
    #[state("01L")]
    NoXmlSchemaFound,
    #[state("01M")]
    ElementNamespaceNotDeclared,
    #[state("01N")]
    GlobalElementNotDeclared,
    #[state("01P")]
    NoXmlElementWithSpecifiedQName,
    #[state("01Q")]
    NoXmlElementWithSpecifiedNamespace,
    #[state("01R")]
    ValidationFailure,
    #[state("01S")]
    InvalidQueryRegularExpression,
    #[state("01T")]
    InvalidQueryOptionFlag,
    #[state("01U")]
    AttemptToReplaceAZeroLengthString,
    #[state("01V")]
    InvalidQueryReplacementString,
    #[state("01W")]
    InvalidRowCountInFetchFirstClause,
    #[state("01X")]
    InvalidRowCountInResultOffsetClause,
    #[state("021")]
    CharacterNotInRepertoire,
    #[state("022")]
    IndicatorOverflow,
    #[state("023")]
    InvalidParameterValue,
    #[state("024")]
    UnterminatedCString,
    #[state("025")]
    InvalidEscapeSequence,
    #[state("026")]
    StringDataLengthMismatch,
    #[state("027")]
    TrimError,
    #[state("029")]
    NoncharacterInUcsString,
    #[state("02A")]
    NullValueInFieldReference,
    #[state("02D")]
    NullValueSubstitutedForMutatorSubjectParameter,
    #[state("02E")]
    ArrayElementError,
    #[state("02F")]
    ArrayDataRightTruncation,
    #[state("02G")]
    InvalidRepeatArgumentInASampleClause,
    #[state("02H")]
    InvalidSampleSize,
}

#[subclass]
pub enum IntegrityConstraintViolation {
    #[state("001")]
    RestrictViolation,
}

#[subclass]
pub enum InvalidTransactionState {
    #[state("001")]
    ActiveSqlTransaction,
    #[state("002")]
    BranchTransactionAlreadyActive,
    #[state("003")]
    InappropriateAccessModeForBranchTransaction,
    #[state("004")]
    InappropriateIsolationLevelForBranchTransaction,
    #[state("005")]
    NoActiveSqlTransactionForBranchTransaction,
    #[state("006")]
    ReadOnlySqlTransaction,
    #[state("007")]
    SchemaAndDataStatementMixingNotSupported,
    #[state("008")]
    HeldCursorRequiresSameIsolationLevel,
}

#[subclass]
pub enum TriggeredDataChangeViolation {
    #[state("001")]
    ModifyTableModifiedByDataChangeDeltaTable,
}

#[subclass]
pub enum SqlRoutineException {
    #[state("002")]
    ModifyingSqlDataNotPermitted,
    #[state("003")]
    ProhibitedSqlStatementAttempted,
    #[state("004")]
    ReadingSqlDataNotPermitted,
    #[state("005")]
    FunctionExecutedNoReturnStatement,
}

#[subclass]
pub enum CursorSensitivityException {
    #[state("001")]
    RequestRejected,
    #[state("002")]
    RequestFailed,
}

#[subclass]
pub enum ExternalRoutineException {
    #[state("001")]
    ContainingSqlNotPermitted,
    #[state("002")]
    ModifyingSqlDataNotPermitted,
    #[state("003")]
    ProhibitedSqlStatementAttempted,
    #[state("004")]
    ReadingSqlDataNotPermitted,
}

#[subclass]
pub enum ExternalRoutineInvocationException {
    #[state("001")]
    NullValueNotAllowed,
}

#[subclass]
pub enum SavepointException {
    #[state("001")]
    InvalidSavepointSpecification,
    #[state("002")]
    TooManySavepoints,
}

#[subclass]
pub enum TransactionRollback {
    #[state("001")]
    SerializationFailure,
    #[state("002")]
    IntegrityConstraintViolation,
    #[state("003")]
    StatementCompletionUnknown,
    #[state("004")]
    TriggeredActionException,
}

#[subclass]
pub enum OlbSpecificError {
    #[state("001")]
    InvalidUrl,
    #[state("002")]
    InvalidJarName,
    #[state("003")]
    InvalidClassDeletion,
    #[state("005")]
    InvalidReplacement,
    #[state("00A")]
    AttemptToReplaceUninstalledJar,
    #[state("00B")]
    AttemptToRemoveUninstalledJar,
    #[state("00C")]
    InvalidJarRemoval,
    #[state("00D")]
    InvalidPath,
    #[state("00E")]
    SelfReferencingPath,
    #[state("102")]
    InvalidJarNameInPath,
    #[state("103")]
    UnresolvedClassName,
    #[state("110")]
    UnsupportedFeature,
    #[state("120")]
    InvalidClassDeclaration,
    #[state("121")]
    InvalidColumnName,
    #[state("122")]
    InvalidNumberOfColumns,
    #[state("130")]
    InvalidProfileState,
}

#[subclass]
pub enum DatalinkException {
    #[state("001")]
    ExternalFileNotLinked,
    #[state("002")]
    ExternalFileAlreadyLinked,
    #[state("003")]
    ReferencedFileDoesNotExist,
    #[state("004")]
    InvalidWriteToken,
    #[state("005")]
    InvalidDatalinkConstruction,
    #[state("006")]
    InvalidWritePermissionForUpdate,
    #[state("007")]
    ReferencedFileNotValid,
}

#[subclass]
pub enum FdwSpecificCondition {
    #[state("001")]
    MemoryAllocationError,
    #[state("002")]
    DynamicParameterValueNeeded,
    #[state("004")]
    InvalidDataType,
    #[state("005")]
    ColumnNameNotFound,
    #[state("006")]
    InvalidDataTypeDescriptors,
    #[state("007")]
    InvalidColumnName,
    #[state("008")]
    InvalidColumnNumber,
    #[state("009")]
    InvalidUseOfNullPointer,
    #[state("00A")]
    InvalidStringFormat,
    #[state("00B")]
    InvalidHandle,
    #[state("00C")]
    InvalidOptionIndex,
    #[state("00D")]
    InvalidOptionName,
    #[state("00J")]
    OptionNameNotFound,
    #[state("00K")]
    ReplyHandle,
    #[state("00L")]
    UnableToCreateExecution,
    #[state("00M")]
    UnableToCreateReply,
    #[state("00N")]
    UnableToEstablishConnection,
    #[state("00P")]
    NoSchemas,
    #[state("00Q")]
    SchemaNotFound,
    #[state("00R")]
    TableNotFound,
    #[state("010")]
    FunctionSequenceError,
    #[state("014")]
    LimitOnNumberOfHandlesExceeded,
    #[state("021")]
    InconsistentDescriptorInformation,
    #[state("024")]
    InvalidAttributeValue,
    #[state("090")]
    InvalidStringLengthOrBufferLength,
    #[state("091")]
    InvalidDescriptorFieldIdentifier,
}

#[subclass]
pub enum CliSpecificCondition {
    // No subclass value defined for these two variants.
    // DynamicParameterValueNeeded,
    // InvalidHandle,
    #[state("001")]
    MemoryAllocationError,
    #[state("003")]
    InvalidDataTypeInApplicationDescriptor,
    #[state("004")]
    InvalidDataType,
    #[state("007")]
    AssociatedStatementIsNotPrepared,
    #[state("008")]
    OperationCanceled,
    #[state("009")]
    InvalidUseOfNullPointer,
    #[state("010")]
    FunctionSequenceError,
    #[state("011")]
    AttributeCannotBeSetNow,
    #[state("012")]
    InvalidTransactionOperationCode,
    #[state("013")]
    MemoryManagementError,
    #[state("014")]
    LimitOnNumberOfHandlesExceeded,
    #[state("017")]
    InvalidUseOfAutomaticallyAllocatedDescriptorHandle,
    #[state("018")]
    ServerDeclinedCancellationRequest,
    #[state("019")]
    NonStringDataCannotBeSentInPieces,
    #[state("020")]
    AttemptToConcatenateANullValue,
    #[state("021")]
    InconsistentDescriptorInformation,
    #[state("024")]
    InvalidAttributeValue,
    #[state("055")]
    NonStringDataCannotBeUsedWithStringRoutine,
    #[state("090")]
    InvalidStringLengthOrBufferLength,
    #[state("091")]
    InvalidDescriptorFieldIdentifier,
    #[state("092")]
    InvalidAttributeIdentifier,
    #[state("093")]
    InvalidDatalinkValue,
    #[state("095")]
    InvalidFunctionIdSpecified,
    #[state("096")]
    InvalidInformationType,
    #[state("097")]
    ColumnTypeOutOfRange,
    #[state("098")]
    ScopeOutOfRange,
    #[state("099")]
    NullableTypeOutOfRange,
    #[state("103")]
    InvalidRetrievalCode,
    #[state("104")]
    InvalidLengthPrecisionValue,
    #[state("105")]
    InvalidParamterMode,
    #[state("106")]
    InvalidFetchOrientation,
    #[state("107")]
    RowValueOutOfRange,
    #[state("108")]
    InvalidCursorPosition,
    #[state("C00")]
    OptionalFeatureNotImplemented,
}

// TODO: RDA subconditions in ISO9579
#[subclass]
pub enum RemoteDatabaseAccess {}
