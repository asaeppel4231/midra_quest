#include "dgra.h"

#include <math.h>

float degrees_to_radiant(uint16_t degrees){
  return degrees * ((float)M_PI / 180.0f);
}

uint16_t radiant_to_degrees(float radiant){
  return radiant * (180.0f / (float)M_PI);
}
