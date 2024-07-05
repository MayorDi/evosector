#version 330 core

in vec2 TexPosition;
out vec4 color;

uniform vec3 resolution;
uniform sampler2D texture0;
uniform uint Time;

float rand(vec3 co) {
    return fract(sin(dot(co, vec3(12.9898, 78.233, 34.0234))) * 437.5453);
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
        float time = Time / 50000000.;
        vec2 uv = (gl_FragCoord.xy / resolution.xy);
        float rand_num = rand(texture_land.rgb + time);
        water = mix(0.8, 1.0, sin(rand_num));
    }

    color = texture_land * water * light;
}
