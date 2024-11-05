
from juiz.juiz import ProcessManifest

def manifest():
    return ProcessManifest("example_container_python_increment")\
        .set_container_type("example_container_python")\
        .add_int_arg("arg0", "test_argument", 1)\
        .into_value()

def example_container_python_increment(container, arg0):
    container.value = container.value + arg0
    return container.value

def container_process_factory():
    return manifest(), example_container_python_increment