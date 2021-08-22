//
// Created by lumin on 2020/11/3.
//
#include <gpiohs.h>
#include <util.h>

volatile gpiohs_t *const gpiohs = (volatile gpiohs_t *)GPIOHS_BASE_ADDR;

void gpiohs_set_drive_mode(uint8_t gpiohs_pin, gpio_drive_mode_t mode)
{

    assert(gpiohs_pin < GPIOHS_MAX_PINNO);

    // some control regs of gpiohs are managed by fpioa
    // so we must find the owner of this function i.e. the
    // bound pin, and set registers w.r.t that pin so that
    // the changes does matter the owner pin
    int bound_pin = fpioa_get_owner_pin_by_function(FUNC_GPIOHS0 + gpiohs_pin);
    assert(bound_pin >= 0);

    fpioa_pull_t pull;
    uint32_t gpiohs_dir;
    switch (mode)
    {
        case GPIO_DM_INPUT:
            pull = FPIOA_PULL_NONE;
            gpiohs_dir = 0;
            break;
        case GPIO_DM_INPUT_PULL_DOWN:
            pull = FPIOA_PULL_DOWN;
            gpiohs_dir = 0;
            break;
        case GPIO_DM_INPUT_PULL_UP:
            pull = FPIOA_PULL_UP;
            gpiohs_dir = 0;
            break;
        case GPIO_DM_OUTPUT:
            pull = FPIOA_PULL_DOWN;
            gpiohs_dir = 1;
            break;
        default:
            assert(!"GPIO drive mode is not supported.");
    }

    // configure pin register on FPIOA
    fpioa_set_io_pull(bound_pin, pull);

    // configure gpiohs register(as input or output)
    set_bit(gpiohs_dir ? gpiohs->output_en.u32 : gpiohs->input_en.u32, bound_pin);
    clear_bit((!gpiohs_dir) ? gpiohs->output_en.u32 : gpiohs->input_en.u32, bound_pin);
}

void gpiohs_set_pin_output_value(uint8_t gpiohs_pin, gpio_pin_value_t value)
{
    assert(gpiohs_pin < GPIOHS_MAX_PINNO);
    switch (value)
    {
        case GPIO_PV_LOW:
            clear_bit(gpiohs->output_val.u32, gpiohs_pin);
            break;
        case GPIO_PV_HIGH:
            set_bit(gpiohs->output_val.u32, gpiohs_pin);
            break;
        default:
            break;
    }
}