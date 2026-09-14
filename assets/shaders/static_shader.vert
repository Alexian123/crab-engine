#version 330 core

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 aColor;
layout(location = 2) in vec2 aUV;
layout(location = 3) in vec3 aNormal;

out vec3 vColor;
out vec2 vUV;
out vec3 vNormal;
out vec3 vFragPos;

layout (std140) uniform CameraMatrices {
    mat4 uProjection;
    mat4 uView;
};

uniform mat4 uModel;
uniform mat4 uNormal;

void main() {
    vec4 fragPos = uModel * vec4(aPos, 1.0);
    gl_Position = uProjection * uView * fragPos;
    vColor = aColor;
    vUV = aUV;
    vNormal = mat3(uNormal) * aNormal;
    vFragPos = vec3(fragPos);
}
