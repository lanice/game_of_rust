#version 140

in vec2 position;

uniform float width;
uniform float height;

void main() {
    float dxy = height / width;
    float dyx = width / height;

    gl_Position = mix(
        vec4(position.x * dxy, position.y, 0.0, 1.0),
        vec4(position.x, position.y * dyx, 0.0, 1.0),
        step(width, height)
    );
}