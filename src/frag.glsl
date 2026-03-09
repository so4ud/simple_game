#version 140

in vec2 v_uv;

uniform sampler2D text;

void main() {
    gl_FragColor = texture2D(text, v_uv);

    // float x = sin(gl_FragCoord.x);
    // float y = cos(gl_FragCoord.y);
    // float mult = x * y;


    // gl_FragColor = vec4(mult);
}