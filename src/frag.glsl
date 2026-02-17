#version 140

in vec2 v_uv;

uniform sampler2D text;

void main() {
    gl_FragColor = texture2D(text, v_uv);
}