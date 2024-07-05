#version 330 core

in vec2 TexPosition;
out vec4 color;

uniform vec3 resolution;
uniform sampler2D texture0;
uniform uint Time;

float rand(vec3 co) {
    return mix(0.5, 1.0, cos(dot(co, vec3(100.0, 120.0, 40.0))));
}

void main(void) {
    vec3 uv = gl_FragCoord.xyz / resolution.xyz;
    float time = Time / 100.0;

    float k = cos(TexPosition.x * 10.0 + time);
    vec3 l = vec3(mix(0.6, 1.0, k));
    vec4 light = vec4(l, 1.0);

    vec4 texture_land = texture(texture0, TexPosition);

    float sum_color = texture_land.r + texture_land.g + texture_land.b;
    float proc_blue = texture_land.b / sum_color;

    float water = 1.0;
    if (proc_blue > 0.60) {
        float t = time/100.0;
        float rand_num = rand(texture_land.rgb + t);
        water = mix(0.9, 1.0, rand_num);
    }

    color = texture_land * water * light;
}
