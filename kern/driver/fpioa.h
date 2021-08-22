//
// Created by lumin on 2020/11/2.
//

#ifndef LAB8_FPIOA_H
#define LAB8_FPIOA_H

#include <defs.h>
#include <io.h>

#define FPIOA_NUM_IO    (48)

typedef enum
{
    FUNC_JTAG_TCLK        = 0,  /*!< JTAG Test Clock */
    FUNC_JTAG_TDI         = 1,  /*!< JTAG Test Data In */
    FUNC_JTAG_TMS         = 2,  /*!< JTAG Test Mode Select */
    FUNC_JTAG_TDO         = 3,  /*!< JTAG Test Data Out */
    FUNC_SPI0_D0          = 4,  /*!< SPI0 Data 0 */
    FUNC_SPI0_D1          = 5,  /*!< SPI0 Data 1 */
    FUNC_SPI0_D2          = 6,  /*!< SPI0 Data 2 */
    FUNC_SPI0_D3          = 7,  /*!< SPI0 Data 3 */
    FUNC_SPI0_D4          = 8,  /*!< SPI0 Data 4 */
    FUNC_SPI0_D5          = 9,  /*!< SPI0 Data 5 */
    FUNC_SPI0_D6          = 10, /*!< SPI0 Data 6 */
    FUNC_SPI0_D7          = 11, /*!< SPI0 Data 7 */
    FUNC_SPI0_SS0         = 12, /*!< SPI0 Chip Select 0 */
    FUNC_SPI0_SS1         = 13, /*!< SPI0 Chip Select 1 */
    FUNC_SPI0_SS2         = 14, /*!< SPI0 Chip Select 2 */
    FUNC_SPI0_SS3         = 15, /*!< SPI0 Chip Select 3 */
    FUNC_SPI0_ARB         = 16, /*!< SPI0 Arbitration */
    FUNC_SPI0_SCLK        = 17, /*!< SPI0 Serial Clock */
    FUNC_UARTHS_RX        = 18, /*!< UART High speed Receiver */
    FUNC_UARTHS_TX        = 19, /*!< UART High speed Transmitter */
    FUNC_RESV6            = 20, /*!< Reserved function */
    FUNC_RESV7            = 21, /*!< Reserved function */
    FUNC_CLK_SPI1         = 22, /*!< Clock SPI1 */
    FUNC_CLK_I2C1         = 23, /*!< Clock I2C1 */
    FUNC_GPIOHS0          = 24, /*!< GPIO High speed 0 */
    FUNC_GPIOHS1          = 25, /*!< GPIO High speed 1 */
    FUNC_GPIOHS2          = 26, /*!< GPIO High speed 2 */
    FUNC_GPIOHS3          = 27, /*!< GPIO High speed 3 */
    FUNC_GPIOHS4          = 28, /*!< GPIO High speed 4 */
    FUNC_GPIOHS5          = 29, /*!< GPIO High speed 5 */
    FUNC_GPIOHS6          = 30, /*!< GPIO High speed 6 */
    FUNC_GPIOHS7          = 31, /*!< GPIO High speed 7 */
    FUNC_GPIOHS8          = 32, /*!< GPIO High speed 8 */
    FUNC_GPIOHS9          = 33, /*!< GPIO High speed 9 */
    FUNC_GPIOHS10         = 34, /*!< GPIO High speed 10 */
    FUNC_GPIOHS11         = 35, /*!< GPIO High speed 11 */
    FUNC_GPIOHS12         = 36, /*!< GPIO High speed 12 */
    FUNC_GPIOHS13         = 37, /*!< GPIO High speed 13 */
    FUNC_GPIOHS14         = 38, /*!< GPIO High speed 14 */
    FUNC_GPIOHS15         = 39, /*!< GPIO High speed 15 */
    FUNC_GPIOHS16         = 40, /*!< GPIO High speed 16 */
    FUNC_GPIOHS17         = 41, /*!< GPIO High speed 17 */
    FUNC_GPIOHS18         = 42, /*!< GPIO High speed 18 */
    FUNC_GPIOHS19         = 43, /*!< GPIO High speed 19 */
    FUNC_GPIOHS20         = 44, /*!< GPIO High speed 20 */
    FUNC_GPIOHS21         = 45, /*!< GPIO High speed 21 */
    FUNC_GPIOHS22         = 46, /*!< GPIO High speed 22 */
    FUNC_GPIOHS23         = 47, /*!< GPIO High speed 23 */
    FUNC_GPIOHS24         = 48, /*!< GPIO High speed 24 */
    FUNC_GPIOHS25         = 49, /*!< GPIO High speed 25 */
    FUNC_GPIOHS26         = 50, /*!< GPIO High speed 26 */
    FUNC_GPIOHS27         = 51, /*!< GPIO High speed 27 */
    FUNC_GPIOHS28         = 52, /*!< GPIO High speed 28 */
    FUNC_GPIOHS29         = 53, /*!< GPIO High speed 29 */
    FUNC_GPIOHS30         = 54, /*!< GPIO High speed 30 */
    FUNC_GPIOHS31         = 55, /*!< GPIO High speed 31 */
    FUNC_GPIO0            = 56, /*!< GPIO pin 0 */
    FUNC_GPIO1            = 57, /*!< GPIO pin 1 */
    FUNC_GPIO2            = 58, /*!< GPIO pin 2 */
    FUNC_GPIO3            = 59, /*!< GPIO pin 3 */
    FUNC_GPIO4            = 60, /*!< GPIO pin 4 */
    FUNC_GPIO5            = 61, /*!< GPIO pin 5 */
    FUNC_GPIO6            = 62, /*!< GPIO pin 6 */
    FUNC_GPIO7            = 63, /*!< GPIO pin 7 */
    FUNC_UART1_RX         = 64, /*!< UART1 Receiver */
    FUNC_UART1_TX         = 65, /*!< UART1 Transmitter */
    FUNC_UART2_RX         = 66, /*!< UART2 Receiver */
    FUNC_UART2_TX         = 67, /*!< UART2 Transmitter */
    FUNC_UART3_RX         = 68, /*!< UART3 Receiver */
    FUNC_UART3_TX         = 69, /*!< UART3 Transmitter */
    FUNC_SPI1_D0          = 70, /*!< SPI1 Data 0 */
    FUNC_SPI1_D1          = 71, /*!< SPI1 Data 1 */
    FUNC_SPI1_D2          = 72, /*!< SPI1 Data 2 */
    FUNC_SPI1_D3          = 73, /*!< SPI1 Data 3 */
    FUNC_SPI1_D4          = 74, /*!< SPI1 Data 4 */
    FUNC_SPI1_D5          = 75, /*!< SPI1 Data 5 */
    FUNC_SPI1_D6          = 76, /*!< SPI1 Data 6 */
    FUNC_SPI1_D7          = 77, /*!< SPI1 Data 7 */
    FUNC_SPI1_SS0         = 78, /*!< SPI1 Chip Select 0 */
    FUNC_SPI1_SS1         = 79, /*!< SPI1 Chip Select 1 */
    FUNC_SPI1_SS2         = 80, /*!< SPI1 Chip Select 2 */
    FUNC_SPI1_SS3         = 81, /*!< SPI1 Chip Select 3 */
    FUNC_SPI1_ARB         = 82, /*!< SPI1 Arbitration */
    FUNC_SPI1_SCLK        = 83, /*!< SPI1 Serial Clock */
    FUNC_SPI_SLAVE_D0     = 84, /*!< SPI Slave Data 0 */
    FUNC_SPI_SLAVE_SS     = 85, /*!< SPI Slave Select */
    FUNC_SPI_SLAVE_SCLK   = 86, /*!< SPI Slave Serial Clock */
    FUNC_I2S0_MCLK        = 87, /*!< I2S0 Master Clock */
    FUNC_I2S0_SCLK        = 88, /*!< I2S0 Serial Clock(BCLK) */
    FUNC_I2S0_WS          = 89, /*!< I2S0 Word Select(LRCLK) */
    FUNC_I2S0_IN_D0       = 90, /*!< I2S0 Serial Data Input 0 */
    FUNC_I2S0_IN_D1       = 91, /*!< I2S0 Serial Data Input 1 */
    FUNC_I2S0_IN_D2       = 92, /*!< I2S0 Serial Data Input 2 */
    FUNC_I2S0_IN_D3       = 93, /*!< I2S0 Serial Data Input 3 */
    FUNC_I2S0_OUT_D0      = 94, /*!< I2S0 Serial Data Output 0 */
    FUNC_I2S0_OUT_D1      = 95, /*!< I2S0 Serial Data Output 1 */
    FUNC_I2S0_OUT_D2      = 96, /*!< I2S0 Serial Data Output 2 */
    FUNC_I2S0_OUT_D3      = 97, /*!< I2S0 Serial Data Output 3 */
    FUNC_I2S1_MCLK        = 98, /*!< I2S1 Master Clock */
    FUNC_I2S1_SCLK        = 99, /*!< I2S1 Serial Clock(BCLK) */
    FUNC_I2S1_WS          = 100,    /*!< I2S1 Word Select(LRCLK) */
    FUNC_I2S1_IN_D0       = 101,    /*!< I2S1 Serial Data Input 0 */
    FUNC_I2S1_IN_D1       = 102,    /*!< I2S1 Serial Data Input 1 */
    FUNC_I2S1_IN_D2       = 103,    /*!< I2S1 Serial Data Input 2 */
    FUNC_I2S1_IN_D3       = 104,    /*!< I2S1 Serial Data Input 3 */
    FUNC_I2S1_OUT_D0      = 105,    /*!< I2S1 Serial Data Output 0 */
    FUNC_I2S1_OUT_D1      = 106,    /*!< I2S1 Serial Data Output 1 */
    FUNC_I2S1_OUT_D2      = 107,    /*!< I2S1 Serial Data Output 2 */
    FUNC_I2S1_OUT_D3      = 108,    /*!< I2S1 Serial Data Output 3 */
    FUNC_I2S2_MCLK        = 109,    /*!< I2S2 Master Clock */
    FUNC_I2S2_SCLK        = 110,    /*!< I2S2 Serial Clock(BCLK) */
    FUNC_I2S2_WS          = 111,    /*!< I2S2 Word Select(LRCLK) */
    FUNC_I2S2_IN_D0       = 112,    /*!< I2S2 Serial Data Input 0 */
    FUNC_I2S2_IN_D1       = 113,    /*!< I2S2 Serial Data Input 1 */
    FUNC_I2S2_IN_D2       = 114,    /*!< I2S2 Serial Data Input 2 */
    FUNC_I2S2_IN_D3       = 115,    /*!< I2S2 Serial Data Input 3 */
    FUNC_I2S2_OUT_D0      = 116,    /*!< I2S2 Serial Data Output 0 */
    FUNC_I2S2_OUT_D1      = 117,    /*!< I2S2 Serial Data Output 1 */
    FUNC_I2S2_OUT_D2      = 118,    /*!< I2S2 Serial Data Output 2 */
    FUNC_I2S2_OUT_D3      = 119,    /*!< I2S2 Serial Data Output 3 */
    FUNC_RESV0            = 120,    /*!< Reserved function */
    FUNC_RESV1            = 121,    /*!< Reserved function */
    FUNC_RESV2            = 122,    /*!< Reserved function */
    FUNC_RESV3            = 123,    /*!< Reserved function */
    FUNC_RESV4            = 124,    /*!< Reserved function */
    FUNC_RESV5            = 125,    /*!< Reserved function */
    FUNC_I2C0_SCLK        = 126,    /*!< I2C0 Serial Clock */
    FUNC_I2C0_SDA         = 127,    /*!< I2C0 Serial Data */
    FUNC_I2C1_SCLK        = 128,    /*!< I2C1 Serial Clock */
    FUNC_I2C1_SDA         = 129,    /*!< I2C1 Serial Data */
    FUNC_I2C2_SCLK        = 130,    /*!< I2C2 Serial Clock */
    FUNC_I2C2_SDA         = 131,    /*!< I2C2 Serial Data */
    FUNC_CMOS_XCLK        = 132,    /*!< DVP System Clock */
    FUNC_CMOS_RST         = 133,    /*!< DVP System Reset */
    FUNC_CMOS_PWDN        = 134,    /*!< DVP Power Down Mode */
    FUNC_CMOS_VSYNC       = 135,    /*!< DVP Vertical Sync */
    FUNC_CMOS_HREF        = 136,    /*!< DVP Horizontal Reference output */
    FUNC_CMOS_PCLK        = 137,    /*!< Pixel Clock */
    FUNC_CMOS_D0          = 138,    /*!< Data Bit 0 */
    FUNC_CMOS_D1          = 139,    /*!< Data Bit 1 */
    FUNC_CMOS_D2          = 140,    /*!< Data Bit 2 */
    FUNC_CMOS_D3          = 141,    /*!< Data Bit 3 */
    FUNC_CMOS_D4          = 142,    /*!< Data Bit 4 */
    FUNC_CMOS_D5          = 143,    /*!< Data Bit 5 */
    FUNC_CMOS_D6          = 144,    /*!< Data Bit 6 */
    FUNC_CMOS_D7          = 145,    /*!< Data Bit 7 */
    FUNC_SCCB_SCLK        = 146,    /*!< SCCB Serial Clock */
    FUNC_SCCB_SDA         = 147,    /*!< SCCB Serial Data */
    FUNC_UART1_CTS        = 148,    /*!< UART1 Clear To Send */
    FUNC_UART1_DSR        = 149,    /*!< UART1 Data Set Ready */
    FUNC_UART1_DCD        = 150,    /*!< UART1 Data Carrier Detect */
    FUNC_UART1_RI         = 151,    /*!< UART1 Ring Indicator */
    FUNC_UART1_SIR_IN     = 152,    /*!< UART1 Serial Infrared Input */
    FUNC_UART1_DTR        = 153,    /*!< UART1 Data Terminal Ready */
    FUNC_UART1_RTS        = 154,    /*!< UART1 Request To Send */
    FUNC_UART1_OUT2       = 155,    /*!< UART1 User-designated Output 2 */
    FUNC_UART1_OUT1       = 156,    /*!< UART1 User-designated Output 1 */
    FUNC_UART1_SIR_OUT    = 157,    /*!< UART1 Serial Infrared Output */
    FUNC_UART1_BAUD       = 158,    /*!< UART1 Transmit Clock Output */
    FUNC_UART1_RE         = 159,    /*!< UART1 Receiver Output Enable */
    FUNC_UART1_DE         = 160,    /*!< UART1 Driver Output Enable */
    FUNC_UART1_RS485_EN   = 161,    /*!< UART1 RS485 Enable */
    FUNC_UART2_CTS        = 162,    /*!< UART2 Clear To Send */
    FUNC_UART2_DSR        = 163,    /*!< UART2 Data Set Ready */
    FUNC_UART2_DCD        = 164,    /*!< UART2 Data Carrier Detect */
    FUNC_UART2_RI         = 165,    /*!< UART2 Ring Indicator */
    FUNC_UART2_SIR_IN     = 166,    /*!< UART2 Serial Infrared Input */
    FUNC_UART2_DTR        = 167,    /*!< UART2 Data Terminal Ready */
    FUNC_UART2_RTS        = 168,    /*!< UART2 Request To Send */
    FUNC_UART2_OUT2       = 169,    /*!< UART2 User-designated Output 2 */
    FUNC_UART2_OUT1       = 170,    /*!< UART2 User-designated Output 1 */
    FUNC_UART2_SIR_OUT    = 171,    /*!< UART2 Serial Infrared Output */
    FUNC_UART2_BAUD       = 172,    /*!< UART2 Transmit Clock Output */
    FUNC_UART2_RE         = 173,    /*!< UART2 Receiver Output Enable */
    FUNC_UART2_DE         = 174,    /*!< UART2 Driver Output Enable */
    FUNC_UART2_RS485_EN   = 175,    /*!< UART2 RS485 Enable */
    FUNC_UART3_CTS        = 176,    /*!< UART3 Clear To Send */
    FUNC_UART3_DSR        = 177,    /*!< UART3 Data Set Ready */
    FUNC_UART3_DCD        = 178,    /*!< UART3 Data Carrier Detect */
    FUNC_UART3_RI         = 179,    /*!< UART3 Ring Indicator */
    FUNC_UART3_SIR_IN     = 180,    /*!< UART3 Serial Infrared Input */
    FUNC_UART3_DTR        = 181,    /*!< UART3 Data Terminal Ready */
    FUNC_UART3_RTS        = 182,    /*!< UART3 Request To Send */
    FUNC_UART3_OUT2       = 183,    /*!< UART3 User-designated Output 2 */
    FUNC_UART3_OUT1       = 184,    /*!< UART3 User-designated Output 1 */
    FUNC_UART3_SIR_OUT    = 185,    /*!< UART3 Serial Infrared Output */
    FUNC_UART3_BAUD       = 186,    /*!< UART3 Transmit Clock Output */
    FUNC_UART3_RE         = 187,    /*!< UART3 Receiver Output Enable */
    FUNC_UART3_DE         = 188,    /*!< UART3 Driver Output Enable */
    FUNC_UART3_RS485_EN   = 189,    /*!< UART3 RS485 Enable */
    FUNC_TIMER0_TOGGLE1   = 190,    /*!< TIMER0 Toggle Output 1 */
    FUNC_TIMER0_TOGGLE2   = 191,    /*!< TIMER0 Toggle Output 2 */
    FUNC_TIMER0_TOGGLE3   = 192,    /*!< TIMER0 Toggle Output 3 */
    FUNC_TIMER0_TOGGLE4   = 193,    /*!< TIMER0 Toggle Output 4 */
    FUNC_TIMER1_TOGGLE1   = 194,    /*!< TIMER1 Toggle Output 1 */
    FUNC_TIMER1_TOGGLE2   = 195,    /*!< TIMER1 Toggle Output 2 */
    FUNC_TIMER1_TOGGLE3   = 196,    /*!< TIMER1 Toggle Output 3 */
    FUNC_TIMER1_TOGGLE4   = 197,    /*!< TIMER1 Toggle Output 4 */
    FUNC_TIMER2_TOGGLE1   = 198,    /*!< TIMER2 Toggle Output 1 */
    FUNC_TIMER2_TOGGLE2   = 199,    /*!< TIMER2 Toggle Output 2 */
    FUNC_TIMER2_TOGGLE3   = 200,    /*!< TIMER2 Toggle Output 3 */
    FUNC_TIMER2_TOGGLE4   = 201,    /*!< TIMER2 Toggle Output 4 */
    FUNC_CLK_SPI2         = 202,    /*!< Clock SPI2 */
    FUNC_CLK_I2C2         = 203,    /*!< Clock I2C2 */
    FUNC_INTERNAL0        = 204,    /*!< Internal function signal 0 */
    FUNC_INTERNAL1        = 205,    /*!< Internal function signal 1 */
    FUNC_INTERNAL2        = 206,    /*!< Internal function signal 2 */
    FUNC_INTERNAL3        = 207,    /*!< Internal function signal 3 */
    FUNC_INTERNAL4        = 208,    /*!< Internal function signal 4 */
    FUNC_INTERNAL5        = 209,    /*!< Internal function signal 5 */
    FUNC_INTERNAL6        = 210,    /*!< Internal function signal 6 */
    FUNC_INTERNAL7        = 211,    /*!< Internal function signal 7 */
    FUNC_INTERNAL8        = 212,    /*!< Internal function signal 8 */
    FUNC_INTERNAL9        = 213,    /*!< Internal function signal 9 */
    FUNC_INTERNAL10       = 214,    /*!< Internal function signal 10 */
    FUNC_INTERNAL11       = 215,    /*!< Internal function signal 11 */
    FUNC_INTERNAL12       = 216,    /*!< Internal function signal 12 */
    FUNC_INTERNAL13       = 217,    /*!< Internal function signal 13 */
    FUNC_INTERNAL14       = 218,    /*!< Internal function signal 14 */
    FUNC_INTERNAL15       = 219,    /*!< Internal function signal 15 */
    FUNC_INTERNAL16       = 220,    /*!< Internal function signal 16 */
    FUNC_INTERNAL17       = 221,    /*!< Internal function signal 17 */
    FUNC_CONSTANT         = 222,    /*!< Constant function */
    FUNC_INTERNAL18       = 223,    /*!< Internal function signal 18 */
    FUNC_DEBUG0           = 224,    /*!< Debug function 0 */
    FUNC_DEBUG1           = 225,    /*!< Debug function 1 */
    FUNC_DEBUG2           = 226,    /*!< Debug function 2 */
    FUNC_DEBUG3           = 227,    /*!< Debug function 3 */
    FUNC_DEBUG4           = 228,    /*!< Debug function 4 */
    FUNC_DEBUG5           = 229,    /*!< Debug function 5 */
    FUNC_DEBUG6           = 230,    /*!< Debug function 6 */
    FUNC_DEBUG7           = 231,    /*!< Debug function 7 */
    FUNC_DEBUG8           = 232,    /*!< Debug function 8 */
    FUNC_DEBUG9           = 233,    /*!< Debug function 9 */
    FUNC_DEBUG10          = 234,    /*!< Debug function 10 */
    FUNC_DEBUG11          = 235,    /*!< Debug function 11 */
    FUNC_DEBUG12          = 236,    /*!< Debug function 12 */
    FUNC_DEBUG13          = 237,    /*!< Debug function 13 */
    FUNC_DEBUG14          = 238,    /*!< Debug function 14 */
    FUNC_DEBUG15          = 239,    /*!< Debug function 15 */
    FUNC_DEBUG16          = 240,    /*!< Debug function 16 */
    FUNC_DEBUG17          = 241,    /*!< Debug function 17 */
    FUNC_DEBUG18          = 242,    /*!< Debug function 18 */
    FUNC_DEBUG19          = 243,    /*!< Debug function 19 */
    FUNC_DEBUG20          = 244,    /*!< Debug function 20 */
    FUNC_DEBUG21          = 245,    /*!< Debug function 21 */
    FUNC_DEBUG22          = 246,    /*!< Debug function 22 */
    FUNC_DEBUG23          = 247,    /*!< Debug function 23 */
    FUNC_DEBUG24          = 248,    /*!< Debug function 24 */
    FUNC_DEBUG25          = 249,    /*!< Debug function 25 */
    FUNC_DEBUG26          = 250,    /*!< Debug function 26 */
    FUNC_DEBUG27          = 251,    /*!< Debug function 27 */
    FUNC_DEBUG28          = 252,    /*!< Debug function 28 */
    FUNC_DEBUG29          = 253,    /*!< Debug function 29 */
    FUNC_DEBUG30          = 254,    /*!< Debug function 30 */
    FUNC_DEBUG31          = 255,    /*!< Debug function 31 */
    FUNC_MAX              = 256,    /*!< Function numbers */
} fpioa_function_t;

