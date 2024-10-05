

def pyprint(arg1):
    print('print', arg1)
    return arg1

def manifest():
    return {
        "type_name": "print_value_python",
        "arguments" : { 
            "arg1": {
                "type": "int",
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    }

def process_factory():
    return pyprint