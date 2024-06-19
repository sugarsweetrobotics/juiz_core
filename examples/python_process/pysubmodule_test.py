import sys

sys.path.append('.')
from submodule import submodule_func

def pysubmodule_test(arg0, arg1):
    return submodule_func(arg0, arg1)

def manifest():
    return {
        "type_name": "pysubmodule_test",
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
