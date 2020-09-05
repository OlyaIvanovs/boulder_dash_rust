#version 330 core

layout(location = 0) in vec3 Position;
layout(location = 1) in vec3 Color;

out vec3 color;

uniform float angle;

void main() {
    mat4 rotationZ = mat4(
        cos(angle), -sin(angle), 0, 0,
        sin(angle), cos(angle), 0, 0,
        0, 0, 1, 0, 
        0, 0, 0, 1);

    gl_Position = rotationZ * vec4(Position, 1.0);
    color = Color;

}