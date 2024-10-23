import time, math, traceback, typing
import cv2

from turtle_py import Turtle
from turtle_sim_map import TurtleSimMap
turtle_sim = None

class TurtleSim(object):
    def __init__(self, turtles=None, map_metadata=None):
        print('map_metadata=', map_metadata)
        self.timestamp = time.time()
        self.turtles = [] # typing.List[Turtle]
        if map_metadata:
            self.load_map(map_metadata)
        else:
            self.map = None
            
        if isinstance(turtles, list):
            for t in turtles:
                init_pose = t.get('init_pose', {'x':0.0,'y':0.0,'th':0.0})
                self.spawn_turtle(init_x=init_pose['x'],
                                  init_y=init_pose['y'],
                                  init_th=init_pose['th'])
        pass
    
    
    def load_map(self, map_metadata: dict):
        self.map = TurtleSimMap(map_metadata=map_metadata)

    def get_map(self):
        # print('get_map:', self.map)
        return self.map.get_colored_img()
    
    def get_map_metadata(self):
        return self.map.get_metadata()
    
    def spawn_turtle(self, init_x, init_y, init_th):
        print(f'spawn_turtle({init_x}, {init_y}, {init_th})')
        self.turtles.append(Turtle(len(self.turtles), (init_x, init_y, init_th)))
        return len(self.turtles) - 1
        
    def update(self):
        try:
            previous_time = self.timestamp
            self.timestamp = time.time()
            duration = self.timestamp - previous_time
            for t in self.turtles:
                t.update(duration)
        except Exception as e:
            traceback.print_exc()
                        
    def get_turtle_pose(self, index):
        t = self.turtles[index]
        return t.pose
        # return {
        #     "x": t.pose[0],
        #     "y": t.pose[1],
        #     "th": t.pose[2]
        # }
    
    def set_turtle_target_velocity(self, index, velocity):
        # print('set_turtle_target_velocity', index, velocity)
        t = self.turtles[index]
        return t.set_target_velocity(velocity)
    
    def get_profile(self):
        turtles = []
        for i, t in enumerate(self.turtles):
            prof = t.profile()
            prof['lidar'] = self.get_turtle_lidar(i)
            turtles.append(prof)
        return {
            'turtles': turtles
        }
        
    def get_turtle_lidar(self, index):
        ranges = [10.0] * 200
        init_data = {
           'min_angle': -1.5,
           'max_angle': 1.5,
           'angle_res':
               3 / len(ranges),
           'ranges': ranges
        }
        st = time.time()
        ld = self.map.get_lidar_on_map(self.get_turtle_pose(index), init_data)
        # print('d:', time.time() - st)
        return ld
        
def turtle_sim(manifest):
    map_metadata = manifest.get('map_metadata', None)
    turtles = manifest.get('turtles', None)
    global turtle_sim
    turtle_sim = TurtleSim(turtles=turtles, map_metadata=map_metadata)
    return turtle_sim

    
def load_map_factory(m=None):
    def load_map(turtleSim: TurtleSim, map_metadata: dict):
        return turtleSim.load_map(map_metadata)
    return load_map

    
def get_map_factory():
    def get_map(turtleSim: TurtleSim):
        return turtleSim.get_map()
    return get_map

def get_map_metadata_factory():
    def get_map_metadata(turtleSim: TurtleSim) -> dict:
        return turtleSim.get_map_metadata()
    return get_map_metadata

def get_turtle_lidar_factory():
    def get_turtle_lidar_factory(sim: TurtleSim, index: int):
        return sim.get_turtle_lidar(index)
    return get_turtle_lidar_factory

def spawn_turtle_factory():
    def spawn_turtle(turtleSim: TurtleSim, init_pose):
        return turtleSim.spawn_turtle(init_pose['x'], init_pose['y'], init_pose['th'])
    return spawn_turtle

def get_turtle_pose_factory():
    def get_turtle_pose(turtleSim: TurtleSim, index):
        p = turtleSim.get_turtle_pose(index).tolist()
        return {
            "x": p[0],
            "y": p[1],
            "th": p[2]
        }
    return get_turtle_pose
    
def set_turtle_target_velocity_factory():
    def set_turtle_target_velocity(turtleSim: TurtleSim, index: int, velocity: dict):
        return turtleSim.set_turtle_target_velocity(index, velocity)
    return set_turtle_target_velocity

def update_factory():    
    def update(turtleSim: TurtleSim):
        return turtleSim.update()
    return update

    
def get_profile_factory():
    def get_profile(turtleSim: TurtleSim) :
        return turtleSim.get_profile()
    return get_profile

def sign(a):
    if a < 0: return -1
    return +1

        
# def turtle(manifest):
#     return Turtle()

