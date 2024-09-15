
import cv2
import numpy as np
import math
import time

class TurtleSimMap:
    
    
    def __init__(self, map_metadata):
        if map_metadata is not None:
            self.load_map(map_metadata=map_metadata)
        
    def load_map(self, map_metadata: dict):
        # print(f'turtleSim.load_map({map_filepath})')
        map_filepath = map_metadata.get('map', None)
        if map_filepath is None:
            print('ERROR. map_metadata must contain "map" data for map filepath.')
        self.colored_map = cv2.imread(map_filepath)
        self.map = cv2.cvtColor(self.colored_map, cv2.COLOR_BGR2GRAY)
        self.map_backup = self.map.copy()
        self.map_metadata = map_metadata
        self.map_metadata['shape'] = self.map.shape
        self.scale_m_per_px = np.array([
            -map_metadata['width'] / self.map.shape[1],
            map_metadata['height'] / self.map.shape[0]
        ])
        self.map_metadata["scale_m_per_px"] = {
            "x": -self.scale_m_per_px[0],
            "y": self.scale_m_per_px[1],
        }
        self.origin_px = np.array([
            self.map_metadata["position_of_topleft"]["x"] / self.scale_m_per_px[0],
            self.map_metadata["position_of_topleft"]["y"] / self.scale_m_per_px[1]
        ])
        return self.map
    
    def _point_in_px(self, p_m):
        return (self.origin_px - p_m[0:2] / self.scale_m_per_px).astype(np.int32)

    def get_img(self):
        return self.map
    
    def get_colored_img(self):
        return self.colored_map
    
    def get_metadata(self):
        return self.map_metadata
    
    # def draw_lidar_on_map(self, pose, data):
    #     pose_px = self.point_in_px(pose["x"], pose["y"])
    #     for i, r in enumerate(data["ranges"]):
    #         angle = data["min_angle"] + data["angle_res"] * i + pose["th"]
    #         range_p_m = {
    #             "x": r * math.cos(angle) + pose["x"],
    #             "y": r * math.sin(angle) + pose["y"]
    #         }
    #         range_p_px = self.point_in_px(range_p_m["x"], range_p_m["y"])
    #         def on_plot(img, x, y):
    #             d = img[y, x]
    #             flag = False
    #             th = 127
    #             if len(d) == 3:
    #                 flag = d[0] < th and d[1] < th and d[2] < th
    #             else:
    #                 flag = d[0] < th
        
    #             if flag: #障害物
    #                 #return False
    #                 pass
    #             plot(img, x, y)
    #             return True
    #         bresenhams_line(self.map, (pose_px["x"], pose_px["y"]), (range_p_px["x"], range_p_px["y"]), on_plot=on_plot)
    #     # bresenhams_line(self.map, [250, 250], [400,250], on_plot=plot)
    #     pass
    
    def get_lidar_on_map(self, pose, init_data):
        pose_px = self._point_in_px(pose)
        angle = init_data["min_angle"] + pose[2]
        angle_res = init_data["angle_res"]
        def on_plot(img, x, y, th=127):
            return img[y, x] < th
        def get_range(r, angle_list=[angle]):
            angle = angle_list[0]
            range_p_px = self._point_in_px(r*np.array([np.cos(angle), np.sin(angle)]) + pose[0:2])
            angle_list[0] = angle + angle_res
            return bresenhams_line(self.map, pose_px, range_p_px, on_plot=on_plot, scale=self.scale_m_per_px[1], max_len=r)
        init_data["ranges"] = [get_range(r) for r in init_data["ranges"]]
        return init_data
bltime = 0.0
def plot(img, x, y):
    img[y, x] = [0, 0, 255]
    return True

def flip_plot(img, x, y):
    img[x, y] = [0, 2, 255]
    return True

def in_img(img, x, y):
    h, w= img.shape
    return x >= 0 and x < w and y >= 0 and y < h

def bresenhams_line(img, p0, p1, on_plot, scale, max_len):
    """_summary_

    Args:
        img (_type_): _description_
        p0 (_type_): _description_
        p1 (_type_): _description_
        on_plot (_type_): _description_

    Returns:
        int: Line length. If negative, line reaches to the edge of img.
    """
    x0, y0 = p0
    x1, y1 = p1
    dx = int(x1 - x0)
    dy = int(y1 - y0)
    if dx == 0:
        for y in range(y0, y1, -1 if dy < 0 else 1):
            if not in_img(img, x0, y):
                return max_len
            if on_plot(img, int(x0), int(y)):
                return abs(y-y0) * scale
        return max_len
    
    derror = abs(dy/dx)  # dx != 0
    if derror > 1.0: # flip mode
        _on_plot = lambda img, x, y: on_plot(img, y, x)  
        y0, x0 = p0
        y1, x1 = p1
        dx = int(x1 - x0)
        dy = int(y1 - y0)
        derror = abs(dy/dx)
    else:
        _on_plot = on_plot
        
    y = y0
    error = 0.0
    ay = (1 if dy > 0 else -1)
    for x in range(x0, x1, -1 if dx < 0 else 1):
        if not in_img(img, x, y):
            return max_len
        if _on_plot(img, x, y):
            return math.sqrt(abs(x - x0)**2 + abs(y - y0)**2) * scale
        error = error + derror
        if error >= 0.5:
            y = y + ay
            error = error - 1.0
            
    return max_len # math.sqrt(line_dx**2 + abs(y - y0)**2) * scale


def test():
    metadata = {
        "map": "map.png",
        "width": 10.0,
        "height": 10.0,
        "position_of_topleft": {
            "x": -5.0,
            "y": 5.0
        }
    }
    
    pose = np.array([-0.5, 0.5, 1.0])
    ranges = [10.0] * 1000
    lidar = {
        'min_angle': -1.5,
        'max_angle': 1.5,
        'angle_res':
            3 / len(ranges),
        'ranges': ranges
    }
    
    map = TurtleSimMap(metadata)
    dsum = 0.0
    bsum = 0.0
    import time
    global bltime
    try_times = 1
    for i in range(try_times):
        st = time.process_time()
        bltime = 0.0
        map.get_lidar_on_map(pose, lidar)
        d = time.process_time() - st
        dsum = dsum + d
    print('d:', dsum/ try_times * 1000, '[ms]')
    img = map.get_colored_img()
    cv2.imshow("windows", img)
    cv2.waitKey(-1)

if __name__ == '__main__':
    test()