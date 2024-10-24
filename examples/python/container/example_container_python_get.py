from juiz import ProcessManifest

def manifest():
    return ProcessManifest("example_container_python_get")\
        .set_container_type("example_container_python")\
        .into_value()

def example_container_python_get(container):
    return container.value

def container_process_factory():
    return manifest(), example_container_python_get