#version 330 core

layout(location = 0) in vec2 aPos;
layout(location = 1) in vec2 aUV;

out vec2 vUV[11];

uniform float uTargetWidth;

void main(void) {
	gl_Position = vec4(aPos, 0.0, 1.0);
	float pixelSize = 1.0 / uTargetWidth;

	for (int i = -5; i <= 5; i++) {
	    vUV[i + 5] = aUV + vec2(i * pixelSize, 0.0);
	}
}
