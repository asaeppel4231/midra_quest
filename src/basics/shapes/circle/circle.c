#include "circle.h"
#include <math.h>

#ifdef DEBUG
#include <assert.h>

bool circle_radius_ok(circle_t circle){
  return circle.radius > 0;
}
#endif

float calculate_circle_area(circle_t circle){
  #ifdef DEBUG
  assert(circle_radius_ok(circle));
  #endif
  return (float)M_PI * (circle.radius * circle.radius);
}
