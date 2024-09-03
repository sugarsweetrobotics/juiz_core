import time, math, traceback
import cv2


turtle_sim = None

class TurtleSim(object):
    def __init__(self):
        self.map = None
        pass
    
    def load_map(self, map_filepath):
        print(f'turtleSim.load_map({map_filepath})')
        self.map = cv2.imread(map_filepath)
        return self.map
        
def turtle_sim(manifest):
    global turtle_sim
    turtle_sim = TurtleSim()
    return turtle_sim

def load_map(turtleSim: TurtleSim, map_filepath):
    return turtleSim.load_map(map_filepath)
    
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
    def __init__(self):
        # print('Turtle.__init__() called')
        self.timestamp = time.time()
        self.accel = (0.1, 0.1, 0.05) # m/sec^2, rad/sec^2
        self.max_velocity = (1.0, 1.0, 0.5)
        self.target_velocity = (0, 0, 0)
        self.velocity = (0, 0, 0)
        self.max_pose = (20, 20)
        self.pose = (0, 0, 0)
        
    def set_target_velocity(self, vel):
        # print(f'set_target_velocity(vel={vel}) called')
        self.target_velocity=[fit_range(v,m)for v,m in zip(vel,self.max_velocity)]
        #vx, vy, wz = vel
        #mx, my, mz = self.max_velocity
        #vx = fit_range(vx, mx)
        #vy = fit_range(vy, my)
        #wz = fit_range(wz, mz)
        #self.target_velocity = (vx, vy, wz)
        
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
        #print('set_velocity result: ', self.velocity)
        #vx, vy, wz = vel
        #mx, my, mz = self.max_velocity
        #vx = fit_range(vx, mx)
        #vy = fit_range(vy, my)
        #wz = fit_range(wz, mz)
        #self.velocity = (vx, vy, wz)
        
    def update_velocity(self, dt: float):
        #print(f'update_velocity({self.velocity}, {self.accel}, {self.target_velocity}) called')
        self.set_velocity( [add_acc(v,a,dt,t) for v,a,t in zip(self.velocity,self.accel,self.target_velocity)] )
        #print('update_velocity() exit')
        #vx, vy, wz = self.velocity
        #ax, ay, az = self.accel
        #tx, ty, tz = self.target_velocity
        #vx = add_acc(vx, ax, dt, tx)
        #vy = add_acc(vy, ay, dt, ty)
        #wz = add_acc(wz, az, dt, tz)
        #self.velocity = (vx, vy, wz)
        
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
        
        
def turtle(manifest):
    return Turtle()

def update(turtle: Turtle):
    # print('turtle_py.update() called')
    try:
        previous_time = turtle.timestamp
        turtle.timestamp = time.time()
        duration = turtle.timestamp - previous_time
        turtle.update_velocity(duration)
        turtle.update_pose(duration)
        return turtle.timestamp
    except:
        traceback.print_exc()
        
def update_factory(manifest):
    print("tutle_py.update_factory() called")
    return update

def increment(pycomp, arg0):
    pycomp.buf = pycomp.buf + arg0
    return pycomp.buf

def get_pose(turtle):
    print("tutle_py.get_pose() called")
    return (turtle.timestamp, turtle.pose)

def get_pose_factory(manifest):
    print("tutle_py.get_pose_factory() called")
    return get_pose

def set_target_velocity(turtle: Turtle, target_velocity):
    print('turtle_py.set_target_velocity({target_velocity}) called')
    v = (target_velocity["vx"],  target_velocity["vy"],  target_velocity["wz"])
    turtle.set_target_velocity(v)
    
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
                    }
                ]  
            },
            {
                "type_name": "turtle",
                "factory": "turtle_manifest",
                "processes": [
                    {
                        "type_name": "get_pose",
                        "factory": "get_pose_factory",
                        "arguments": {
                        }
                    },
                    {
                        "type_name": "update",
                        "factory": "update_factory",
                        "arguments": {
                        }
                    },
                    {
                        "type_name": "set_target_velocity",
                        "factory": "set_target_velocity_factory",
                        "arguments": {
                            "target_velocity": {
                                "default": {
                                    "vx": 0.0,
                                    "vy": 0.0,
                                    "wz": 0.0
                                }
                            }
                        }
                    },
                ]
            }
        ]
    }