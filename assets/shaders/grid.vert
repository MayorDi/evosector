#version 330 core

layout (location = 0) in vec3 vert_pos;
layout (location = 1) in vec2 tex_pos;

out vec2 TexPosition;

uniform vec3 resolution;
uniform vec2 camera_pos;
uniform float camera_scale;


void main(void) {
    vec4 n_cam_pos = vec4(camera_pos.xy / resolution.xy, 0.0, 1.0);
    vec4 n_pos = vec4(vert_pos.xy / resolution.xy, 0.0, 1.0);

    mat4 transform_matrix = mat4(
        1.0,            0.0,            0.0, 0.0,
        0.0,            1.0,            0.0, 0.0,
        0.0,            0.0,            1.0, 0.0,
        n_cam_pos.x,    -n_cam_pos.y,   0.0, 1.0
    );

    mat4 scale_matrix = mat4(
        camera_scale,   0.0,            0.0, 0.0,
        0.0,            -camera_scale,  0.0, 0.0,
        0.0,            0.0,            1.0, 0.0,
        0.0,            0.0,            0.0, 1.0
    );

    vec4 pos = transform_matrix * scale_matrix * n_pos;
    TexPosition = tex_pos;
    gl_Position = vec4(pos);
}