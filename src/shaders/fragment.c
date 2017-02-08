#version 130

in vec2 texcoord;

uniform float color;

out vec4 outColor;

void main() {
	outColor = vec4(
		1.0 * color + 0.8 * texcoord.x,
		0.7 * color + 0.8 * texcoord.y,
		0.4 * color, 1.0);
}