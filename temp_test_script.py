#!/usr/bin/env python3
import sys, os
import cv2

sys.path.append(os.path.join("bindings", "pyjuiz"))

import juiz.proxy as proxy
from PIL import Image

c = proxy.ContainerProxy("cv_container", "localhost", 8000)

def test_output_image(c):
    assert(c.cv_container_capture() == 1)
    img = c.cv_container_get_img()
    img.save('test_output_image.png', 'PNG')


def test_input_image(c):
    img = Image.open('test_input_image.png')
    a = c.cv_container_set_img(img)
    img = c.cv_container_get_img()
    img.save('test_output_image.png', 'PNG')


if __name__ == '__main__':
    test_input_image(c)