#include "float.h"

#include <math.h>
#include <assert.h>
#include <float.h>
#include <stddef.h>

/* All following safe_float functions (including this function) are AI generated */
float safe_float_add(float a, float b){    
    if (a > 0.0f && b > 0.0f){
        assert(b <= FLT_MAX - a);
    } 
    else if (a < 0.0f && b < 0.0f){
        assert(b >= -FLT_MAX - a);
    }
    float result = a + b;
    assert(!isnan(result) && !isinf(result));
    
    return result;
}

float safe_float_sub(float a, float b){    
    if (a > 0.0f && b < 0.0f){
        assert(a <= FLT_MAX + b);
    } 
    else if (a < 0.0f && b > 0.0f){
        assert(a >= -FLT_MAX + b);
    }
    float result = a - b;
    assert(!isnan(result) && !isinf(result));
    
    return result;
}

float safe_float_mul(float a, float b) {
    if (a == 0.0f || b == 0.0f) {
        return 0.0f;
    }

    float abs_a = fabsf(a);
    float abs_b = fabsf(b);

    assert(abs_a <= FLT_MAX / abs_b);
    assert(abs_a >= FLT_MIN / abs_b);

    return a * b;
}

float safe_float_div(float a, float b) {
    assert(b != 0.0f);

    if (a == 0.0f) {
        return 0.0f;
    }

    float abs_a = fabsf(a);
    float abs_b = fabsf(b);

    assert(abs_a <= FLT_MAX * abs_b);
    assert(abs_a >= FLT_MIN * abs_b);

    return a / b;
}
