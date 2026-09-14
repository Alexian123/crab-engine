#version 330 core

#include "common/camera.glsl"
#include "common/material.glsl"
#include "common/lighting.glsl"

in vec3 vColor;
in vec2 vUV;
in vec3 vNormal;
in vec3 vFragPos;

out vec4 FragColor;

void main() {
    int diffuseIndex = getDiffuseIndex(uMaterial.indexMask);
    int specularIndex = getSpecularIndex(uMaterial.indexMask);
    int emissionIndex = getEmissionIndex(uMaterial.indexMask);

    int useDiffuse = getUseDiffuse(uMaterial.useMask);
    int useSpecular = getUseSpecular(uMaterial.useMask);
    int useEmission = getUseEmission(uMaterial.useMask);

    Surface surface;
    surface.diffuseColor = texture(uTextures[diffuseIndex], vUV).rgb * useDiffuse;
    surface.specularColor = texture(uTextures[specularIndex], vUV).rgb * useSpecular;
    surface.normal = normalize(vNormal);
    surface.fragPos = vFragPos;
    surface.viewDir = normalize(uViewPos - vFragPos);
    surface.shininess = uMaterial.shininess;

    int numDirLights = getNumDirLights(uNumLightsMask);
    int numPointLights = getNumPointLights(uNumLightsMask);
    int numSpotLights = getNumSpotLights(uNumLightsMask);

    // directional lights
    vec3 result = vec3(0.0);
    for (int i = 0; i < MAX_NUM_DIR_LIGHTS; ++i) {
        if (i >= numDirLights) break;
        result += calculateDirLight(uDirLights[i], surface);
    }

    // point lights
    for (int i = 0; i < MAX_NUM_POINT_LIGHTS; ++i) {
        if (i >= numPointLights) break;
        result += calculatePointLight(uPointLights[i], surface);
    }

    // spot lights
    for (int i = 0; i < MAX_NUM_SPOT_LIGHTS; ++i) {
        if (i >= numSpotLights) break;
        result += calculateSpotLight(uSpotLights[i], surface);
    }

    // self light (emission)
    result += texture(uTextures[emissionIndex], vUV).rgb * useEmission;

    FragColor = vec4(result, 1.0);
}
