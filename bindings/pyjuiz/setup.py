from setuptools import setup, find_packages

setup(
    name="juiz",
    version="0.0.1",
    install_requires=["yaml", "httpx", "pillow"],
    extras_require={
        #"develop": ["dev-packageA", "dev-packageB"]
    },
    packages=find_packages(),
)