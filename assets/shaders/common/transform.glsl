#ifndef TRANSFORM_GLSL
#define TRANSFORM_GLSL

layout (std140) uniform TransformData {
    mat4 uModel;
    mat4 uNormal;
};

#endif
