

def manifest():
    return {
        "type_name": "example_container_python",
    }
    
class PyContainer:
    value: int
    def __init__(self, value):
        self.value = value

def example_container_python(manifest):
    return PyContainer(manifest.get("value", 0))

