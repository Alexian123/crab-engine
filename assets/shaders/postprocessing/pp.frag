#version 330 core

in vec2 vUV;

out vec4 FragColor;

uniform sampler2D uScreenTexture;

const float offset = 1.0 / 300.0;

void main()
{
    // invert colors
    //FragColor = vec4(vec3(1.0 - texture(uScreenTexture, vUV)), 1.0);

    // grayscale
    // FragColor = texture(uScreenTexture, vUV);
    // float average = (FragColor.r + FragColor.g + FragColor.b) / 3.0;
    // FragColor = vec4(average, average, average, 1.0);

    // kernel effects
    vec2 offsets[9] = vec2[](
        vec2(-offset,  offset), // top-left
        vec2( 0.0f,    offset), // top-center
        vec2( offset,  offset), // top-right
        vec2(-offset,  0.0f),   // center-left
        vec2( 0.0f,    0.0f),   // center-center
        vec2( offset,  0.0f),   // center-right
        vec2(-offset, -offset), // bottom-left
        vec2( 0.0f,   -offset), // bottom-center
        vec2( offset, -offset)  // bottom-right
    );

    vec3 sampleTex[9];
    for(int i = 0; i < 9; i++)
    {
        sampleTex[i] = vec3(texture(uScreenTexture, vUV.st + offsets[i]));
    }

    // sharpen kernel
    // float kernel[9] = float[](
    //     -1, -1, -1,
    //     -1,  9, -1,
    //     -1, -1, -1
    // );
    // vec3 col = vec3(0.0);
    // for(int i = 0; i < 9; i++)
    //     col += sampleTex[i] * kernel[i];
    // FragColor = vec4(col, 1.0);


    // blur kernel
    // float kernel[9] = float[](
    //     1.0 / 16, 2.0 / 16, 1.0 / 16,
    //     2.0 / 16, 4.0 / 16, 2.0 / 16,
    //     1.0 / 16, 2.0 / 16, 1.0 / 16
    // );
    // vec3 col = vec3(0.0);
    // for(int i = 0; i < 9; i++)
    //     col += sampleTex[i] * kernel[i];
    // FragColor = vec4(col, 1.0);


    // edge detection
    float kernel[9] = float[](
        1, 1, 1,
        1, -8, 1,
        1, 1, 1
    );
    vec3 col = vec3(0.0);
    for(int i = 0; i < 9; i++)
        col += sampleTex[i] * kernel[i];
    FragColor = vec4(col, 1.0);
}
