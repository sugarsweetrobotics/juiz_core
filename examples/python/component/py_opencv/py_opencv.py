import cv2
import numpy as np
import juiz

from PIL import Image
class CvContainer:
    def __init__(self, camera_id):
        self._capture = cv2.VideoCapture(camera_id)
            
        _ = self._capture.read()
        _ = self._capture.read()
        self._img = None

@juiz.juiz_component_container    
def cv_container(camera_id:int=0):
    return CvContainer(camera_id)

def test_cv_container(cam_id=0):
    cv = cv_container(cam_id)

@juiz.juiz_component_container_process( container_type="cv_container" )
def cv_container_capture(component):
    ret, component._img = component._capture.read()
    return ret

def test_cv_container_capture(cam_id=0):
    cv = cv_container(cam_id)
    assert(cv_container_capture(cv) == True)
    cv2.imwrite("./test_cv_container_capture_output.png", cv._img)
    
@juiz.juiz_component_container_process( container_type="cv_container" )
def cv_container_get_img(component):
    pil_image = Image.fromarray(component._img)
    return pil_image

@juiz.juiz_component_container_process( container_type="cv_container" )
def cv_container_set_img(component, img: Image.Image):
    component._img = np.array(img)
    return True

@juiz.juiz_component_container_process( container_type="cv_container" )
def cv_container_save_img(component, filename: str):
    cv2.imwrite("filename", component._img)
    return True

def component_manifest():
    return {
        "type_name": "py_opencv",
        "containers": [
            {
                "type_name": "cv_container",
                "processes": [
                    {
                        "type_name": "cv_container_capture"
                    },
                    {
                        "type_name": "cv_container_get_img"
                    },
                    {
                        "type_name": "cv_container_set_img"
                    },
                    {
                        "type_name": "cv_container_save_img"
                    }
                ]
            }
        ]
    }