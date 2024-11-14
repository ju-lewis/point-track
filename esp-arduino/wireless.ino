#include "wireless.h"
#include <WiFi.h>

int getNumNetworks() {
    return (int)WiFi.scanNetworks();
}



