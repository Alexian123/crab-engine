#version 330 core

in vec2 vUV;
in vec3 vNormal;
in vec3 vFragPos;

out vec4 OutColor;

uniform vec3 uColor;
uniform sampler2D uDiffuseMap;

void main() {
    vec3 normal = normalize(vNormal);

	vec4 texColor = texture(uDiffuseMap, vUV);

	vec3 result = texColor.xyz * uColor;

	OutColor = vec4(result, 1.0);
}
