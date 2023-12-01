
#[derive(Debug, PartialEq)]
pub enum JuizError {
    GeneralError,
    ProcessManifestError,
    ArgumentMissingError,
    ArgumentMissingWhenCallingError,
    ArgumentIsNotObjectError,
    ArgumentManifestIsInvalidError,
    NotImplementedError,
    ProcessCanNotFoundError,
    ManifestNameMissingError,
    ManifestTypeNameMissingError,
    ManifestArgumentsMissingError,
    ManifestArgumentsInvalidError,
    ManifestArgumentDefaultValueMissingError,
    ArgumentTypeIsNotStringError,
    ArgumentTypeIsNotMatchWithDefaultError,
    ManifestArgumentDefaultValueIsInvalidTypeError,
    ProcessOutputMemoIsNotInitializedError,
    ArgumentDescriptionIsNotStringError,
    SourceConnectionCanNotBorrowMutableProcessReferenceError,
    DestinationConnectionCanNotBorrowMutableProcessReferenceError,
    ValueAccessValueIsNotObjectError,
    ValueAccessKeyNotFoundError,
    ValueAccessValueIsNotStrError,
    ConnectionBuilderCanNotBorrowSourceProcessError,
    ConnectionBuilderCanNotBorrowDestinationProcessError,
    ProcessRackCanNotBorrowInsertedProcessError,
    CoreBrokerCanNotLockProcessMutexError,
    SourceConnectionNewReceivedInvalidManifestTypeError,
    DestinationConnectionNewReceivedInvalidManifestTypeError

}