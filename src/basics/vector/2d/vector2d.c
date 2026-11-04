#include "vector2d.h"

#include <math.h>

#ifdef DEBUG

#include <safe/float.h>
#include <assert.h>

#endif

void vect2_add(vect2_t vec1, vect2_t vec2, vect2_t* dest){
  #ifdef DEBUG
  assert(dest != NULL);

  dest->x = safe_float_add(vec1.x, vec2.x);
  dest->y = safe_float_add(vec1.y, vec2.y);

  #else

  dest->x = vec1.x + vec2.x;
  dest->y = vec1.y + vec2.y;
  
  #endif
}

void vect2_sub(vect2_t vec1, vect2_t vec2, vect2_t* dest){
  #ifdef DEBUG
  assert(dest != NULL);

  dest->x = safe_float_sub(vec1.x, vec2.x);
  dest->y = safe_float_sub(vec1.y, vec2.y);

  #else

  dest->x = vec1.x - vec2.x;
  dest->y = vec1.y - vec2.y;
  
  #endif  
}

float vect2_length(vect2_t start, vect2_t end){
    vect2_t temp;
    vect2_sub(start, end, &temp);

    return sqrtf(
                 (temp.x * temp.x) + (temp.y * temp.y)
                );
}

void vect2_scale(vect2_t vec, float f, vect2_t* dest){
  #ifdef DEBUG
  assert(dest != NULL);

  dest->x = safe_float_mul(vec.x, f);
  dest->y = safe_float_mul(vec.y, f);

  #else

  dest->x = vec.x * f;
  dest->y = vec.y * f;
  
  #endif
}

void vect2_normalize(vect2_t vec, vect2_t* dest){
    #ifdef DEBUG
    assert(dest != NULL);
    #endif

    float length = vect2_length(vec, (vect2_t){.x = 0, .y = 0});

    #ifdef DEBUG
    dest->x = safe_float_div(vec.x, length);
    dest->y = safe_float_div(vec.y, length);

    #else

    dest->x = vec.x / length;
    dest->y = vec.y / length;
    
    #endif
}

float vect2_dot_product(vect2_t vec1, vect2_t vec2){
    #ifdef DEBUG
    return safe_float_add(safe_float_mul(vec1.x, vec2.x), safe_float_mul(vec1.y, vec2.y));

    #else

    return (vec1.x * vec2.x) + (vec1.y * vec2.y);
    #endif
}

float vect2_cross_product(vect2_t vec1, vect2_t vec2){
    #ifdef DEBUG
    return safe_float_sub(safe_float_mul(vec1.x, vec2.y), safe_float_mul(vec1.y, vec2.x));

    #else

    return (vec1.x * vec2.y) - (vec1.y * vec2.x);
    #endif
}

void vect2_reflect(vect2_t vec, vect2_t normal, float restitution, vect2_t* dest){
    #ifdef DEBUG
    assert(dest != NULL);
    #endif

    float dot_product = vect2_dot_product(vec, normal);

    #ifdef DEBUG

    float tmp = safe_float_add(1.0f, restitution);
    tmp = safe_float_mul(tmp, dot_product);

    float result = safe_float_mul(tmp, normal.x);
    result = safe_float_sub(vec.x, result);

    dest->x = result;

    result = safe_float_mul(tmp, normal.y);
    result = safe_float_sub(vec.y, result);

    dest->y = result;
    
    #else

    dest->x = vec.x - (1.0f + restitution) * dot_product * normal.x;
    dest->y = vec.y - (1.0f + restitution) * dot_product * normal.y;

    #endif
}
