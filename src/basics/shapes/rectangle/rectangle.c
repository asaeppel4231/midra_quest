#include "rectangle.h"

#ifdef DEBUG

#include <assert.h>

bool rect_width_ok(rect_t rect){
  return rect.width > 0;
}

bool rect_height_ok(rect_t rect){
  return rect.height > 0;
}
#endif

float calculate_rect_area(rect_t rect){
  #ifdef DEBUG
  assert(rect_width_ok(rect));
  assert(rect_height_ok(rect));
  #endif
  return rect.width * rect.height;
}
