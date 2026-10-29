#ifndef TILE_POSITION_H
#define TILE_POSITION_H

#include "free_position.h"

#include <stdint.h>

typedef struct midra_quest_tile_position {
  uint32_t x;
  uint32_t y;
}t_pos_t;

t_pos_t calculate_tile_position(pos_t pos, uint16_t tile_width, uint16_t tile_height);

#endif
