
from juiz import *

class PyContainer:
    value: int
    def __init__(self, value):
        self.value = value

@juiz_container
def example_container_python(initial_value:int = 0):
    # print(f'example_container_python(value = {initial_value}) called')
    return PyContainer(initial_value)
