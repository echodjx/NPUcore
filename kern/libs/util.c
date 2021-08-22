//
// Created by lumin on 2020/11/16.
//

#include <util.h>

void set_bits_value(volatile uint32_t *bits, uint32_t mask, uint32_t value)
{
    uint32_t masked_origin_value = (*bits) & ~mask;
    *bits = masked_origin_value | (value & mask);
}

void set_bits_value_offset(volatile uint32_t *bits, uint32_t mask, uint32_t value, uint32_t offset)
{
    set_bits_value(bits, mask << offset, value << offset);
}

void set_bit(volatile uint32_t *bits, uint32_t offset)
{
    set_bits_value(bits, 1 << offset, 1 << offset);
}

void clear_bit(volatile uint32_t *bits, uint32_t offset)
{
    set_bits_value(bits, 1 << offset, 0);
}