#version 400

out vec4 fragColor;

//in vec3 col;

uniform vec2 res;

const float MAX_ITER  = 128.0;

float mandelbrot(vec2 uv) {
    vec2 c = 5.0 * uv - vec2(0.7, 0.0);
    vec2 z = vec2(0.0);
    float iter = 0.0;
    for (float i = 0; i < MAX_ITER; i++) {
        z = vec2(z.x*z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        if (dot(z, z) >= 5.0) return iter / MAX_ITER * 1.5;
        iter++;
    }
    return 0.0;
}

vec3 rand_(float m) {
    float x = fract(sin(2.0*m) + 5625.246);
    float y = fract(sin(m+x) + 2216.486);
    float z = fract(sin(x+y)+8276.352);
    return vec3(x, y, z);
}


void main() { 

    vec2 uv = (gl_FragCoord.xy - 0.5 * res.xy)/res.y;
    vec3 col = vec3(0.0);
    float m = mandelbrot(uv);
    col += rand_(m);
    //col = pow(col, vec3(0.45));

    fragColor = vec4(col, 0.0);
}