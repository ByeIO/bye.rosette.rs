from setuptools import setup, find_packages

VERSION = '0.0.1' 
DESCRIPTION = 'rosette_py绑定'
LONG_DESCRIPTION = 'rosette机器人框架的python绑定'

# 配置
setup(
       # 名称必须匹配文件名 'verysimplemodule'
        name="rosette_py", 
        version=VERSION,
        author="qsbye",
        author_email="<2557877116@qq.com>",
        description=DESCRIPTION,
        long_description=LONG_DESCRIPTION,
        packages=find_packages(),
        # add any additional packages that 
        install_requires=[], 
        # 需要和你的包一起安装，例如：'caer'
        
        keywords=['python', 'ROS', 'rosette'],
        classifiers= [
            "Development Status :: 3 - Alpha",
            "Intended Audience :: Education",
            "Programming Language :: Python :: 3",
            "Operating System :: MacOS :: MacOS X",
            "Operating System :: Microsoft :: Windows",
        ]
)
