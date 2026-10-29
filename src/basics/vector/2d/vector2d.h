#ifndef VECTOR2D_H
#define VECTOR2D_H

typedef struct midra_quest_vector_2d {
  float x;
  float y;
}vect2_t;

void vect2_add(vect2_t vec1, vect2_t vec2, vect2_t* dest);
void vect2_sub(vect2_t vec1, vect2_t vec2, vect2_t* dest);

float vect2_length(vect2_t vec1, vect2_t vec2);

void vect2_scale(vect2_t vec, float f, vect2_t* dest);
void vect2_normalize(vect2_t vec, vect2_t* dest);

float vect2_dot_product(vect2_t vec1, vect2_t vec2);
float vect2_cross_product(vect2_t vec1, vect2_t vec2);

void vect2_reflect(vect2_t vec1, vect2_t normal, float restitution, vect2_t* dest);

#endif
