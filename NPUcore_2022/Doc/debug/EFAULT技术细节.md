经过对比NPUCore的运行日志与Debian上的strace，我们发现NPUCore和Debian对于`read(fd, NULL, 0)`的行为不一致，NPUCore返回了`EFAULT`，而Debian接受了`NULL`并正常返回。经过查阅Linux manual，我们发现对于`EFAULT`有两种解释：

`man 2 read`

> EFAULT: buf is outside your accessible address space.

`man 2 getcwd`

> EFAULT: buf points to a bad address.

这两句话在表述上模棱两可，什么是"accessible"，什么是"bad address"呢？我们在Debian上进行实验，得出以下结论：

1. EFAULT buf is outside your accessible address space.
返回EFAULT错误， 当且仅当如果buf指针对应的地址（这个地址甚至可以是NULL地址）在用户可用的地址空间（用户程序的页表）上（不论权限如何，或者说，对应页表项可以无相应读/写权限）
1. EFAULT buf points to a bad address.
返回EFAULT错误，当且仅当buf对应的地址在用户可用的地址空间（用户程序的页表）上且对应页表项有相应的读/写权限