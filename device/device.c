#include <freertos/FreeRTOS.h>
#include <freertos/task.h>
#include <driver/gpio.h>
#include <hd44780.h>

//#include "testing.h"

/* LCD Pin Definitions */
#define LCD_ENABLE GPIO_NUM_13
//#define LCD_RW     GPIO_NUM_12 Read/Write is tied to GND to permanently set write mode
#define LCD_RS     GPIO_NUM_14

#define LCD_D7     GPIO_NUM_17
#define LCD_D6     GPIO_NUM_5
#define LCD_D5     GPIO_NUM_18
#define LCD_D4     GPIO_NUM_19


void test_lcd_write();


//static const uint8_t char_data[] = {
//    0x04, 0x0e, 0x0e, 0x0e, 0x1f, 0x00, 0x04, 0x00,
//    0x1f, 0x11, 0x0a, 0x04, 0x0a, 0x11, 0x1f, 0x00
//};


/*
    Logging box entry point
*/
void app_main(void) {
    test_lcd_write();
}



void test_lcd_write() {

    // Define LCD connection
    hd44780_t lcd = {
        .write_cb = NULL,
        .font = HD44780_FONT_5X8,
        .lines = 2,

        .pins = {
            // Control pins
            .rs = LCD_RS,
            .e = LCD_ENABLE,

            // Data register pins
            .d4 = LCD_D4,
            .d5 = LCD_D5,
            .d6 = LCD_D6,
            .d7 = LCD_D7,

            // Backlight isn't controlled yet
            .bl = HD44780_NOT_USED,
        }
    };

    ESP_ERROR_CHECK(hd44780_init(&lcd));

    ESP_ERROR_CHECK(hd44780_upload_character(&lcd, 0, (const uint8_t *)"Hello!"));
}




