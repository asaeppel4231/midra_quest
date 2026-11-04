#include "coarse_rect.h"

#include <basics/vector/2d/vector2d.h>

#ifdef DEBUG

#include <basics/safe/float.h>

#endif

float radius_rect_coarse_collision(float rect_width, float rect_height){
  vect2_t temp = {
    #ifdef DEBUG
    .x = safe_float_div(rect_width, 2.0f),
    .y = safe_float_div(rect_height, 2.0f)
    #else
    .x = rect_width / 2.0f,
    .y = rect_height / 2.0f
    #endif
  };

  return vect2_length(temp, (vect2_t){.x = 0, .y = 0});
}
