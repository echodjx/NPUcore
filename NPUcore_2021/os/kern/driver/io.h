//
// Created by lumin on 2020/11/4.
//

#ifndef LAB8_IO_H
#define LAB8_IO_H

#include <defs.h>

#define UARTHS_IRQ          (0x0C200004U)

#define UARTHS_DATA_REG     (0x38000004U)
#define GPIOHS_BASE_ADDR    (0x38001000U)

#define SPI_SLAVE_BASE_ADDR (0x50240000U)
#define FPIOA_BASE_ADDR     (0x502B0000U)
#define SYSCTL_BASE_ADDR    (0x50440000U)
#define SPI0_BASE_ADDR      (0x52000000U)
#define SPI1_BASE_ADDR      (0x53000000U)
#define SPI3_BASE_ADDR      (0x54000000U)

#define IO_REGION_NUM       (4)
#define IO_REGION_START0    (0x0C200000U)
#define IO_REGION_END0      (0x0C201000U)
#define IO_REGION_START1    (0x38000000U)
#define IO_REGION_END1      (0x38002000U)
#define IO_REGION_START2    (0x50240000U)
#define IO_REGION_END2      (0x55000000U)
#define IO_REGION_START3    (0x80000000U)
#define IO_REGION_END3      (0x80000000U + PTSIZE)

typedef struct
{
    uintptr_t io_start;
    uintptr_t io_end;
    uintptr_t io_size;
} io_region_t;

#endif //LAB8_IO_H
