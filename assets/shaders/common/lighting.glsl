#ifndef LIGHTING_GLSL
#define LIGHTING_GLSL

struct Light {
    vec3 position;
    vec3 color;
    float intensity;
};

void computeLighting(Light light) {
}

#endif
