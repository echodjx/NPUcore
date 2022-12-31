# 信号

## 简介

信号是Linux下常见的跨进程通信机制，特别是在父子进程之间应用很多，这篇文档用于教怎么从零开始搭建一套简易信号系统。

## 相关系统调用

有关的系统调用有4个,分别是:
+ `kill`,  负责给进程传递信号 
+ `sigaction`, 负责设定特定信号处理的方式  
+ `sigprocmask`, 负责建立特定进程的"信号掩码",从而过滤并忽略特定的信号  
+ `sigreturn`, 负责调用完成自定义信号处理函数后返回  

## 机理

信号是在软件层次上对中断机制的一种模拟。在原理上，一个进程收到一个信号与处理器收到一个中断请求可以说是一样的。信号是异步的，一个进程不必通过任何操作来等待信号的到达，事实上，进程也不知道信号到底什么时候到达。信号可以直接进行用户空间进程和内核进程之间的交互，内核进程也可以利用它来通知用户空间进程发生了哪些系统事件。信号因此可以处理进程间异步通信。

### 信号的发出

当特定时间发生,信号发出。其对应事件是系统硬编码且不可更改的,具体内容可见于manpage: signal(2)部分，或者本操作系统的function reference。

当进程发出信号的时候会调用`kill`系统调用，这个时候内核处理`kill`时会讲对应进程的信号置位，信号量的维护放在TCB里，在NPUCore里它用一个叫`SigInfo`的数据结构维护。

```rust
pub struct TaskControlBlockInner {
    pub siginfo: SigInfo,
}
```

`SigInfo`这个数据结构如下：

```rust
pub struct SigInfo {
    pub signal_pending: Signals,
    pub signal_handler: BTreeMap<Signals, SigAction>,
}
```

+ `signal_pending` 维护信号标志位，（这里手册上说是有1024位，但是目前从简单实现的角度上考虑只留了64位，一般程序64位就很够用了）
+ `signal_handler` 维护用户自定义处理函数

内核在处理`kill`系统调用时会置位进程的`signal_pending`的信号位，然后还需要判断进程是否在睡眠状态，如果在则会唤醒进程。

```rust
let mut inner = task.acquire_inner_lock();
inner.add_signal(signal);
// wake up target process if it is sleeping
if inner.task_status == TaskStatus::Interruptible {
    inner.task_status = TaskStatus::Ready;
    wake_interruptible(task.clone());
}
```

### 信号的接收

当内核发生调度，进程在返回用户态前会执行`do_signal`函数。该函数用于检测进程的signal，然后执行相应的处理函数。如果其他进程有往该进程发送过signal，该进程处理，这样就完成了一次进程间异步通讯。

```rust
pub fn trap_return() -> ! {
    do_signal();
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,
            in("a1") user_satp,
            options(noreturn)
        );
    }
}
```

### 信号掩码

在TCB中，需要存放一个信号掩码过滤并忽略特定的信号，当位被置1时对应信号会被忽略掉。

```rust
pub struct TaskControlBlockInner {
    pub sigmask: Signals,
}
```

这个值可以被`sigprocmask`系统调用修改，`sigprocmask`系统调用的定义如下：

```rust
pub fn sigprocmask(how: u32, set: *const Signals, oldset: *mut Signals) -> isize
```

+ `how` 决定执行的动作
+ `set` 指针，如果非0则是用户准备好的`Signal`
+ `oldset` 指针，如果非0则要进程的`sigmask`写入

根据手册，`how`有三种形式

0. 将进程的`sigmask`添加`set`的标志位
1. 将进程的`sigmask`删去`set`的标志位
2. 将进程的`sigmask`赋值为`set`

根据手册，这四个信号是不允许被忽略，即他们的标志位永远不可能为1。

+ `SIGILL` 非法指令，进程应该被终止掉
+ `SIGSEGV` 非法访存，进程应该被终止掉
+ `SIGKILL` 进程销毁信号，进程应该被终止掉
+ `SIGSTOP` 进程停止信号，进程应给被阻塞掉

所以这个系统调用的核心步骤如下：