/**
 * @brief      FPIOA pull settings
 *
 * @note       FPIOA pull settings description
 *
 * | PU  | PD  | Description                       |
 * |-----|-----|-----------------------------------|
 * | 0   | 0   | No Pull                           |
 * | 0   | 1   | Pull Down                         |
 * | 1   | 0   | Pull Up                           |
 * | 1   | 1   | Undefined                         |
 *
 */

/* clang-format off */
typedef enum
{
    FPIOA_PULL_NONE,      /*!< No Pull */
    FPIOA_PULL_DOWN,      /*!< Pull Down */
    FPIOA_PULL_UP,        /*!< Pull Up */
    FPIOA_PULL_MAX        /*!< Count of pull settings */
} fpioa_pull_t;

/**
 * @brief      FPIOA IO
 *
 *             FPIOA IO is the specific pin of the chip package. Every IO
 *             has a 32bit width register that can independently implement
 *             schmitt trigger, invert input, invert output, strong pull
 *             up, driving selector, static input and static output. And more,
 *             it can implement any pin of any peripheral devices.
 *
 * @note       FPIOA IO's register bits Layout
 *
 * | Bits      | Name     |Description                                        |
 * |-----------|----------|---------------------------------------------------|
 * | 31        | PAD_DI   | Read current IO's data input.                     |
 * | 30:24     | NA       | Reserved bits.                                    |
 * | 23        | ST       | Schmitt trigger.                                  |
 * | 22        | DI_INV   | Invert Data input.                                |
 * | 21        | IE_INV   | Invert the input enable signal.                   |
 * | 20        | IE_EN    | Input enable. It can disable or enable IO input.  |
 * | 19        | SL       | Slew rate control enable.                         |
 * | 18        | SPU      | Strong pull up.                                   |
 * | 17        | PD       | Pull select: 0 for pull down, 1 for pull up.      |
 * | 16        | PU       | Pull enable.                                      |
 * | 15        | DO_INV   | Invert the result of data output select (DO_SEL). |
 * | 14        | DO_SEL   | Data output select: 0 for DO, 1 for OE.           |
 * | 13        | OE_INV   | Invert the output enable signal.                  |
 * | 12        | OE_EN    | Output enable.It can disable or enable IO output. |
 * | 11:8      | DS       | Driving selector.                                 |
 * | 7:0       | CH_SEL   | Channel select from 256 input.                    |
 *
 */
