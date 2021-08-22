//
// Created by lumin on 2020/11/3.
//

#ifndef LAB8_GPIOHS_H
#define LAB8_GPIOHS_H

#include <defs.h>
#include <assert.h>
#include <fpioa.h>

#define GPIOHS_MAX_PINNO 32

typedef enum
{
    GPIO_PV_LOW,
    GPIO_PV_HIGH
} gpio_pin_value_t;

/**
 * drive mode(work mode) of specific gpio pin
 */
typedef enum
{
    GPIO_DM_INPUT,              // as input
    GPIO_DM_INPUT_PULL_UP,      // as input with pull up resistance
    GPIO_DM_INPUT_PULL_DOWN,    // as input with pull down resistance
    GPIO_DM_OUTPUT,             // as output
} gpio_drive_mode_t;

/**
 * @brief       GPIO bits object
 */
typedef struct
{
    uint32_t b0 : 1;
    uint32_t b1 : 1;
    uint32_t b2 : 1;
    uint32_t b3 : 1;
    uint32_t b4 : 1;
    uint32_t b5 : 1;
    uint32_t b6 : 1;
    uint32_t b7 : 1;
    uint32_t b8 : 1;
    uint32_t b9 : 1;
    uint32_t b10 : 1;
    uint32_t b11 : 1;
    uint32_t b12 : 1;
    uint32_t b13 : 1;
    uint32_t b14 : 1;
    uint32_t b15 : 1;
    uint32_t b16 : 1;
    uint32_t b17 : 1;
    uint32_t b18 : 1;
    uint32_t b19 : 1;
    uint32_t b20 : 1;
    uint32_t b21 : 1;
    uint32_t b22 : 1;
    uint32_t b23 : 1;
    uint32_t b24 : 1;
    uint32_t b25 : 1;
    uint32_t b26 : 1;
    uint32_t b27 : 1;
    uint32_t b28 : 1;
    uint32_t b29 : 1;
    uint32_t b30 : 1;
    uint32_t b31 : 1;
} __attribute__((packed, aligned(4))) gpiohs_bits_t;

/**
 * @brief       GPIO bits multi access union
 */
typedef union
{
    /* 32x1 bit mode */
    uint32_t u32[1];
    /* 16x2 bit mode */
    uint16_t u16[2];
    /* 8x4 bit mode */
    uint8_t u8[4];
    /* 1 bit mode */
    gpiohs_bits_t bits;
} __attribute__((packed, aligned(4))) gpiohs_u32_t;

/**
 * @brief      GPIO object
 *
 *             The GPIO controller is a peripheral device mapped in the
 *             internal memory map, discoverable in the Configuration String.
 *             It is responsible for low-level configuration of the actual
 *             GPIO pads on the device (direction, pull up-enable, and drive
 *             value), as well as selecting between various sources of the
 *             controls for these signals. The GPIO controller allows seperate
 *             configuration of each of N GPIO bits.
 *
 *             Once the interrupt is pending, it will remain set until a 1 is
 *             written to the *_ip register at that bit.
 */
typedef struct
{
    /* Address offset 0x00, Input Values */
    gpiohs_u32_t input_val;
    /* Address offset 0x04, Input enable */
    gpiohs_u32_t input_en;
    /* Address offset 0x08, Output enable */
    gpiohs_u32_t output_en;
    /* Address offset 0x0c, Onput Values */
    gpiohs_u32_t output_val;
    /* Address offset 0x10, Internal Pull-Ups enable */
    gpiohs_u32_t pullup_en;
    /* Address offset 0x14, Drive Strength */
    gpiohs_u32_t drive;
    /* Address offset 0x18, Rise interrupt enable */
    gpiohs_u32_t rise_ie;
    /* Address offset 0x1c, Rise interrupt pending */
    gpiohs_u32_t rise_ip;
    /* Address offset 0x20, Fall interrupt enable */
    gpiohs_u32_t fall_ie;
    /* Address offset 0x24, Fall interrupt pending */
    gpiohs_u32_t fall_ip;
    /* Address offset 0x28, High interrupt enable */
    gpiohs_u32_t high_ie;
    /* Address offset 0x2c, High interrupt pending */
    gpiohs_u32_t high_ip;
    /* Address offset 0x30, Low interrupt enable */
    gpiohs_u32_t low_ie;
    /* Address offset 0x34, Low interrupt pending */
    gpiohs_u32_t low_ip;
    /* Address offset 0x38, HW I/O Function enable */
    gpiohs_u32_t iof_en;
    /* Address offset 0x3c, HW I/O Function select */
    gpiohs_u32_t iof_sel;
    /* Address offset 0x40, Output XOR (invert) */
    gpiohs_u32_t output_xor;
} __attribute__((packed, aligned(4))) gpiohs_t;

/**
 * @brief       GPIO High-speed object instanse
 */
extern volatile gpiohs_t *const gpiohs;

/**
 * @brief       Set Gpiohs drive mode
 *
 * @param[in]   gpiohs_pin         Gpiohs pin
 * @param[in]   mode        Gpiohs pin drive mode
 */
void gpiohs_set_drive_mode(uint8_t gpiohs_pin, gpio_drive_mode_t mode);

/**
 * @brief      Set Gpiohs pin value
 *
 * @param[in]   gpiohs_pin      Gpiohs pin
 * @param[in]   value    Gpiohs pin value
 */
void gpiohs_set_pin_output_value(uint8_t gpiohs_pin, gpio_pin_value_t value);
#endif //LAB8_GPIOHS_H