1. 如果`oldset`指向非0，则将进程的`sigmask`写入
2. 如果`set`指向非0，根据`how`对进程`sigmask`进行相应的处理
3. 清除进程`sigmask`中的四个特殊标志位

### 默认处理函数

`do_signal`这个函数实际上由两部分组成，一部分是默认处理函数，一部分是自定义处理函数。

默认函数处理起来比较容易，照着手册上的语义做就可以了，实现下来会有三种处理方式：

1. 销毁进程
2. 阻塞进程
3. 继续执行进程

这里需要稍微强调一下继续执行进程这个的意义，如果没有信号，我们会发现当一个进程执行wait后不知道什么时候该唤醒。唤醒必须由它的子进程来做，子进程向父进程传递一个信号用于唤醒父进程接着执行。

### 自定义处理函数

Ok. 这里是信号处理的重中之重,我们先介绍一下其基本原理和内容. Linux允许用户替换信号的处理方式，用户只需要准备好相关的处理函数，调用`sigaction`系统调用替换信号处理方式。

在讲系统调用之前需要先回过头讲一下`signal_handler`有关的数据结构，它维护的是一些`Signal`与`SigAction`的映射，根据手册，`SigAction`的定义如下：

```rust
pub struct SigAction {
    pub handler: SigHandler,
    pub flags: SigActionFlags,
    pub restorer: usize,
    pub mask: Signals,
}
```

出于简单实现的考虑，我们仅需要考虑`handler`这个量，它是一个指向处理函数的指针，我们可以先将`SigAction`视为存放函数指针的结构体。

`sigaction`系统调用的定义如下：

```rust
pub fn sigaction(signum: usize, act: *const SigAction, oldact: *mut SigAction) -> isize
```

+ `signum` 需要自定义的信号标志位
+ `act` 指针，如果非0则是用户准备好的`SigAction`
+ `oldact` 指针，如果非0则要进程对应标志位的`SigAction`写入

这里我们可以简化实现，假设对于每个信号有且只会定义一次自定义处理。不会销毁定义，这样`SigActionHandler`的0和1的情况暂且不需要处理。

这样只需要每次考虑往`signal_handler`里面添加对应的`SigAction`。不过有一点任需注意：`SIGILL`, `SIGSEGV`, `SIGKILL`, `SIGSTOP`四个特殊信号不能由用户自定义。

所以这个系统调用实现如下：

```rust
pub fn sigaction(signum: usize, act: *const SigAction, oldact: *mut SigAction) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    let result = Signals::from_signum(signum);
    match result {
        Err(_) | Ok(Some(Signals::SIGKILL)) | Ok(Some(Signals::SIGSTOP)) | Ok(None) => {
            warn!("[sigaction] bad signum: {}", signum);
            EINVAL
        }
        Ok(Some(signal)) => {
            trace!("[sigaction] signal: {:?}", signal);
            let token = inner.get_user_token();
            if oldact as usize != 0 {
                if let Some(sigact) = inner.siginfo.signal_handler.remove(&signal) {
                    copy_to_user(token, &sigact, oldact);
                    trace!("[sigaction] *oldact: {:?}", sigact);
                } else {
                    copy_to_user(token, &SigAction::new(), oldact);
                    trace!("[sigaction] *oldact: not found");
                }
            }
            if act as usize != 0 {
                let sigact = &mut SigAction::new();
                copy_from_user(token, act, sigact);
                sigact.mask.remove(
                    Signals::SIGILL | Signals::SIGSEGV | Signals::SIGKILL | Signals::SIGSTOP,
                );
                // push to PCB, ignore mask and flags now
                if !(sigact.handler == SigActionHandler::SIG_DFL
                    || sigact.handler == SigActionHandler::SIG_IGN)
                {
                    inner.siginfo.signal_handler.insert(signal, *sigact);
                };
                trace!("[sigaction] *act: {:?}", sigact);
            }
            SUCCESS
        }
    }
}
```

用户设定完自定义函数之后，执行do_signal函数时需先判断要处理的信号是否被用户自定义处理过，如果是，则加载用户的自定义处理函数，不是则加载系统默认处理函数。

这会涉及到两个问题，自定义函数的加载和返回内核态。

