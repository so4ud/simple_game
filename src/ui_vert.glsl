#version 140 

in vec3 position;
in vec3 color;
in vec2 uv;

out vec2 v_uv;
out vec4 v_color;

uniform mat4 mat;

void main() {
    gl_Position = mat * vec4(position, 1.0);
    v_color = vec4(color, 1);
    v_uv = uv;
}