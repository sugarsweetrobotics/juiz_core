

def pydict_test(arg0, arg1):
    return {
        "arg0": arg0,
        "arg1": arg1,
        "string_data": "this_is_string",
    }

def manifest():
    print('manifest called')
    return {
        "type_name": "pydict_test",
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
