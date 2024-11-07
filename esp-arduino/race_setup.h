#ifndef  RACE_CONFIG
#define  RACE_CONFIG

#include <LiquidCrystal.h>


/* Race configuration data */

typedef struct {
    char   boatNum[3];
    char courseNum[3];
    char  nomSpeed[3];
} RaceConfig;


RaceConfig enterConfig(LiquidCrystal lcd);

#endif
