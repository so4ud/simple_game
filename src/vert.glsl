#version 140 

in vec3 position;
in vec3 normal;
in vec2 uv;

out vec2 v_uv;
out vec3 v_normal;

uniform mat4 m;
uniform mat4 v;
uniform mat4 p;

void main() {
    mat4 mat = mat4(
        vec4(1, 0, 0, 0),
        vec4(0, 1, 0, 0),
        vec4(0, 0, 1, 0),
        vec4(0, 0, 0, 1)
    );
    mat *= p;
    mat *= v;
    mat *= m;


    gl_Position = mat * vec4(position, 1.0);

    v_normal = normal;
    v_uv = vec2(uv);
}