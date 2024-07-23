

def pyadd(arg1):
    return arg1 + 1

def manifest():
    return {
        "type_name": "increment_process_python",
        "arguments" : { 
            "arg1": {
                "type": "int",
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    }

def process_factory():
    return pyadd