#version 330 core

uniform vec3 light_color;
uniform float light_ambient;
uniform vec3 light_position;

in vec4 _color;
in vec3 _light_direction;
in vec3 _normal;
in vec3 _position;
out vec4 color;

void main()
{
    vec3 light_direction = normalize(light_position - _position);
    float brightness = mix(light_ambient, 1.0, max(0.0, dot(_normal, light_direction)));
    color = vec4(brightness * _color.xyz, _color.w);
}