
from juiz import ContainerManifest

class PyContainer:
    value: int
    def __init__(self, value):
        self.value = value
        
    @classmethod
    def manifest(cls):
        return ContainerManifest.new("example_container_python") \
            .into_value()
        
def example_container_python(manifest):
    return PyContainer(manifest.get("value", 0))

def container_factory():
    return PyContainer.manifest(), example_container_python