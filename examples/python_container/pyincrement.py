
def pyincrement(container, arg0):
    container.data = container.data + arg0
    return container.data

def manifest():
    return {
        "type_name": "pyincrement",
        "container_type_name": "pycontainer",
        "arguments" : {
            "arg0": {
                "type": "int",
                "description": "test_argument",
                "default": 1,
            }
        }, 
    }
