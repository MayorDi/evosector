#version 330 core

in vec2 TexPosition;
out vec4 color;

uniform vec3 resolution;
uniform sampler2D texture0;
uniform uint Time;

float random(float x) {
    return fract(sin(x) * 10000.);
}

float noise(vec2 p) {
    return random(p.x + p.y * 10000.);
}

vec2 sw(vec2 p) {
    return vec2(floor(p.x), floor(p.y));
}
vec2 se(vec2 p) {
    return vec2(ceil(p.x), floor(p.y));
}
vec2 nw(vec2 p) {
    return vec2(floor(p.x), ceil(p.y));
}
vec2 ne(vec2 p) {
    return vec2(ceil(p.x), ceil(p.y));
}

float smoothNoise(vec2 p) {
    vec2 interp = smoothstep(0., 1., fract(p));
    float s = mix(noise(sw(p)), noise(se(p)), interp.x);
    float n = mix(noise(nw(p)), noise(ne(p)), interp.x);
    return mix(s, n, interp.y);
}

float fractalNoise(vec2 p) {
    float x = 0.;
    x += smoothNoise(p);
    x += smoothNoise(p * 2.) / 2.;
    x += smoothNoise(p * 4.) / 4.;
    x /= 1. + 1. / 2. + 1. / 4. + 1.;
    return x;
}

float movingNoise(vec2 p, float time) {
    float x = fractalNoise(p + time);
    return fractalNoise(p + vec2(x, p.x));
}

// call this for water noise function
float nestedNoise(vec2 p, float time) {
    float x = movingNoise(p, time);
    float y = movingNoise(p + 100., time);
    return movingNoise(p + vec2(x, y), time);
}

void main(void) {
    vec3 uv = gl_FragCoord.xyz / resolution.xyz;

    float k = cos(TexPosition.x * 10.0 + Time / 100.0);
    vec3 l = vec3(mix(0.6, 1.0, k));
    vec4 light = vec4(l, 1.0);

    vec4 texture_land = texture(texture0, TexPosition);

    float sum_color = texture_land.r + texture_land.g + texture_land.b;
    float proc_blue = texture_land.b / sum_color;

    float water = 1.0;
    if (proc_blue > 0.60) {
        water = nestedNoise(TexPosition.ts * 6., Time / 500.0) * 2.5;
    }

    color = texture_land * water * light;
}
