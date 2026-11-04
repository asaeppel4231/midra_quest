#ifndef CIRCLE_H
#define CIRCLE_H

#include <position/free_position.h>

#include <stdint.h>

#ifdef DEBUG
#include <stdbool.h>
#endif

typedef struct midra_quest_circle {
  float radius;
  pos_t pos;
  uint8_t layer;
}circle_t;

float calculate_circle_area(circle_t circle);

#ifdef DEBUG
bool circle_radius_ok(circle_t circle);
#endif

#endif
