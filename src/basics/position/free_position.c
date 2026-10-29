#include "free_position.h"

#include <stddef.h>

void move_pos(pos_t* pos, float vx, float vy){
  vect2_add(*pos, (vect2_t){.x = vx, .y = vy}, pos);
}

float x_pos_distance(pos_t pos1, pos_t pos2){
    return pos1.x - pos2.x;
}

float y_pos_distance(pos_t pos1, pos_t pos2){
    return pos1.y - pos2.y;
}

