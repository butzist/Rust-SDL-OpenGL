#version 330 core

uniform mat4 transform;
uniform mat3 normal_transform;
in vec3 position;
in vec3 normal;
in vec4 color;
out vec4 _color;
out vec3 _normal;
out vec3 _position;

void main()
{
    vec4 transformed = transform * vec4(position, 1.0);
    gl_Position = transformed;

    _position = vec3(transformed);
    _normal = normal_transform * normal;
    _color = color;
}