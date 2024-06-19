

def pyadd(arg0, arg1):
    return arg0 + arg1

def manifest():
    return {
        "type_name": "pyadd",
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
