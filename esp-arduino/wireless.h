/* Header file for PointTrack logging box wireless functionality
 * Written by Julian Lewis
 * 23/10/2024
 */

#ifndef WIRELESS
#define WIRELESS

#include <stdlib.h>
#include <LiquidCrystal.h>


int getNumNetworks();
void scanAndConnect(LiquidCrystal lcd);


#endif
