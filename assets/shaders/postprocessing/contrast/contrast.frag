#version 330 core

in vec2 vUV;

out vec4 FragColor;

uniform sampler2D uColorTexture;
uniform float uContrast;

void main(void) {
	FragColor = texture(uColorTexture, vUV);
	FragColor.rgb = (FragColor.rgb - 0.5) * (1.0 + uContrast) + 0.5;
}
