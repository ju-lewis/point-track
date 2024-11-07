/* Wireless testing and logging functionality for Point-Track development
 * 
 * Written by Julian Lewis
 * 11th Oct 2024
 */


#include "freertos/FreeRTOS.h"
#include "driver/uart.h"
#include "driver/gpio.h"
#include "testing.h"

#include <string.h>


const char *TEST_MESSAGE = "Testing.";

void config_led();


void debug_loop() {

    // Configure UART communication
    uart_config_t uart_config = {
        .baud_rate = 115200,
        .data_bits = UART_DATA_8_BITS,
        .parity = UART_PARITY_DISABLE,
        .stop_bits = UART_STOP_BITS_1,
        .flow_ctrl = UART_HW_FLOWCTRL_DISABLE,
        .source_clk = UART_SCLK_DEFAULT
    };
    uart_param_config(UART_NUM_2, &uart_config);





    
    // Set UART pinout
    uart_set_pin(UART_NUM_2,
            5,                  // TX
            4,                  // RX
            UART_PIN_NO_CHANGE, // Ignore RTS
            UART_PIN_NO_CHANGE  // Ignore CTS
    );

    // Install UART driver
    uart_driver_install(UART_NUM_2, 1024, 1024, 10, NULL, 0);
    config_led();

    // Read from UART
    uint8_t data[128];
    for(;;) {
        int len = 0;
        uart_get_buffered_data_len(UART_NUM_2, (size_t*)&len);
        len = uart_read_bytes(UART_NUM_2, data, len, 100);

        // If data was successfully read, indicate on LED
        if(len > 0) {
            gpio_set_level(GPIO_NUM_23, 1);
        } else {
            gpio_set_level(GPIO_NUM_23, 0);
        }
    }
}


void config_led() {
    gpio_reset_pin(GPIO_NUM_23);
    gpio_set_direction(GPIO_NUM_23, GPIO_MODE_OUTPUT);
}
