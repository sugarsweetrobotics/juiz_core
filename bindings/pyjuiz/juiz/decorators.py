import datetime
import inspect
import functools
import io

from .juiz import *
from PIL.Image import Image
class PyJuizProcessArgumentUnknownTypeError(Exception): pass
class PyJuizContainerArgumentUnknownTypeError(Exception): pass

def allow_no_arg_decorator(decorator_function):
    def wrapper(*args, **kwargs):
        
        if len(args) != 0 and callable(args[0]):
            decorated_function= args[0]
            return functools.wraps(decorated_function)(decorator_function(decorated_function))
        else:
            def _wrapper(decorated_function):
                return functools.wraps(decorated_function)(decorator_function(decorated_function, *args, **kwargs))
            return _wrapper
    return wrapper


class JuizProcess(object):
    def __init__(self, proc):
        self.__proc = proc
        self.name = proc.__name__
        self.signature = inspect.signature(proc)
        self._manifest = ProcessManifest.new(self.name).set_description(proc.__doc__)
        for p in self.signature.parameters:
            param = self.signature.parameters[p]
            param_type = param.annotation
            param_default = param.default
            if param_type is inspect._empty:
                if isinstance(param_default, int):
                    param_type = int
                elif isinstance(param_default, bool):
                    param_type = bool
                elif isinstance(param_default, float):
                    param_type = float
                elif isinstance(param_default, list):
                    param_type = list
                elif isinstance(param_default, str):
                    param_type = str
                elif isinstance(param_default, dict):
                    param_type = dict
                else:
                    raise PyJuizProcessArgumentUnknownTypeError()
            if param_default is inspect._empty:
                if param_type == int:
                    param_default = 0
                elif param_type == float:
                    param_default = 0.0
                elif param_type == dict:
                    param_default = {}
                elif param_type == list:
                    param_default = []
                elif param_type == bool:
                    param_default = False
                elif param_type == str:
                    param_default = ""
                elif param_type == Image:
                    param_default =  None
                else:
                    raise PyJuizProcessArgumentUnknownTypeError()
                
            # print('p:', param.name, param.default, param.kind, param.annotation, param)
            if param_type == int:
                self._manifest.add_int_arg(param.name, "", param_default)
            elif param_type == float:
                self._manifest.add_float_arg(param.name, "", param_default)
            elif param_type == bool:
                self._manifest.add_bool_arg(param.name, "", param_default)
            elif param_type == str:
                self._manifest.add_string_arg(param.name, "", param_default)
            elif param_type == dict:
                self._manifest.add_object_arg(param.name, "", param_default)
            elif param_type == list:
                self._manifest.add_array_arg(param.name, "", param_default)
            elif param_type == Image:
                self._manifest.add_image_arg(param.name, "", param_default)
    def manifest(self):
        return self._manifest.into_value()
        
    def __call__(self, *args, **kwargs):
        return convert_process_result(self.__proc(*args, **kwargs))
    
@allow_no_arg_decorator
def juiz_process(process_function, description="Default Description"):
    return JuizProcess(process_function)

class JuizContainer(object):
    def __init__(self, proc):
        self.__proc = proc
        self.name = proc.__name__
        self.signature = inspect.signature(proc)
        self._manifest = ContainerManifest.new(self.name).set_description(proc.__doc__)
        for p in self.signature.parameters:
            param = self.signature.parameters[p]
            param_type = param.annotation
            param_default = param.default
            if param_type is inspect._empty:
                if isinstance(param_default, int):
                    param_type = int
                elif isinstance(param_default, bool):
                    param_type = bool
                elif isinstance(param_default, float):
                    param_type = float
                elif isinstance(param_default, list):
                    param_type = list
                elif isinstance(param_default, str):
                    param_type = str
                elif isinstance(param_default, dict):
                    param_type = dict
                elif isinstance(param_default, Image):
                    param_type = Image
                else:
                    raise PyJuizProcessArgumentUnknownTypeError()
            if param_default is inspect._empty:
                if param_type == int:
                    param_default = 0
                elif param_type == float:
                    param_default = 0.0
                elif param_type == dict:
                    param_default = {}
                elif param_type == list:
                    param_default = []
                elif param_type == bool:
                    param_default = False
                elif param_type == str:
                    param_default = ""
                elif param_type == Image:
                    param_default =  None
                else:
                    raise PyJuizProcessArgumentUnknownTypeError()
                
            # print('p:', param.name, param.default, param.kind, param.annotation, param)
            if param_type == int:
                self._manifest.add_int_arg(param.name, "", param_default)
            elif param_type == float:
                self._manifest.add_float_arg(param.name, "", param_default)
            elif param_type == bool:
                self._manifest.add_bool_arg(param.name, "", param_default)
            elif param_type == str:
                self._manifest.add_string_arg(param.name, "", param_default)
            elif param_type == dict:
                self._manifest.add_object_arg(param.name, "", param_default)
            elif param_type == list:
                self._manifest.add_array_arg(param.name, "", param_default)
            elif param_type == Image:
                self._manifest.add_image_arg(param.name, "", param_default)
    def manifest(self):
        return self._manifest.into_value()
        
    def __call__(self, *args, **kwargs):
        return self.__proc(*args, **kwargs)
    
