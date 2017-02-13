#version 140

uniform float dt;
uniform float x_scale;
uniform float y_scale;

in vec2 position;

out vec3 vColor;

void main() {
    float x = (position.x - dt) * x_scale - 1.0;
    float y = position.y * y_scale;

    gl_Position = vec4(x, y, 0.0, 1.0);

    vColor = vec3(0.0, 0.0, 0.0);
}