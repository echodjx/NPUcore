
# Table of Contents

1.  [Futex](#org357e82a)
    1.  [NPUcore的多线程](#org7ffa2f5)
    2.  [支持的命令](#orgadfda83)
        1.  [wait](#orgb4cb379)
        2.  [wake](#org3e92c3b)
        3.  [requeue](#org89fe773)
    3.  [实现方式](#org499d430)
        1.  [数据结构](#org810146f)
        2.  [唤醒检测时机](#org671ce8e)
        3.  [Timeout](#orgb00a923)


<a id="org357e82a"></a>

# Futex

Futex本身是快速用户态互斥锁的英文缩写, 是Linux提供的一种通过共享内存地址进行线程/进程间同步和互斥的一种机制. 


<a id="org7ffa2f5"></a>

## NPUcore的多线程

和Linux一样, NPUcore使用进程来实现线程, 所以线程本质上只是共享内存空间的多个进程. 我们只支持进程内Futex.


<a id="orgadfda83"></a>

## 支持的命令

目前我们的Futex支持的命令只有3个, 分别是: wait, wake, requeue.


<a id="orgb4cb379"></a>

### wait

首先检测待共享内存地址uaddr处的内存是否为val,否则报错EAGAIN,是则阻塞等待当前线程其他Futex命令(如wake/requeue)唤醒或者等待超时.

如果额外提供了timeout, 则其等待超过timeout结构体中的时间就结束等待, 否则就一直等待下去知道其他唤醒条件满足


<a id="org3e92c3b"></a>

### wake

wake则是唤醒等待共享地址uaddr的所有进程中的最多val个,并返回唤醒的数量


<a id="org89fe773"></a>

### requeue

也是唤醒 `uaddr`, 只不过最多唤醒 `val` 个, 如果超出, 就将其余的等待进程从该地址的 `futex` 中转移到另一个地址 `uaddr2` 中


<a id="org499d430"></a>

## 实现方式

事实上, 上面的说明实际上是对实现提供了比较清晰的说明, 我们只补充额外的信息

首先, 我们使用等待队列实现Futex. 这样的好处在于等待队列可以通过异步的方式最大程度上利用等待的时间进行任务的调度, 将时间分配给其他的计算和等待任务并尽早返回和唤醒. 


<a id="org810146f"></a>

### 数据结构

具体来说, 我们给所有进程存指向全局Futex管理器结构的指针, 其中包括了一个BTree, 保存着所有的地址到其对应等待队列的映射. 

那么这种实验就很好处理了, wait就是加入等待队列的操作, 如果没有就创建一个BTree节点, wait就是加入进程到队列, wake就是取出并唤醒进程, requeue按照上述说法检测并唤醒或移动即可.


<a id="org671ce8e"></a>

### 唤醒检测时机

显然在WAIT的开始是需要检测的.

此外, `set_tid_address`有一个要求,如果其`clear_child_tid`不为0, 则需要地址的变量变化检测在`do_exit`, 也就是退出之前,进行futex的检测并唤醒一个进程.这一点需要注意.


<a id="orgb00a923"></a>

### Timeout

Timeout同样是用等待队列实现, 在wait的时候, 将其加入睡眠的等待队列即可.

此外,由于我们的系统在实现的时候如果对已经是就绪态/运行态的进程进行唤醒不会有任何的问题, 所以实际上即使是唤醒也不用管其Futex是否已经满足其他唤醒条件

