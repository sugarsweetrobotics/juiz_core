


class PyContainer:
    value: int
    def __init__(self, value):
        self.value = value
        
    @classmethod
    def manifest(cls):
        return {
            "type_name": "example_container_python",
        }   

def example_container_python(manifest):
    return PyContainer(manifest.get("value", 0))

def container_factory():
    return PyContainer.manifest(), example_container_python