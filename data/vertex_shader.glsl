#version 140

in vec2 position;
in vec2 tex_coords;

out vec2 v_tex_coords;

uniform float width;
uniform float height;

void main() {
    float dxy = height / width;
    float dyx = width / height;

    v_tex_coords = tex_coords;

    gl_Position = mix(
        vec4(position.x * dxy, position.y, 0.0, 1.0),
        vec4(position.x, position.y * dyx, 0.0, 1.0),
        step(width, height)
    );
}