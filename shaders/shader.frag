#version 330 core

in vec2 tex_coords;
out vec4 Color;

uniform sampler2D ourTexture;

void main() {
    Color = texture(ourTexture, tex_coords);
}