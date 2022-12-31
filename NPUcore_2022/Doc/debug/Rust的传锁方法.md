对于操作系统来说，互斥锁是稀疏平常的，而rust语言机制就导致需要在函数入口参数处或是结构体里需要对锁保有一个调用入口，要不然无法对锁里面的东西进行访问，假设我们有一个锁：

```rust
fn foo()
{
    let lock: Mutex<usize> = Mutex::new(1);
    let mut locked: MutexGuard<usize> = lock.lock();
    *locked = 2;
    drop(locked);
}
```

在`lock`被上锁了之后`locked`就成为了能够访问`lock`内部具体内容的入口。

如果在函数内部直接调用是比较简单的，但是如果假设有一个函数调用，被调用函数需要`locked`的信息，所以需要对锁进行传递，我们在实现的初期是直接这么实现的：直接传递锁的所有权。

```rust
fn bar(locked: MutexGuard<usize>) -> MutexGuard<usize>{
    println!("{}", *locked);
    locked
}

fn foo() {
    let lock: Mutex<usize> = Mutex::new(1);
    let mut locked: MutexGuard<usize> = lock.lock();
    *locked = 2;
    let locked = bar(locked);
    drop(locked);
}
```

上面这种方法当然可行，不过非常麻烦的一点是需要经常需要一个变量接受返回的锁，这样写代码人都写麻掉，于是我们采取了另一种方式：直接传递引用不就行了？

```rust
fn bar(locked: &MutexGuard<usize>){
    println!("{}", *locked);
}

fn foo() {
    let lock: Mutex<usize> = Mutex::new(1);
    let mut locked: MutexGuard<usize> = lock.lock();
    *locked = 2;
    bar(&locked);
    drop(locked);
}
```

这样就能节省相当的代码量，这个问题不是很难，比较难的是当我们需要将锁存到一个结构体里，比如我们需要将锁的内容做成一个迭代器，这同样可以通过移动所有权的方式将锁的所有权传递到结构体里，不过由于rust声明周期的原因需要指定特定的生命周期：

```rust
struct Node<'a> {
    /// 需要指定生命周期保证locked和Node的生命周期一致
    pub locked: MutexGuard<'a, usize>
}

fn foo() {
    let lock : Mutex<usize> = Mutex::new(1);
    let locked = lock.lock();
    
    let mut lock_node = Node { locked };
    // 这样能够通过Node使用里面的lock
    *lock_node.locked = 2;
    // 如果需要将locked拿出来就可以直接拿，拿出来之后lock_node就无效了
    let locked = lock_node.locked;

    drop(locked);
}
```

这种方式确实可行，也保证锁的可控，但是下面这个情况会不可避免要在函数之间传递所有权：

```rust
struct Node<'a> {
    pub locked: MutexGuard<'a, usize>
}

fn bar(locked: MutexGuard<usize>) -> MutexGuard<usize>{
    let mut lock_node = Node { locked };
    *lock_node.locked = 2;
    
    let locked = lock_node.locked;
    locked
}

fn foo() {
    let lock: Mutex<usize> = Mutex::new(1);
    let locked: MutexGuard<usize> = lock.lock();

    let locked = bar(locked);

    drop(locked);
}
```

在函数之间传递所有权是不美的，这样会让开发变得相当困扰，在我们研究了一段时候发现实际上结构体里只需要存储对应的引用就可以了，这样就可以避免在函数之间传递锁的所有权：

```rust
struct Node<'a, 'b> {
    /// 值得注意的是引用的生命周期和锁的生命周期是不一样的，
    /// 所以需要多加一个参数进行区分。
    pub locked: &'a mut MutexGuard<'b, usize>
}

fn bar(locked: &mut MutexGuard<usize>){
    let lock_node = Node { locked };
    **lock_node.locked = 2;
}

fn foo() {
    let lock: Mutex<usize> = Mutex::new(1);
    let mut locked: MutexGuard<usize> = lock.lock();

    bar(&mut locked);

    drop(locked);
}
```

上述这种方式不需要传递所有权，只要顶层锁好，将锁的引用下放下面的函数就能方便使用，是一个不错的方法。