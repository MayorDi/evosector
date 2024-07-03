#version 330 core

in vec2 TexPosition;
out vec4 color;

uniform vec3 resolution;
uniform sampler2D texture0;
uniform uint Time;

void main(void) {
    vec3 uv = gl_FragCoord.xyz / resolution.xyz;
    float k = cos(TexPosition.x * 10.0 + Time/100.0);
    vec3 l = vec3(mix(0.6, 1.0, k));
    vec4 light = vec4(l, 1.0);
    color = texture(texture0, TexPosition) * light;
}