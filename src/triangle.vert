#version 330 core

uniform float theta;
in vec2 position;
in vec3 color;
out vec3 _color;

vec2 rotate(vec2 position, float theta) {
    return vec2(
        position.x * cos(theta) - position.y * sin(theta),
        position.x * sin(theta) + position.y * cos(theta)
    );
}

void main()
{
    vec2 rotated = rotate(position, theta);
    gl_Position = vec4(rotated, 0.0, 1.0);
    _color = color;
}