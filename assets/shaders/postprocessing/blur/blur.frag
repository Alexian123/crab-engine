#version 330 core

in vec2 vUV[11];

out vec4 FragColor;

uniform sampler2D uColorTexture;

void main(void) {
    FragColor = vec4(0.0);
	FragColor += texture(uColorTexture, vUV[0]) * 0.0093;
    FragColor += texture(uColorTexture, vUV[1]) * 0.028002;
    FragColor += texture(uColorTexture, vUV[2]) * 0.065984;
    FragColor += texture(uColorTexture, vUV[3]) * 0.121703;
    FragColor += texture(uColorTexture, vUV[4]) * 0.175713;
    FragColor += texture(uColorTexture, vUV[5]) * 0.198596;
    FragColor += texture(uColorTexture, vUV[6]) * 0.175713;
    FragColor += texture(uColorTexture, vUV[7]) * 0.121703;
    FragColor += texture(uColorTexture, vUV[8]) * 0.065984;
    FragColor += texture(uColorTexture, vUV[9]) * 0.028002;
    FragColor += texture(uColorTexture, vUV[10]) * 0.0093;
}