返回内核态可以利用系统调用，`sigreturn`系统调用的作用类似自定义信号处理函数执行完后的内核接口。在用户态下用户是不可能通过sp(函数返回地址寄存器)来实现系统调用，所以借鉴trampoline的思想在用户态的指定位置存放一段汇编代码，汇编代码的功能是执行系统调用。 

```r
__call_sigreturn:
    # ecall sys_sigreturn
    addi	a7, zero, 139
    ecall
```

这里将`__call_sigreturn`作为自定义函数的返回地址即可完成返回内核态。

自定义函数的加载逻辑就显得有点麻烦了，我们设计了一个单独的trampoline用于signal的自定义处理函数.这里留一个问题,一会儿揭晓答案:
为什么不能沿用原来的Trampoline, 非要设计出单独的trampoline呢? 我们先继续看下去.

首先，可能对于一系列信号都有它自己的自定义处理，导致可能先要处理一系列信号处理后才最终回到用户态被中断的程序。

我们注意到原来从用户态进出内核态本质上是一个压栈弹栈的过程，进内核态将寄存器压入用户栈，退内核态将寄存器从用户栈中弹出。

这一过程给了我们启发，既然出内核态都要弹栈，为什么不在内核态预先在用户态栈“压”好一个`trap_context`，然后退出内核时自然就跳到自定义函数的执行入口呢?

那么根据标准，这个`trap_context`需要注意以下几点：

1. `a0`寄存器（第一个形参）存放信号参数
2. `ra`寄存器（返回地址）存放`__call_sigreturn`的地址
3. `spec`寄存器（pc）存放`handler`
4. `sp`寄存器需要记得及时修改

```rust
let trap_cx = inner.get_trap_cx();
let sp = unsafe { (trap_cx.x[2] as *mut TrapContext).sub(1) };
if (sp as usize) < USER_STACK_TOP {
    trap_cx.sepc = usize::MAX; // we don't have enough space on user stack, return a bad address to kill this program
} else {
    copy_to_user(inner.get_user_token(), trap_cx, sp as *mut TrapContext); // push trap context into user stack
    trap_cx.set_sp(sp as usize); // update sp, because we've pushed something into stack
    trap_cx.x[10] = signal.to_signum().unwrap(); // a0 <- signum, parameter.
    trap_cx.x[1] = SIGNAL_TRAMPOLINE; // ra <- __call_sigreturn, when handler ret, we will go to __call_sigreturn
    trap_cx.sepc = act.handler.addr().unwrap(); // restore pc with addr of handler
}
```

当调用`sigreturn`时，将trap_context从用户栈拿回来。

```rust
pub fn sys_sigreturn() -> isize {
    // mark not processing signal handler
    let current_task = current_task().unwrap();
    info!("[sys_sigreturn] pid: {}", current_task.pid.0);
    let inner = current_task.acquire_inner_lock();
    // restore trap_cx
    let trap_cx = inner.get_trap_cx();
    let sp = trap_cx.x[2];
    copy_from_user(inner.get_user_token(), sp as *const TrapContext, trap_cx);
    return trap_cx.x[10] as isize; //return a0: not modify any of trap_cx
}
```

考虑原来的trampoline的Map方式,是在用户页表上出现,但是没有U权限的一页,其中的内容都是跳转相关的代码,包括页表切换,因为提权操作是单独的指令,执行后页表并没有直接切换,而是跳到跳板进行下一个指令,这时,具有了S权限的处理器才可以开始保存现场,切换页表等等操作.后续的操作都是发生在S态的内核程序,这使得这种设计十分合适.

而单独的signal_trampoline则不同,自定义 signal_handler 需要执行的不是内核中的代码,而是用户程序中的代码,因此是使用用户级权限执行,执行后跳转到`sigreturn`系统调用回到内核态,然后我们应该将其进行常见的上下文恢复回到原地继续执行,这里的代码和trampoline不同,所需的权限也不一样(用户级), 因此不能沿用原来的跳板

显然,这样的要求决定了,不论是Trampoline还是signal_trampoline,其都必须占用一个单独的页.

这样就完成了自定义函数的设计。