# def update_(turtle: Turtle):
#     # print('turtle_py.update() called')
#     try:
#         previous_time = turtle.timestamp
#         turtle.timestamp = time.time()
#         duration = turtle.timestamp - previous_time
#         turtle.update_velocity(duration)
#         turtle.update_pose(duration)
#         return turtle.timestamp
#     except:
#         traceback.print_exc()
        
# def update_factory_(manifest):
#     print("tutle_py.update_factory() called")
#     return update

# def increment(pycomp, arg0):
#     pycomp.buf = pycomp.buf + arg0
#     return pycomp.buf

# def get_pose(turtle):
#     print("tutle_py.get_pose() called")
#     return (turtle.timestamp, turtle.pose)

# def get_pose_factory(manifest):
#     print("tutle_py.get_pose_factory() called")
#     return get_pose

# def set_target_velocity(turtle: Turtle, target_velocity):
#     print('turtle_py.set_target_velocity({target_velocity}) called')
#     v = (target_velocity["vx"],  target_velocity["vy"],  target_velocity["wz"])
#     turtle.set_target_velocity(v)
    
def component_manifest():
    return {
        "type_name": "turtle_sim_py",
        "containers": [
            {
                "type_name": "turtle_sim",
                "factory": "turtle_sim_manifest",
                "processes": [
                    {
                        "type_name": "load_map",
                        "factory": "load_map_factory",
                        "arguments": {
                            "map_metadata": {
                                "default": {
                                    "map": "map.png",
                                    "width": 10.0,
                                    "height": 10.0,
                                    "position_of_topleft": {
                                        "x": -5.0,
                                        "y": -5.0
                                    }
                                }
                            }
                        }
                    },
                    {
                        "type_name": "get_map",
                        "factory": "get_map_factory",
                        "arguments": {
                        }
                    },
                    {
                        "type_name": "get_profile",
                        "factory": "get_profile_factory",
                        "arguments": {}
                    },
                    {
                        "type_name": "spawn_turtle",
                        "factory": "spawn_turtle_factory",
                        "arguments": {
                            "init_pose": {
                                "default": {
                                    "x": 0.0,
                                    "y": 0.0,
                                    "th": 0.0
                                }
                            }
                        }
                    },
                    {
                        "type_name": "get_turtle_pose",
                        "factory": "get_turtle_pose_factory",
                        "arguments": {
                            "index": {
                                "default": {
                                    0
                                }
                            }
                        }
                    },
                    {
                        "type_name": "get_turtle_lidar",
                        "factory": "get_turtle_lidar_factory",
                        "arguments": {
                            "index": {
                                "default": {
                                    0
                                }
                            }
                        }
                    },
                    {
                        "type_name": "set_turtle_target_velocity",
                        "factory": "set_turtle_target_velocity_factory",
                        "arguments": {
                            "index": {
                                "default": 
                                    0
                                
                            },
                            "velocity": {
                                "default": {
                                    "vx": 0.0,
                                    "vy": 0.0,
                                    "wz": 0.0
                                }
                            }
                        }
                    },
                    {
                        "type_name": "update",
                        "factory": "update_factory",
                        "arguments": {
                        }
                    },
                    {
                        "type_name": "get_map_metadata",
                        "factory": "get_map_metadata_factory",
                        "arguments": {}
                    }
                ]  
            },
            # {
            #     "type_name": "turtle",
            #     "factory": "turtle_manifest",
            #     "processes": [
            #         {
            #             "type_name": "get_pose",
            #             "factory": "get_pose_factory",
            #             "arguments": {
            #             }
            #         },
            #         #{
            #         #    "type_name": "update",
            #         #    "factory": "update_factory",
            #         #    "arguments": {
            #         #    }
            #         #},
            #         {
            #             "type_name": "set_target_velocity",
            #             "factory": "set_target_velocity_factory",
            #             "arguments": {
            #                 "target_velocity": {
            #                     "default": {
            #                         "vx": 0.0,
            #                         "vy": 0.0,
            #                         "wz": 0.0
            #                     }
            #                 }
            #             }
            #         },
            #     ]
            # }
        ]
    }
    
    
    
def test():
    print('TESTING>>>>')
    from inspect import signature
    def argument_info(func):
        sig = signature(func)
        sig_dict = {}
        for k, v in sig.parameters.items():
            sig_dict[k] = {
                "kind": str(v.kind),
                "name": v.name,
                "annotation": str(v.annotation.__name__) if v.annotation is not v.empty else "empty",
                "default": str(v.default) if v.default is not v.empty else "empty",
            }
        return sig_dict
    print(argument_info(load_map_factory()))    
    
if __name__ == '__main__':
    test()