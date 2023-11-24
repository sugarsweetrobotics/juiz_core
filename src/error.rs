
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
    ManifestArgumentsMissingError,
    ManifestArgumentsInvalidError,
    ManifestArgumentDefaultValueMissingError,
    ArgumentTypeIsNotStringError,
    ArgumentTypeIsNotMatchWithDefaultError,
    ManifestArgumentDefaultValueIsInvalidTypeError,
    ProcessOutputMemoIsNotInitializedError,
    ArgumentDescriptionIsNotStringError,
    SourceConnectionCanNotBorrowMutableProcessReferenceError
}