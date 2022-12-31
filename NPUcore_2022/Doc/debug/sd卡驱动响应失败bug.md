## 问题发现

在决赛第一阶段，将`NPUCore`放到k210开发板上测试时，在发生大量I/O情况时出现了执行失败的问题：

## 问题原因

这个问题实际上是因为在执行I/O时sbi总线忙于其他事，无法及时给出响应。

## 问题解决

让驱动支持容错，通过多次尝试等待sbi总线空闲：

```rust
fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        let lock = self.0.lock();
        let mut result = lock.read_sector(buf, block_id as u32);
        let mut cont_cnt = 0;
        while result.is_err() {
            if cont_cnt >= 0 {
                log::error!("[sdcard] read_sector(buf, {}) error. Retrying...", block_id);
                result = lock.read_sector(buf, block_id as u32);
            }
            cont_cnt += 1;
            if cont_cnt >= 5 {
                log::error!(
                    "[sdcard] read_sector(buf[{}], {}) error exceeded contineous retry count, waiting...",
                    buf.len(),
                    block_id
                );
                Self::wait_for_one_sec();
                if lock.read_sector(buf, block_id as u32).is_err() {
                    lock.init();
                    Self::wait_for_one_sec();
                } else {
                    break;
                }
                cont_cnt = 0;
            }
        }
    }
```

通过多次容错使得`NPUCore`在k210开发板上的稳定性提升。