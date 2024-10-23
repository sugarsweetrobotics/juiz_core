from dataclasses import dataclass, asdict
from typing import List, Any

@dataclass
class ArgumentManifest:
    type: str
    name: str
    default: Any
    description: str = ""
    
    @classmethod
    def new(cls, type_name, name, description, default_value):
        return ArgumentManifest(type=type_name, name=name, default=default_value, description=description )
    
    
@dataclass
class ProcessManifest:
    
    type_name: str
    arguments: List[ArgumentManifest]
    language: str = ""
    dscription: str = ""
    
    @classmethod
    def new(cls, type_name):
        return ProcessManifest(type_name=type_name, arguments=[])
        
    def set_description(self, desc):
        self.description = desc
        return self
        
    def set_language(self, language:str):
        self.language = language
        return self
        
    def add_argument(self, argument_manifest):
        self.arguments.append(argument_manifest)
        return self

    def add_int_arg(self, name, description, default_value:int):
        self.add_argument(ArgumentManifest.new("int", name, description, default_value))
        return self
        
    def add_float_arg(self, name, description, default_value:float):
        self.add_argument(ArgumentManifest.new("float", name, description, default_value))
        return self
        
    def add_string_arg(self, name, description, default_value:str):
        self.add_argument(ArgumentManifest.new("string", name, description, default_value))
        return self

    def add_object_arg(self, name, description, default_value:Any):
        self.add_argument(ArgumentManifest.new("object", name, description, default_value))
        return self
        
    def into_value(self):
        return asdict(self)
    
def manifest():
    v = ProcessManifest.new("increment_process_python")\
        .set_description("increment function") \
        .add_int_arg("arg1", "test_argument", 1)\
        .into_value()
    return v
        
def manifest_v():
    return {
        "type_name": "increment_process_python",
        "arguments" : [
            {
                "name": "arg1",
                "type": "int",
                "description": "test_argument",
                "default": 1,
            }, 
        ]
    }
    
    
def increment_process(arg1):
    print('pyadd ', arg1, '>', arg1+1)
    return arg1 + 1

def process_factory():
    return manifest(), increment_process