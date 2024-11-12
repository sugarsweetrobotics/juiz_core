from dataclasses import dataclass, asdict
from typing import List, Any, Optional

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
    description: str = ""
    factory: str = "process_factory"
    use_demo: bool = False
    language: str = "python"
    name: Optional[str] = None
    container_name: Optional[str] = None
    container_type: Optional[str] = None
    
    @classmethod
    def new(cls, type_name):
        return ProcessManifest(type_name=type_name, arguments=[])
        
    def set_name(self, n):
        self.name = n
        return self
    
    def set_description(self, desc):
        self.description = desc
        return self
        
    def set_language(self, language:str):
        self.language = language
        return self
    
    def set_factory(self, factory:str):
        self.factory = factory
        return self
    
    def set_container_type(self, container_type:str):
        self.container_type = container_type
        return self
    
    def set_container_name(self, container_name:Optional[str]):
        self.container_name = container_name
        return self
        
    def add_argument(self, argument_manifest):
        self.arguments.append(argument_manifest)
        return self

    def add_bool_arg(self, name, description, default_value:bool):
        self.add_argument(ArgumentManifest.new("bool", name, description, default_value))
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

    def add_object_arg(self, name, description, default_value: dict):
        self.add_argument(ArgumentManifest.new("object", name, description, default_value))
        return self
    
    def add_array_arg(self, name, description, default_value: list):
        self.add_argument(ArgumentManifest.new("array", name, description, default_value))
        return self
        
    def into_value(self):
        return asdict(self, dict_factory=lambda x: {k: v for (k, v) in x if v is not None})
    
@dataclass
class ContainerManifest:
    type_name: str
    args: dict
    processes: List[ProcessManifest] 
    arguments: List[ArgumentManifest]
    language: str = "python"
    factory: str = "container_factory"
    description: str = ""
    parent_type_name: Optional[str] = None
    parent_name: Optional[str] = None
    name: Optional[str] = None
    
    @classmethod
    def new(cls, type_name):
        return ContainerManifest(type_name=type_name, args={}, processes=[], arguments=[])
    
    def add_process(self, process_manifest: ProcessManifest):
        self.processes.append(process_manifest\
            .set_container_type(self.type_name)\
            .set_container_name(self.name))
        return self
    
    def set_description(self, desc): 
        self.description = desc
        return self
    
    def into_value(self):
        return asdict(self, dict_factory=lambda x: {k: v for (k, v) in x if v is not None})
    
    def add_argument(self, argument_manifest):
        self.arguments.append(argument_manifest)
        return self
    
    def add_bool_arg(self, name, description, default_value:bool):
        self.add_argument(ArgumentManifest.new("bool", name, description, default_value))
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

    def add_object_arg(self, name, description, default_value: dict):
        self.add_argument(ArgumentManifest.new("object", name, description, default_value))
        return self
    
    def add_array_arg(self, name, description, default_value: list):
        self.add_argument(ArgumentManifest.new("array", name, description, default_value))
        return self
@dataclass
class ComponentManifest:
    type_name: str
    containers: List[ContainerManifest]
    processes: List[ProcessManifest]
    description: str = ""
    language: str = "python"
    
    @classmethod
    def new(cls, type_name):
        return ComponentManifest(type_name=type_name, containers=[], processes=[])
    
    def set_description(self, desc):
        self.description = desc
        return self
    
    def set_language(self, lang):
        self.language = lang
        return self
    
    def add_container(self, c):
        self.containers.append(c)
        return self
    
    def add_process(self, p: ProcessManifest):
        self.processes.append(p)
        return self