
def manifest():
    return {
        "type_name": "example_container_python_increment",
        "container_type_name": "example_container_python",
        "arguments" : {
            "arg0": {
                "type": "int",
                "description": "test_argument",
                "default": 1,
            }
        }, 
    }


def example_container_python_increment(container, arg0):
    container.value = container.value + arg0
    return container.value