typedef struct
{
    uint32_t ch_sel : 8;
    /*!< Channel select from 256 input. */
    uint32_t ds : 4;
    /*!< Driving selector. */
    uint32_t oe_en : 1;
    /*!< Static output enable, will AND with OE_INV. */
    uint32_t oe_inv : 1;
    /*!< Invert output enable. */
    uint32_t do_sel : 1;
    /*!< Data output select: 0 for DO, 1 for OE. */
    uint32_t do_inv : 1;
    /*!< Invert the result of data output select (DO_SEL). */
    uint32_t pu : 1;
    /*!< Pull up enable. 0 for nothing, 1 for pull up. */
    uint32_t pd : 1;
    /*!< Pull down enable. 0 for nothing, 1 for pull down. */
    uint32_t resv0 : 1;
    /*!< Reserved bits. */
    uint32_t sl : 1;
    /*!< Slew rate control enable. */
    uint32_t ie_en : 1;
    /*!< Static input enable, will AND with IE_INV. */
    uint32_t ie_inv : 1;
    /*!< Invert input enable. */
    uint32_t di_inv : 1;
    /*!< Invert Data input. */
    uint32_t st : 1;
    /*!< Schmitt trigger. */
    uint32_t resv1 : 7;
    /*!< Reserved bits. */
    uint32_t pad_di : 1;
    /*!< Read current IO's data input. */
} __attribute__((packed,aligned(4))) fpioa_io_config_t;

