#!/usr/bin/env python3
import juiz

@juiz.juiz_process
def talker_python():
    print('talker_python says "Hello, World!"')
    return "Hello, World!"

