from dataclasses import dataclass, asdict
from typing import List, Any

from juiz import ProcessManifest


def manifest():
    v = ProcessManifest.new("increment_process_python")\
        .set_description("increment function") \
        .add_int_arg("arg1", "test_argument value", 1)\
        .into_value()
    return v
        
    
def increment_process(arg1):
    print('pyadd ', arg1, '>', arg1+1)
    return arg1 + 1

def process_factory():
    return manifest(), increment_process