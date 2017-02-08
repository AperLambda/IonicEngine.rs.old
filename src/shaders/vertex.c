#version 130

in vec2 position;

uniform vec2 modifier;

out vec2 texcoord;

void main() {
	gl_Position = vec4(position+modifier, 0.0, 1.0);
	texcoord = vec2(gl_Position.x * 0.5 + 0.5, gl_Position.y * 0.5 + 0.5);
}