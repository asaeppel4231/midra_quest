/* Following functions in this file are AI generated */

#[inline(always)]
#[track_caller]
#[allow(unused_variables)]
fn check_f32(op: &str, old_val: f32, a: f32, b: f32, result: f32) -> f32 {
    #[cfg(debug_assertions)]
    {
        // 1. Overflow & NaN Prüfung
        if !result.is_finite() {
            panic!(
                "\n[FLOAT OVERFLOW / NaN] Operation: '{}'\n  Vorheriger Wert: {}\n  Operand A:       {}\n  Operand B:       {}\n  Ergebnis:        {}\n",
                op, old_val, a, b, result
            );
        }

        // 2. Underflow Prüfung (Ergebnis rutscht in ungenaue Subnormal-Zahlen oder wird 0.0, obwohl die Eingaben nicht 0.0 waren)
        if result != 0.0 && result.abs() < f32::MIN_POSITIVE {
            panic!(
                "\n[FLOAT UNDERFLOW (Subnormal)] Operation: '{}'\n  Vorheriger Wert: {}\n  Operand A:       {}\n  Operand B:       {}\n  Ergebnis:        {}\n",
                op, old_val, a, b, result
            );
        }

        if result == 0.0 && a != 0.0 && b != 0.0 && op != "-" {
            panic!(
                "\n[FLOAT UNDERFLOW (Flush to Zero)] Operation: '{}'\n  Vorheriger Wert: {}\n  Operand A:       {}\n  Operand B:       {}\n  Ergebnis:        0.0\n",
                op, old_val, a, b
            );
        }
    }

    result
}

#[inline(always)]
#[track_caller]
#[allow(unused_variables)]
pub fn safe_add(old_val: f32, a: f32, b: f32) -> f32 {
    check_f32("+", old_val, a, b, a + b)
}

#[inline(always)]
#[track_caller]
#[allow(unused_variables)]
pub fn safe_sub(old_val: f32, a: f32, b: f32) -> f32 {
    check_f32("-", old_val, a, b, a - b)
}

#[inline(always)]
#[track_caller]
#[allow(unused_variables)]
pub fn safe_mul(old_val: f32, a: f32, b: f32) -> f32 {
    check_f32("*", old_val, a, b, a * b)
}

#[inline(always)]
#[track_caller]
#[allow(unused_variables)]
pub fn safe_div(old_val: f32, a: f32, b: f32) -> f32 {
    check_f32("/", old_val, a, b, a / b)
}
