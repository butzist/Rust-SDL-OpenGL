#version 330 core

uniform mat4 transform;
in vec3 position;
in vec3 color;
out vec3 _color;

void main()
{
    gl_Position = transform * vec4(position, 1.0);
    _color = color;
}