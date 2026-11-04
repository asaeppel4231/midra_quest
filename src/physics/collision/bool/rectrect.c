#include "rectrect.h"
#include "coarse_rect.h"

#ifdef DEBUG

#include <basics/safe/float.h>

#endif

bool coarse_rect_rect_collision(rect_t rect1, rect_t rect2){
  float radius_1 = radius_rect_coarse_collision(rect1.width, rect1.height);
  float radius_2 = radius_rect_coarse_collision(rect2.width, rect2.height);

  float distance = vect2_length(rect1.pos, rect2.pos);

  #ifdef DEBUG
  if(safe_float_add(radius_1, radius_2) > distance){
  #else
  if(radius_1 + radius_2 > distance){
  #endif
    return true;
  }
  return false;
}

