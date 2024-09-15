
import math, time
import traceback
import numpy as np

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

def fit_range(d, m):
    if d > m: return m
    elif d < -m: return -m
    return d

def normalize_angle(angle):
    while angle > math.pi:
        angle = angle - 2*math.pi
    while angle <= -math.pi:
        angle = angle + 2*math.pi
    return angle


class Turtle:
    def __init__(self, index, init_pose=(0,0,0)):
        # print('Turtle.__init__() called')
        self.index = index
        self.timestamp = time.time()
        self.accel = np.array((0.1, 0.1, 0.05))  # m/sec^2, rad/sec^2
        self.max_velocity = np.array((1.0, 1.0, 0.5))
        self.target_velocity = np.array((0, 0, 0))
        self.velocity = np.array((0, 0, 0))
        self.max_pose = np.array((20, 20))
        self.pose = np.array(init_pose)
        
    def profile(self):
        return {
            'pose': self.pose.tolist(),
            'velocity': self.velocity.tolist(),
            'accel': self.accel.tolist(),
            'max_velocity': self.max_velocity.tolist(),
            'target_velocity': self.target_velocity.tolist(),
        }
        
    def set_target_velocity(self, velocity: dict):
        # print(f'set_target_velocity(vel={velocity}) called')
        vel = [velocity['vx'], velocity['vy'], velocity['wz']]
        self.target_velocity=np.array([fit_range(v, m)for v,m in zip(vel,self.max_velocity)])
        return "OK"
        
    def set_pose(self, pose):
        x, y, th = pose
        mx, my = self.max_pose
        x = fit_range(x, mx)
        y = fit_range(y, my)
        th = normalize_angle(th)
        self.pose = np.array((x, y, th))
        
    def set_velocity(self, vel):
        # print(f'set_velocity({vel}) called')
        if len(vel) != 3:
            print("set_velocity failed. value length is not 3.")
            return
        self.velocity = np.array([fit_range(v,m) for v,m in zip(vel, self.max_velocity)])
        
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