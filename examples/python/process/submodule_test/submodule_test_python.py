import sys
from juiz import ProcessManifest

sys.path.append('.')
from submodule import submodule_func

def pysubmodule_test(arg0, arg1):
    return submodule_func(arg0, arg1)

def manifest():
    return ProcessManifest("submodule_test_python")\
        .add_int_arg("arg0", "test_argument", 1)\
        .add_int_arg("arg1", "test_argument", 1)\
        .into_value()

def process_factory():
    return pysubmodule_test