/**
 * @brief      FPIOA tie setting
 *
 *             FPIOA Object have 48 IO pin object and 256 bit input tie bits.
 *             All SPI arbitration signal will tie high by default.
 *
 * @note       FPIOA function tie bits RAM Layout
 *
 * | Address   | Name             |Description                       |
 * |-----------|------------------|----------------------------------|
 * | 0x000     | TIE_EN[31:0]     | Input tie enable bits [31:0]     |
 * | 0x004     | TIE_EN[63:32]    | Input tie enable bits [63:32]    |
 * | 0x008     | TIE_EN[95:64]    | Input tie enable bits [95:64]    |
 * | 0x00C     | TIE_EN[127:96]   | Input tie enable bits [127:96]   |
 * | 0x010     | TIE_EN[159:128]  | Input tie enable bits [159:128]  |
 * | 0x014     | TIE_EN[191:160]  | Input tie enable bits [191:160]  |
 * | 0x018     | TIE_EN[223:192]  | Input tie enable bits [223:192]  |
 * | 0x01C     | TIE_EN[255:224]  | Input tie enable bits [255:224]  |
 * | 0x020     | TIE_VAL[31:0]    | Input tie value bits [31:0]      |
 * | 0x024     | TIE_VAL[63:32]   | Input tie value bits [63:32]     |
 * | 0x028     | TIE_VAL[95:64]   | Input tie value bits [95:64]     |
 * | 0x02C     | TIE_VAL[127:96]  | Input tie value bits [127:96]    |
 * | 0x030     | TIE_VAL[159:128] | Input tie value bits [159:128]   |
 * | 0x034     | TIE_VAL[191:160] | Input tie value bits [191:160]   |
 * | 0x038     | TIE_VAL[223:192] | Input tie value bits [223:192]   |
 * | 0x03C     | TIE_VAL[255:224] | Input tie value bits [255:224]   |
 *
 * @note       Function which input tie high by default
 *
 * | Name          |Description                            |
 * |---------------|---------------------------------------|
 * | SPI0_ARB      | Arbitration function of SPI master 0  |
 * | SPI1_ARB      | Arbitration function of SPI master 1  |
 *
 *             Tie high means the SPI Arbitration input is 1
 *
 */

