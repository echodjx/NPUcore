PROJECT_DIR := $(shell pwd)

MUSL_TOOLCHAIN_PREFIX := riscv64-linux-musl
MUSL_TOOLCHAIN_DIR := $(PROJECT_DIR)/$(MUSL_TOOLCHAIN_PREFIX)-cross/bin
MUSL_CC := $(MUSL_TOOLCHAIN_PREFIX)-gcc
MUSL_AR := $(MUSL_TOOLCHAIN_PREFIX)-ar
MUSL_OBJCOPY := $(MUSL_TOOLCHAIN_PREFIX)-objcopy

BASH_DIR := $(PROJECT_DIR)/bash-5.1.16
BASH := $(BASH_DIR)/bash

USER_DIR := $(PROJECT_DIR)/user
INITPROC_SRC := $(USER_DIR)/src/bin/initproc.rs
INITPROC := $(USER_DIR)/target/riscv64gc-unknown-none-elf/release/initproc

OS_DIR := $(PROJECT_DIR)/os
KERNEL := $(OS_DIR)/target/riscv64gc-unknown-none-elf/release/os.bin

export PATH := $(PATH):$(MUSL_TOOLCHAIN_DIR)

all: $(KERNEL)
	cp $(KERNEL) $(PROJECT_DIR)

$(INITPROC): $(INITPROC_SRC)
	cd $(USER_DIR) && make

$(BASH):
	cd $(BASH_DIR) && make 
	$(MUSL_OBJCOPY) --strip-debug $(BASH)

$(KERNEL): $(INITPROC) $(BASH)
	cd $(OS_DIR) && make comp BOARD=k210 COMP=true

