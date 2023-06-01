#version 450 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;

uniform mat4 uTransform;

out vec2 vTexCoord;

vec2 texture_coord_flip(vec2 coord){
  return vec2(coord.x, abs(1 - coord.y));
}

void main() {
    gl_Position = uTransform * vec4(aPos, 1.0);
    vTexCoord = texture_coord_flip(aTexCoord);
}
