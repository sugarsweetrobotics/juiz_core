

class PyCompContainer:
    def __init__(self):
        self.buf = 0
        
def pycomp_container(manifest):
    return PyCompContainer()

def increment(pycomp, arg0):
    pycomp.buf = pycomp.buf + arg0
    return pycomp.buf

def get(pycomp):
    return pycomp.buf


def component_profile():
    return {
        "type_name": "pycomponent",
        "containers": [
            {
                "type_name": "pycomp_container",
                "processes": [
                    {
                        "type_name": "increment",
                        "arguments": {
                            "arg0": {
                                "type": "int",
                                "description": "test_argument",
                                "default": 1,
                            }
                        }
                    },
                    {
                        "type_name": "get",
                        "arguments": {
                        }
                    },
                ]
            }
        ]
    }