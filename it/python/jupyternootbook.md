# 简介

> Free software, open standards, and web services for interactive computing across all programming languages.

https://jupyter.org/

https://ipython.readthedocs.io/en/stable/index.html



什么是 Jupyter Notebook？

> 按照 Jupyter 创始人 Fernando Pérez 的说法，他最初的梦想是做一个综合 Ju （Julia）、Py （Python）和 R 三种科学运算语言的计算工具平台，所以将其命名为 Ju-Py-te-R。



三大特点：

1、整合所有的资源

2、交互性编程体验

3、零成本重现结果



在线平台：

Jupyter 官方的 Binder 平台：

文档：https://mybinder.readthedocs.io/en/latest/index.html）

Google 提供的 Google Colab 环境：

文档：https://colab.research.google.com/notebooks/welcome.ipynb

它们让 Jupyter Notebook 变得和石墨文档、Google Doc 在线文档一样，在浏览器点开链接就能运行。



作为你实践的第一站。
第一个是 Jupyter 官方：

https://mybinder.org/v2/gh/binder-examples/matplotlib-versions/mpl-v2.0/?filepath=matplotlib_versions_demo.ipynb

第二个是 Google Research 提供的 Colab 环境，尤其适合机器学习的实践应用：
https://colab.research.google.com/notebooks/basic_features_overview.ipynb



安装：https://jupyter.org/install.html

运行：https://jupyter.readthedocs.io/en/latest/running.html#running



# 安装

Jupyter本地的两种安装办法：

1、安装Anconda。conda包含科学计算的几乎所有包，包含jupyter。
2、仅安装了Python。可以pip install ipython, jupyter。安装即可。

命令行启动jupyter：`jupyter notebook`



1、安装`Anaconda`或者`Miniconda`



```shell
# conda安装jupyter

# 创建新的虚拟环境
conda create -n my_env python=3.8 scikit-image=0.18 -y
# 查看新环境是否已经创建成功
conda env list

# 进入新创建的环境
conda activate my_env
# 安装Jupyter
pip install jupyter

# 新建一个notebook并且使用当前新建的环境时，发现没有当前新建环境的IPython内核。
# 在当前环境下建立新的IPython内核后，再次启动jupyter notebook发现已经有了当前环境下的IPython内核

# 在当前环境下建立新的IPython内核
# 安装ipykernel
pip install ipykernel
# 生成ipykernel的配置文件
python -m ipykernel install --name env_name
# 查看已有的kernel
jupyter kernelspec list
```





```shell
# 查看配置选项
jupyter notebook -h
jupyter notebook --help

配置文件配置

# 生成配置文件
jupyter notebook --generate-config
# 配置文件的位置
~/.jupyter                    # linux系统
C:\Users\<UserName>\.jupyter  # windows系统
# 配置文件，jupyter_notebook_config.py

# 在浏览器中自动打开notebook
jupyter notebook
```



# 快捷键



# 扩展

`A collection of various notebook extensions for Jupyter`

https://github.com/ipython-contrib/jupyter_contrib_nbextensions.git

扩展文档

https://jupyter-contrib-nbextensions.readthedocs.io/en/latest/

```shell
# 安装扩展包
pip install jupyter_contrib_nbextensions
# 安装 javascript和css文件
jupyter contrib nbextension install --user

# 启动juputer notebook，发现多了Nbextensions
```

几个比较有用的扩展

```
variable inspector
table of content
snippets
codefolding
autopep8
hide input
split cell notebook
zenmode
```



# 参考

https://blog.csdn.net/qq_40918859/article/details/125067935

https://blog.csdn.net/cainiao_python/article/details/125567913



https://www.bilibili.com/video/BV1Q4411H7fJ/?share_source=copy_web

