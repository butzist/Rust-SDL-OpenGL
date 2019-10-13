#version 330 core

uniform mat4 transform;
in vec3 position;
in vec4 color;
out vec4 _color;

void main()
{
    gl_Position = transform * vec4(position, 1.0);
    _color = color;
}