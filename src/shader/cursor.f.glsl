#version 330 core

out vec4 color;

uniform vec4 cursorColor;

void main() {
    color = vec4(cursorColor);
}

