TARGET := riscv64gc-unknown-none-elf

build: rust-user

rust-user: $(APPS)
	rustup target add $(TARGET)
	@cargo build --release

clean:
	@cargo clean

.PHONY: build rust-user clean
