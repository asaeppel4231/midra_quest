#include "line.h"

void update_line_length(line_t* line){
  #ifdef DEBUG
  assert(line != NULL);
  #endif

  line->length = vect2_length(line->start, line->end);
}
