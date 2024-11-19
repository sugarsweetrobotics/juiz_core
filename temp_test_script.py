#!/usr/bin/env python3
import sys, os
import cv2

sys.path.append(os.path.join("bindings", "pyjuiz"))

import juiz.proxy as proxy
from PIL.Image import Image

c = proxy.ContainerProxy("cv_container", "localhost", 8000)

assert(c.cv_container_capture() == 1)
img = c.cv_container_get_img()
img.save('test_output_image.png', 'PNG')
#cv2.imwrite("test_output_image.png", img)

