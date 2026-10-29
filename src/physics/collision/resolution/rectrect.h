#ifndef RESOLUTION_COLLISION_RECTRECT_H
#define RESOLUTION_COLLISION_RECTRECT_H

#include <stdbool.h>

#include "../../../basics/shapes/rectangle/rectangle.h"
#include "../../../basics/position/free_position.h"

bool narrow_rect_rect_collision(rect_t rect1, rect_t rect2, pos_t* out);

#endif
