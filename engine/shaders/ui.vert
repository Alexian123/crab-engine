#version 330 core

layout(location = 0) in vec2 aPos;

out vec2 vUV;

uniform mat4 uTransform;

void main(void) {
   	gl_Position = uTransform * vec4(aPos, 0.0, 1.0);
   	vUV = vec2((aPos.x + 1.0) / 2.0, 1.0 - (aPos.y + 1.0) / 2.0);
}
