//
// Created by lumin on 2020/11/16.
//

#ifndef LAB8_UTIL_H
#define LAB8_UTIL_H

#include <defs.h>

void set_bits_value(volatile uint32_t *bits, uint32_t mask, uint32_t value);
void set_bits_value_offset(volatile uint32_t *bits, uint32_t mask, uint32_t value, uint32_t offset);
void set_bit(volatile uint32_t *bits, uint32_t offset);
void clear_bit(volatile uint32_t *bits, uint32_t offset);

#endif //LAB8_UTIL_H
