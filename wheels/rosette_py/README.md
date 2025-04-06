# rosette_py包
rosette机器人框架(rust)的python绑定(binding).

## 使用说明


## 开发说明
### 文件介绍
```sh
- __init__.py : 模块入口(类似mod.rs)
- setup.py : 包配置文件(类似Cargo.toml)
- pyproject.toml : 包依赖文件
```

### 编译
```sh
python setup.py sdist bdist_wheel
```
