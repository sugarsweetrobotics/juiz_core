

class PyContainer:
    data: int
    def __init__(self):
        self.data = 0

def pycontainer(manifest):
    print('pycontainer:', manifest)
    return PyContainer()

def manifest():
    return {
        "type_name": "pycontainer",
    }
