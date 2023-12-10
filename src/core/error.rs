
use thiserror::Error;

use crate::Value;

#[derive(Error, Debug, PartialEq)]
#[repr(C)]
pub enum JuizError {
    #[error("JuizError(General)")]
    GeneralError,
    #[error("Value({value:}) is not object type.")]
    ValueIsNotObjectError{value: Value},
    #[error("Value({value:}) is not array type.")]
    ValueIsNotArrayError { value: Value },
    #[error("Plugin Load failed.")]
    PluginLoadFailedError{plugin_path: String},
    #[error("Plugin({plugin_path:}) tried to load symbol({symbol_name:}) but failed.")]
    PluginLoadSymbolFailedError {plugin_path: String, symbol_name: String},
    #[error("ProcessFactory({type_name:}) is already loaded.")]
    ProcessFactoryOfSameTypeNameAlreadyExistsError { type_name: String },
    #[error("ProcessFactory({type_name:}) can not be found.")]
    ProcessFactoryCanNotFoundError { type_name: String },
    #[error("Process({id:}) can not be found.")]
    ProcessCanNotFoundByIdError { id: String },
    #[error("Value(key={key:}) in Value({value:}) is not string.")]
    ValueWithKeyIsNotStringError { value: Value, key: String },
    #[error("Value(key={key:}) in Value({value:}) is not object.")]
    ValueWithKeyIsNotObjectError { value: Value, key: String },
    #[error("Value(key={key:}) in Value({value:}) can not be found.")]
    ValueWithKeyNotFoundError { value: Value, key: String },
    #[error("Locking mutex failed.")]
    MutexLockFailedError {},
    #[error("Process tried to use output_memo, but borrowing value failed.")]
    ProcessOutputMemoIsNotInitializedError { id: String },
    #[error("Process checked given argument({given_argument:}) but the argument does not contain the predefined specification ({process_manifest:})")]
    ArgumentMissingWhenCallingError { process_manifest: Value, given_argument: Value, missing_arg_name: String },
    #[error("Process manifest includes invalid value type string. Manifest is ({manifest:}), type string is ({type_string:}")]
    ManifestArgumentDefaultValueIsInvalidTypeError {manifest: Value, type_string: String  },
    #[error("Process manifest includes invalid value type in default arugment value.Value is ({value:}")]
    ManifestArgumentDefaultValueIsNotAvailableValueTypeError { value: Value },
    #[error("Target Value (key={key:}) in Manifest ({value:}) is not expected type ({expected_type:})")]
    ManifestValueIsNotExpectedTypeError { value: serde_json::Value, key: String, expected_type: String },
    #[error("Manifest Value (key={key:}, value={set_type:}) in Manifest ({value:}) is not equal to default_type ({expected_type:})")]
    ManifestDefaultValueIsNotExpectedTypeError { value: serde_json::Value, key: String, set_type: String, expected_type: String },
    #[error("Given Connection type is invalid (manifest: {manifest:})")]
    ConnectionTypeError { manifest: serde_json::Value },
    #[error("ContainerFactory({type_name:}) can not be found.")]
    ContainerFactoryCanNotFoundError { type_name: String },
    #[error("Container({id:}) can not be found.")]
    ContainerCanNotFoundByIdError { id: String },
    #[error("Container({identifier:}) can not be downcast.")]
    ContainerDowncastingError { identifier: String },
    #[error("ContainerProcessFactory({type_name:}) can not be found.")]
    ContainerProcessFactoryCanNotFoundError { type_name: String },
    #[error("ContainerFactory({type_name:}) is already loaded.")]
    ContainerFactoryOfSameTypeNameAlreadyExistsError { type_name: String },
    #[error("ContainerProcessFactory({type_name:}) is already loaded.")]
    ContainerProcessFactoryOfSameTypeNameAlreadyExistsError { type_name: String },
    #[error("BrokerFactory({type_name:}) is already loaded.")]
    BrokerFactoryOfSameTypeNameAlreadyExistsError { type_name: String },
    #[error("BrokerProxyFactory({type_name:}) is already loaded.")]
    BrokerProxyFactoryOfSameTypeNameAlreadyExistsError { type_name: String },
    #[error("BrokerFactory({type_name_bf:}) and BrokerProxyFactory({type_name_bpf:}) is loaded.")]
    BrokerFactoryAndBrokerProxyFactoryWithDifferentTypeIsRegisteredError { type_name_bf: String, type_name_bpf: String },
    #[error("BrokerFactory({type_name:}) can not be found.")]
    BrokerFactoryCanNotFoundError { type_name: String },
    #[error("BrokerStopFailedError (type_name={type_name:})")]
    BrokerStopFailedError { type_name: String },
    /*
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
    DestinationConnectionNewReceivedInvalidManifestTypeError,
    ProcessFactoryOfSameTypeNameAlreadyExistsError,
    ProcessFactoryCanNotFoundByTypeNameError,
    ManifestIsNotObjectError {  },
    ManifestTypeNameIsNotStringError {  },
    ManifestDoesNotIncludeTypeNameError {  },
    CoreStoreCanNotLockProcessFactoryError {  },
    CoreBrokerCanNotLockProcessFactoryMutexError {  },
    CoreBrokerCanNotInsertProcessError {  },
    PluginLoadFailedError {  },
    ProcesssFactoryCanNotLockError {  },
    ManifestIsNotStringError {  },
    ManifestDoesNotContainsKeyError {  },
    ManifestIsNotArrayError {  },
    ProcessFactoryWrapperCanNotLockProcessFactoryError {  },
    MutexLockFailedError {  },
    ManifestDoesNotIncludeKeyError {  },
    */
}


