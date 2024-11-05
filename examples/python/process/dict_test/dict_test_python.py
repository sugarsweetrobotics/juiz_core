from juiz.juiz import ProcessManifest

def pydict_test(arg0, arg1):
    return {
        "arg0": arg0,
        "arg1": arg1,
        "string_data": "this_is_string",
    }

def manifest():
    return ProcessManifest("dict_test_python")\
        .add_object_arg("arg0", "test_argument", {})\
        .add_object_arg("arg1", "test_argument", {})\
        .into_value()

def process_factory():
    return pydict_test