@allow_no_arg_decorator
def juiz_container(constructor_function, description="Default Description"):
    return JuizContainer(constructor_function)

@allow_no_arg_decorator
def juiz_component_container(constructor_function, description="Default Description"):
    return JuizContainer(constructor_function)

class JuizContainerProcess(object):
    def __init__(self, proc, container_type):
        self.__proc = proc
        self.name = proc.__name__
        self.signature = inspect.signature(proc)
        self._manifest = ProcessManifest.new(self.name).set_description(proc.__doc__).set_container_type(container_type)
        for i, p in enumerate(self.signature.parameters):
            if i == 0:
                continue
            param = self.signature.parameters[p]
            param_type = param.annotation
            param_default = param.default
            if param_type is inspect._empty:
                if isinstance(param_default, int):
                    param_type = int
                elif isinstance(param_default, bool):
                    param_type = bool
                elif isinstance(param_default, float):
                    param_type = float
                elif isinstance(param_default, list):
                    param_type = list
                elif isinstance(param_default, str):
                    param_type = str
                elif isinstance(param_default, dict):
                    param_type = dict
                elif isinstance(param_default, Image):
                    param_type = Image
                else:
                    raise PyJuizProcessArgumentUnknownTypeError()
            if param_default is inspect._empty:
                if param_type == int:
                    param_default = 0
                elif param_type == float:
                    param_default = 0.0
                elif param_type == dict:
                    param_default = {}
                elif param_type == list:
                    param_default = []
                elif param_type == bool:
                    param_default = False
                elif param_type == str:
                    param_default = ""
                elif param_type == Image:
                    param_default = None
                else:
                    print('Param Type Unknown:', param_type)
                    raise PyJuizProcessArgumentUnknownTypeError()
                
            # print('p:', param.name, param.default, param.kind, param.annotation, param)
            if param_type == int:
                self._manifest.add_int_arg(param.name, "", param_default)
            elif param_type == float:
                self._manifest.add_float_arg(param.name, "", param_default)
            elif param_type == bool:
                self._manifest.add_bool_arg(param.name, "", param_default)
            elif param_type == str:
                self._manifest.add_string_arg(param.name, "", param_default)
            elif param_type == dict:
                self._manifest.add_object_arg(param.name, "", param_default)
            elif param_type == list:
                self._manifest.add_array_arg(param.name, "", param_default)
            elif param_type == Image:
                self._manifest.add_image_arg(param.name, "", param_default)
    def manifest(self):
        return self._manifest.into_value()
        
    def __call__(self, *args, **kwargs):
        return convert_process_result(self.__proc(*args, **kwargs))

def convert_process_result(retval):
    # if isinstance(retval, Image):
    #     output = io.BytesIO()
    #     retval.save(output, format='PNG')
    #     return output.getvalue() # Hex Data
    return retval

@allow_no_arg_decorator
def juiz_container_process(constructor_function, container_type:str, description="Default Description"):
    return JuizContainerProcess(constructor_function, container_type)

@allow_no_arg_decorator
def juiz_component_container_process(constructor_function, container_type:str, description="Default Description"):
    return JuizContainerProcess(constructor_function, container_type)