from juiz.juiz import ProcessManifest
from juiz import juiz_container_process

# def manifest():
#     return ProcessManifest("example_container_python_get")\
#         .set_container_type("example_container_python")\
#         .into_value()

@juiz_container_process(
    container_type="example_container_python"
)
def example_container_python_get(container):
    # print(f'example_container_python_get({container}) called')
    return container.value

# def container_process_factory():
#     return manifest(), example_container_python_get