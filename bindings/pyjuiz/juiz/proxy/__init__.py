import httpx
import json
from PIL import Image
from io import BytesIO


class UnsupportedMediaTypeError(Exception): pass
class BadRequestError(Exception): pass
class ForbiddenError(Exception): pass
class NotFoundError(Exception): pass
class RequestTimeoutError(Exception): pass
class InternalServerError(Exception): pass


error_code_and_ex={
    415: UnsupportedMediaTypeError,
    400: BadRequestError,
    403: ForbiddenError,
    404: NotFoundError,
    408: RequestTimeoutError,
    500: InternalServerError,
}

class CRUDProxy():
    
    def __init__(self):
        pass
    
    def _url(self, class_name, function_name, param_dict: dict=None):
        if param_dict is None:
            return f'/api/{class_name}/{function_name}'
        else:
            return f'/api/{class_name}/{function_name}?' + '&'.join([f'{k}={v}' for k, v in param_dict.items()])
    
    def read(self, class_name, function_name, param_dict={}):
        pass
    
    def delete(self, class_name, function_name, param_dict={}):
        pass
    
    def create(self, class_name, function_name, body, param_dict={}):
        pass
    
    def update(self, class_name, function_name, body, param_dict={}):
        pass
    
class HttpProxy(CRUDProxy):
    
    def __init__(self, host, port):
        self._host = host
        self._port = port
        self._base_url = 'http://' + self._host + ':' + str(self._port)
        pass

    def _do_response(self, response):
        if response.status_code == 200:
            ct = response.headers.get('content-type', None)
            if ct and ct == 'application/json':
                return response.json()
            elif ct and ct == 'image/png':
                return Image.open(BytesIO(response.content)).convert('RGB')
        else:
            raise error_code_and_ex[response.status_code]()
    
    def read(self, class_name, function_name, param_dict={}):
        url = self._url(class_name, function_name, param_dict)
        return self._get(url)
    
    def delete(self, class_name, function_name, param_dict={}):
        url = self._url(class_name, function_name, param_dict)
        return self._delete(url)
    
    def create(self, class_name, function_name, body: dict, param_dict: dict={}):
        url = self._url(class_name, function_name, param_dict)
        return self._post(url, body)
    
    def update(self, class_name, function_name, body, param_dict={}):
        url = self._url(class_name, function_name, param_dict)
        return self._patch(url, body)
    
    def _get(self, url):
        with httpx.Client() as client:
            return self._do_response(client.get(self._base_url + url))                
        return None

    def _delete(self, url):
        with httpx.Client() as client:
            return self._do_response(client.delete(self._base_url + url))                
        return None

    def create(self, url, data):
        with httpx.Client() as client:
            return self._do_response(client.create(self._base_url + url,
                                    data=json.dumps(data),
                                   headers={"Content-Type": "application/json"}
                                   ))
        return None
        
    def _post(self, url, data):
        with httpx.Client() as client:
            return self._do_response(client.post(self._base_url + url,
                                    data=json.dumps(data),
                                   headers={"Content-Type": "application/json"}
                                   ))
        return None

    def _patch(self, url, data):
        with httpx.Client() as client:
            return self._do_response(client.patch(self._base_url + url,
                                    data=json.dumps(data),
                                   headers={"Content-Type": "application/json"}
                                   )) 
        return None

