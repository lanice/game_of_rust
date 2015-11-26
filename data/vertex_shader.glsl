#version 140

in vec2 position;

uniform float width;
uniform float height;

void main() {
    float dxy = height / width;

    gl_Position = vec4(position.x * dxy, position.y, 0.0, 1.0);
}