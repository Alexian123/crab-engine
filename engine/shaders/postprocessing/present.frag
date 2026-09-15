#version 330 core

in vec2 vUV;

out vec4 FragColor;

uniform sampler2D uColorTexture;

void main(void) {
    FragColor = texture(uColorTexture, vUV);
}
