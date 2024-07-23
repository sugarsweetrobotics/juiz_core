

def pydict_test(arg0, arg1):
    return {
        "arg0": arg0,
        "arg1": arg1,
        "string_data": "this_is_string",
    }

def manifest():
    return {
        "type_name": "dict_test_python",
        "arguments" : {
            "arg0": {
                "type": "int",
                "description": "test_argument",
                "default": 1,
            }, 
            "arg1": {
                "type": "int",
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    }

def process_factory():
    return pydict_test