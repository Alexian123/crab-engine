#ifndef MATERIAL_GLSL
#define MATERIAL_GLSL

#define MAX_NUM_TEXTURES       16

struct Material
{
    uint indexMask; // b[0:3] = diffuseIndex, b[4:7] = specularIndex, b[8:11] = emissionIndex, b[12:15] = free
    uint useMask; // b[0] = useDiffuse, b[1] = useSpecular, b[2] = useEmission, b[3:15] = free
    float shininess;
};

uniform sampler2D uTextures[MAX_NUM_TEXTURES];
uniform Material uMaterial;

int getDiffuseIndex(uint mask)
{
    return int(mask & 0xFu);
}

int getSpecularIndex(uint mask)
{
    return int((mask >> 4) & 0xFu);
}

int getEmissionIndex(uint mask)
{
    return int((mask >> 8) & 0xFu);
}

int getUseDiffuse(uint mask)
{
    return int((mask >> 0) & 1u);
}

int getUseSpecular(uint mask)
{
    return int((mask >> 1) & 1u);
}

int getUseEmission(uint mask)
{
    return int((mask >> 2) & 1u);
}

#endif
