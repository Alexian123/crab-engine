#ifndef CAMERA_GLSL
#define CAMERA_GLSL

layout (std140) uniform CameraData {
    mat4 uProjection;
    mat4 uView;
    vec3 uViewPos;
    float _CameraData_pad0;
};

#endif