class SystemProxy():

    def __init__(self, base_proxy: CRUDProxy):
        self.crud = base_proxy
        self._processes = None
        self._containers = None
        self._container_processes = None
        pass

    def profile_full(self, update=False):
        return self.crud.read('system', 'profile_full')

    def create_process(self, type_name, name, use_memo=True):
        return self.crud.create('/api/process/', {
            "type_name": type_name,
            "name": name,
            "use_memo": use_memo})
        
    def processes(self):
        ps = self.crud.read('process', 'list')
        self._processes = [ProcessProxy(self, pid) for pid in ps]
        return self._processes.copy()

    def process(self, id):
        ps = self.crud.read('process', 'list')
        for pid in ps:
            if pid == id:
                return ProcessProxy(self, pid)
        return None            
        
    def containers(self, update=False):
        ps = self.crud.read('container', 'list')
        self._containers = [ContainerProxy(self, pid) for pid in ps]
        return self._containers

    def container(self, id):
        cs = self.crud.read('container', 'list')
        for pid in cs:
            if pid == id:
                return ContainerProxy(self, pid)
        return None            
    
    def container_processes(self, update=False):
        ps = self.crud.read('container_process','list')
        if self._container_processes is None or update:
            self._container_processes = [ContainerProcessProxy(self, p) for p in ps]
        return self._container_processes

class ObjectProxy(object):
    
    def __init__(self, system_proxy, class_name, identifier):
        self.system = system_proxy
        self._id = identifier
        self._cn = class_name
        self._profile = None

    def profile_full(self, update=False):
        if self._profile is None or update:
            self._profile = self.system.crud.read(self._cn, 'profile_full', {'identifier': self._id})
        return self._profile

    def identifier(self, update=False):
        return self._id

    def destroy(self):
        return self._system_proxy.delete('/api/' + self._class_name + '/destroy?identifier=' + self._identifier)

class ProcessProxy(ObjectProxy):
    def __init__(self, system_proxy, identifier):
        super().__init__(system_proxy, 'process', identifier)    

    def call(self, *args, **kwargs):
        if len(args) == 1:
            body = args[0]
        elif 'body' in kwargs.keys():
            body = kwargs['body']
        else:
            body = kwargs
        return self.system.crud.update(self._cn, 'call', body, {'identifier': self._id})

    def execute(self):
        return self.system.crud.update(self._cn, 'execute', {}, {'identifier': self._id})
    
    def __repr__(self):
        return f'ProcessProxy("{self.identifier()}")'

class ContainerProxy(ObjectProxy):
    def __init__(self, system_proxy, identifier):
        super().__init__(system_proxy, 'container', identifier)    
        self._profile = self.profile_full()

    def __repr__(self):
        return f'ContainerProxy("{self.identifier()}")'
    
    def processes(self):
        pids = self._profile['processes']
        return [ContainerProcessProxy(self.system, pid) for pid in pids]
        
    def process(self, id):
        pids = self._profile['processes']
        for pid in pids:
            if pid == id:
                return ContainerProcessProxy(self.system, pid)
        return None            
    

class ContainerProcessProxy(ObjectProxy):
    def __init__(self, system_proxy, identifier):
        super().__init__(system_proxy, 'container_process', identifier)    

    def call(self, *args, **kwargs):
        if len(args) == 1:
            body = args[0]
        elif 'body' in kwargs.keys():
            body = kwargs['body']
        else:
            body = kwargs
        return self.system.crud.update(self._cn, 'call', body, {'identifier': self._id})

    def execute(self):
        return self.system.crud.update(self._cn, 'execute', {}, {'identifier': self._id})

    def __repr__(self):
        return f'ContainerProcess("{self.identifier()}")'

class IdentifierStruct(object):

    def __init__(self, broker_type, broker_name, class_name, type_name, name):
        self.broker_type = broker_type
        self.broker_name = broker_name
        self.class_name = class_name
        self.type_name = type_name
        self.name = name

    @classmethod
    def fromIdentifier(self, id):
        s, type_name = id.split('::')
        broker_type, s2 = s.split('://')
        broker_name, class_name, name = s2.split('/')
        return IdentifierStruct(broker_type, broker_name, class_name, type_name, name)

    def to_id(self):
        return f'{self.broker_type}://{self.broker_name}/{self.class_name}/{self.name}::{self.type_name}'

    def __repr__(self):
        return f'IdentifierStruct(broker_type="{self.broker_type}",broker_name="{self.broker_name}",class_name="{self.class_name}",type_name="{self.type_name}",name="{self.name}")'
