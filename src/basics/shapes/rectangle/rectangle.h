#ifndef RECTANGLE_H
#define RECTANGLE_H

#include "../../position/free_position.h"

#include <stdint.h>

#ifdef DEBUG
#include <stdbool.h>
#endif

typedef struct midra_quest_rectangle {
  float width;
  float height;
  /* In radiant */
  float rotation;
  /* Center of rectangle */
  pos_t pos;
  uint8_t layer;
}rect_t;

float calculate_rect_area(rect_t rect);

#ifdef DEBUG
bool rect_width_ok(rect_t rect);
bool rect_height_ok(rect_t rect);
#endif

#endif