typedef struct
{
    uint32_t en[FUNC_MAX/32];
    /*!< FPIOA GPIO multiplexer tie enable array */
    uint32_t val[FUNC_MAX/32];
    /*!< FPIOA GPIO multiplexer tie value array */
} __attribute__((packed,aligned(4))) fpioa_tie_t;

typedef struct
{
    fpioa_io_config_t io[FPIOA_NUM_IO];
    /*!< FPIOA GPIO multiplexer io array */
    fpioa_tie_t tie;
    /*!< FPIOA GPIO multiplexer tie */
} __attribute__((packed,aligned(4))) fpioa_t;

extern volatile fpioa_t *const fpioa;

/**
 * @brief       Set only IO configuration with function number
 *
 * @note        The default IO configuration which bind to function number will
 *              set automatically
 *
 * @param[in]   pin      The IO number
 * @param[in]   function    The function enum number
 *
 * @return      result
 *     - 0      Success
 *     - Other  Fail
 */
int fpioa_set_function(int pin, fpioa_function_t function);

/**
 * @brief      Set IO pull function
 *
 * @param[in]   number  The IO number
 * @param[in]   pull    The pull enum number
 *
 * @return      result
 *     - 0      Success
 *     - Other  Fail
 */
int fpioa_set_io_pull(int pin, fpioa_pull_t pull);

/**
 * @brief       Get IO by function
 *
 * @param[in]   function  The function enum number
 *
 * @return      result
 *     - -1     Fail
 *     - Other  The IO number
 */
int fpioa_get_owner_pin_by_function(fpioa_function_t function);

#endif //LAB8_FPIOA_H
