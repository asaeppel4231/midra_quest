#include "tile_position.h"

t_pos_t calculate_tile_position(pos_t pos, uint16_t tile_width, uint16_t tile_height){
  #ifdef DEBUG
  assert(tile_width > 0);
  assert(tile_height > 0);

  assert(pos.x >= 0.0f);
  assert(pos.y >= 0.0f);
  #endif

  return (t_pos_t){
    .x = (uint32_t)(pos.x / tile_width),
    .y = (uint32_t)(pos.y / tile_height)
  };
}
