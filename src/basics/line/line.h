#ifndef LINE_H
#define LINE_H

#include "../position/free_position.h"

#include <stdio.h>

typedef struct midra_quest_line {
  pos_t start;
  pos_t end;
  float length;
  uint8_t layer;
}line_t;

void update_line_length(line_t* line);

#endif
