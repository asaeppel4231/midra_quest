#ifndef FREE_POSITION_H
#define FREE_POSITION_H

#include <stdint.h>

#include <vector/2d/vector2d.h>

/* Free modifyable position */

typedef struct midra_quest_vector_2d midra_quest_free_position;

typedef midra_quest_free_position pos_t;

void move_pos(pos_t* pos, float vx, float vy);

float x_pos_distance(pos_t pos1, pos_t pos2);
float y_pos_distance(pos_t pos1, pos_t pos2);

#endif
