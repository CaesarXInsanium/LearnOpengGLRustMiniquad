#version 450 core
in vec2 vTexCoord;

uniform sampler2D tex;
    
out vec4 FragColor;

void main() {
    FragColor = texture(tex, vTexCoord);
}
