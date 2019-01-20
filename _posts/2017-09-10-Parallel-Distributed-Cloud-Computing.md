---
layout: post
title:  "Parallel Programming on Remote Servers with Ipyparallel"
date: 2017-09-10 12:00:00
author: Rohan Kotwani
excerpt: ""
tags: 
- Jupyter notebook
- Parallel Programming

---

### Table of Contents

1. Setting up remote engines
2. Starting remote engines


### Introduction

This post demostrates Jupyter notebook's and ipyparallel's capabilities to access remote servers for parallel programming.



#### My ipcluster_config.py file has the following:
    
    c.IPClusterEngines.engine_launcher = \
    'IPython.parallel.apps.launcher.SSHEngineSetLauncher'


    c.SSHEngineSetLauncher.engines = { 
        'host@xxx.xxx.xxx.x02' : (2, ['--profile_dir=/home/to/profile'])
        }
        
#### My ipcontroller_config.py file has the following:

    c.HubFactory.ip = '*'
    

#### upgrading some of your tools might fix the problem.

    pip install --upgrade ipython
    pip install --upgrade setuptools pip
    
    pip install  ipython[all]
    
#### The basic idea is that an sshserver arg to a Client is only for when the Controller is not directly accessible from the Client (remote location). Ssh tunnels are -required- when the machines are not accessible to each other.

First start the ipcontroller. (On MAC):

    (ipcontroller --profile=ssh --ip=* &)
    
Then start the ipcluster and ssh profile for the remote computer:

    (ipcluster start --profile=ssh &)
    
The remote clusters can also be stopped if the engines have died:

    (ipcluster stop --profile=ssh &)
    
Finally start local engines with correct parameters:

    (ipengine --profile=ssh &)

The security file might need to be shared from the controller to the engine:

    /Users/rohankotwani/.ipython/profile_ssh/security


{% highlight python %}
import ipyparallel as ipp
{% endhighlight %}


{% highlight python %}
c = ipp.Client(profile='ssh')
{% endhighlight %}


{% highlight python %}
c.ids
{% endhighlight %}




    [0, 1]




{% highlight python %}
c[:].apply_sync(lambda : "Hello, World")
{% endhighlight %}




    ['Hello, World', 'Hello, World']



### Steps to connect remotely to jupyter notebook

1. jupyter notebook --no-browser --port=8889 --ip=127.0.0.1 (remote host)
2. ssh -N -f -L 127.0.0.1:8889:127.0.0.1:8889 rohankotwani@probably-engine (local host)
3. http://127.0.0.1:8889/tree (local host web browser)
