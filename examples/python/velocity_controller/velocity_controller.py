
def velocity_controller(velocity):
    print(f'velocity_controller({velocity}) called')
    return velocity

def manifest():
    return {
        "type_name": "velocity_controller",
        "arguments" : {
            "velocity": {
                "description": "test_argument",
                "default": {
                    "vx": 0.0, "vy": 0.0, "wz": 0.0},
            }
        }, 
    }

def process_factory():
    return velocity_controller