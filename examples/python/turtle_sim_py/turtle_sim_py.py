import time, math, traceback, typing
import cv2
import typing


turtle_sim = None

class TurtleSim(object):
    def __init__(self, map_filepath=None):
        print('map_filepath=', map_filepath)
        self.timestamp = time.time()
        self.turtles = [] # typing.List[Turtle]
        if map_filepath:
            self.load_map(map_filepath)
        else:
            self.map = None
        self.spawn_turtle(0, 0, 0)
        pass
    
    
    def load_map(self, map_filepath):
        print(f'turtleSim.load_map({map_filepath})')
        self.map = cv2.imread(map_filepath)
        return self.map
    
    def get_map(self):
        print('get_map:', self.map)
        return self.map
    
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
    
    def set_turtle_target_velocity(self, index, velocity):
        print('set_turtle_target_velocity', index, velocity)
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
        return {
            'min_angle': -1.0,
            'max_angle': 1.0,
            'angle_res':
                0.5,
            'ranges': [20.0, 10.0, 10.0, 10.0, 10.0]
        }
            
        
def turtle_sim(manifest):
    map_filepath = manifest.get('map_filepath', None)
    global turtle_sim
    turtle_sim = TurtleSim(map_filepath=map_filepath)
    
    return turtle_sim

def load_map(turtleSim: TurtleSim, map_filepath):
    return turtleSim.load_map(map_filepath)
    
def load_map_factory(m=None):
    print(f'load_map_factory({m})')
    return load_map

def get_map(turtleSim: TurtleSim):
    return turtleSim.get_map()
    
def get_map_factory():
    return get_map

def get_turtle_lidar_factory():
    return lambda sim, index: sim.get_turtle_lidar(index)

def spawn_turtle(turtleSim: TurtleSim, init_pose):
    return turtleSim.spawn_turtle(init_pose['x'], init_pose['y'], init_pose['th'])

def spawn_turtle_factory():
    return spawn_turtle

def get_turtle_pose(turtleSim: TurtleSim, index):
    return turtleSim.get_turtle_pose(index)

def get_turtle_pose_factory():
    return get_turtle_pose
    

def set_turtle_target_velocity_factory():
    def set_turtle_target_velocity(turtleSim: TurtleSim, index: int, velocity: dict):
        return turtleSim.set_turtle_target_velocity(index, velocity)
    return set_turtle_target_velocity

    
def update_factory():    
    def update(turtleSim: TurtleSim):
        return turtleSim.update()
    return update

def get_profile(turtleSim: TurtleSim) :
    return turtleSim.get_profile()
    
def get_profile_factory():
    return get_profile


def normalize_angle(angle):
    while angle > math.pi:
        angle = angle - 2*math.pi
    while angle <= -math.pi:
        angle = angle + 2*math.pi
    return angle

def add_acc(current_vel, acc, dt, tgt_vel, epsilon=0.01):
    dv = tgt_vel - current_vel
    da = acc * dt
    vel = 0
    if dv > 0:
        if dv > da:
            vel = current_vel + da
        else:
            vel = tgt_vel
    elif dv < -0:
        if dv < -da:
            vel = current_vel - da
        else:
            vel = tgt_vel
    else:
        vel = current_vel
    return vel

def sign(a):
    if a < 0: return -1
    return +1

def fit_range(d, m):
    if d > m: return m
    elif d < -m: return -m
    return d

class Turtle:
    def __init__(self, index, init_pose=(0,0,0)):
        # print('Turtle.__init__() called')
        self.index = index
        self.timestamp = time.time()
        self.accel = (0.1, 0.1, 0.05) # m/sec^2, rad/sec^2
        self.max_velocity = (1.0, 1.0, 0.5)
        self.target_velocity = (0, 0, 0)
        self.velocity = (0, 0, 0)
        self.max_pose = (20, 20)
        self.pose = init_pose
        
    def profile(self):
        return {
            'pose': self.pose,
            'velocity': self.velocity,
            'accel': self.accel,
            'max_velocity': self.max_velocity,
            'target_velocity': self.target_velocity,
        }
        
    def set_target_velocity(self, velocity: dict):
        # print(f'set_target_velocity(vel={velocity}) called')
        vel = [velocity['vx'], velocity['vy'], velocity['wz']]
        self.target_velocity=[fit_range(v, m)for v,m in zip(vel,self.max_velocity)]
        return "OK"
        
    def set_pose(self, pose):
        x, y, th = pose
        mx, my = self.max_pose
        x = fit_range(x, mx)
        y = fit_range(y, my)
        th = normalize_angle(th)
        self.pose = (x, y, th)
        
    def set_velocity(self, vel):
        # print(f'set_velocity({vel}) called')
        if len(vel) != 3:
            print("set_velocity failed. value length is not 3.")
            return
        self.velocity = [fit_range(v,m) for v,m in zip(vel, self.max_velocity)]
        
    def update_velocity(self, dt: float):
        #print(f'update_velocity({self.velocity}, {self.accel}, {self.target_velocity}) called')
        self.set_velocity( [add_acc(v,a,dt,t) for v,a,t in zip(self.velocity,self.accel,self.target_velocity)] )

    def update_pose(self, dt: float):
        # print(f'update_pose({self.pose}, {self.velocity})')
        x, y, th = self.pose
        vx, vy, wz = self.velocity
        ddx = vx * dt
        ddy = vy * dt
        ddth = wz * dt
        cth = math.cos(th + ddth / 2)
        sth = math.sin(th + ddth / 2)
        dx = ddx * cth - ddy * sth
        dy = ddx * sth + ddy * cth
        dth = ddth
        x = x + dx
        y = y + dy
        th = normalize_angle(th + dth)
        self.set_pose((x,y,th))
        
    def update(self, dt: float):
        # print(f'turtle_py.update({dt}) called')
        try:
            previous_time = self.timestamp
            self.timestamp = time.time()
            duration = self.timestamp - previous_time
            self.update_velocity(duration)
            self.update_pose(duration)
            return self.timestamp
        except:
            traceback.print_exc()
        
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
    
def component_profile():
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
                            "map_filepath": {
                                "default": "map.png"
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