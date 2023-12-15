
use std::sync::mpsc;

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

    #[error("Value(key={key:}) in Value({value:}) is not bool.")]
    ValueWithKeyIsNotBoolError { value: serde_json::Value, key: String },

    #[error("Value(key={key:}) in Value({value:}) is not object.")]
    ValueWithKeyIsNotObjectError { value: Value, key: String },
    #[error("Value(key={key:}) in Value({value:}) can not be found.")]
    ValueWithKeyNotFoundError { value: Value, key: String },
    #[error("Locking mutex failed. Error is \"{error:}\"")]
    MutexLockFailedError { error: String },
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
    #[error("Given Connection type is invalid (type_string: {type_string:})")]
    ConnectionTypeError { type_string: String },
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
    #[error("LocalBrokerProxy::send() Failed. (SendError({send_error:}))")]
    LocalBrokerProxySendError { send_error: mpsc::SendError<serde_json::Value> },
    #[error("LocalBrokerProxy::recv() Timeout. (RecvTimeoutError({error:}))")]
    LocalBrokerProxyReceiveTimeoutError{ error: mpsc::RecvTimeoutError },
    #[error("BrokerProxy::recv() failed. Function request(name={function_name:}) does not match to the response({response_function_name:})")]
    BrokerProxyFunctionNameInResponseDoesNotMatchError { function_name: String, response_function_name: String },
    #[error("Requested Function Name ({request_function_name:}) is not supported by Broker")]
    BrokerProxyRequestFunctionNameNotSupportedError { request_function_name: String },
    #[error("Broker Send Error (err={error:})")]
    BrokerSendError { error: mpsc::SendError<serde_json::Value> },
    #[error("Broker can not Lock sender mutex.")]
    BrokerSendCanNotLockSenderError {  },
    #[error("Broker can not Lock receiver mutex.")]
    BrokerRecvCanNotLockReceiverError {  },
    #[error("CRUDBroker can not detect resource class_name. resoure_name is {resource_name:}")]
    CRUDBrokerGivenResourseNameHasNoClassNameError { resource_name: String },
    #[error("CRUDBroker can not detect function ({function_name:}) in class_name ({class_name:})")]
    CRUDBrokerCanNotFindFunctionError { class_name: String, function_name: String },
    #[error("CRUDBroker can not get object identifier to execute function{function_name:}) in class({class_name:})")]
    CRUDBrokerCanNotGetIdentifierOfObjectError { class_name: String, function_name: String },
    #[error("CRUDBroker can not detect identifier parameter.")]
    CRUDBrokerIdentifierNeededError {  },
    #[error("CRUDBroker can not detect parameter (key={key_name}).")]
    CRUDBrokerCanNotParameterFunctionError { key_name: String },
    #[error("CRUDBroker can not detect parameter")]
    CRUDBrokerParameterIsInvalidTypeError {  },
    #[error("CRUDBroker can not detect method. Given method_name is {method_name:}")]
    CRUDBRokerCanNotFindMethodError { method_name: String },
    #[error("ExecutionContextCore is not connected to any process.")]
    ExecutionContextCoreNotConnectedToProcessError {  },

    #[error("Factory({type_name:}) is already loaded.")]
    FactoryOfSameTypeNameAlreadyExistsError { type_name: String },
    #[error("Factory({type_name:}) can not be found.")]
    FactoryCanNotFoundError { type_name: String },
    #[error("Object({id:}) can not be found.")]
    ObjectCanNotFoundByIdError { id: String },
    #[error("Value is not Num representation (value={value:}, key={key:})")]
    ValueWithKeyIsNotNumError { value: serde_json::Value, key: String },

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


