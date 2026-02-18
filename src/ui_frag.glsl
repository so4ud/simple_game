#version 140

in vec2 v_uv;
in vec4 v_color;

void main() {
    gl_FragColor = v_color;
}