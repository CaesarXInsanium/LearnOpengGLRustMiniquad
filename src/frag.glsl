#version 450 core

in vec2 vTexCoord;

uniform sampler2D tex01;
uniform sampler2D tex02;
    
out vec4 FragColor;

void main() {
    FragColor = mix(
    texture(tex01, vTexCoord),
    texture(tex02, vTexCoord),
    0.5
  );
}
