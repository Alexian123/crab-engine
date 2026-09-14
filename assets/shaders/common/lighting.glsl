#ifndef LIGHTING_GLSL
#define LIGHTING_GLSL

#define MAX_NUM_DIR_LIGHTS      4
#define MAX_NUM_POINT_LIGHTS    8
#define MAX_NUM_SPOT_LIGHTS     2

struct Surface
{
    vec3 diffuseColor;
    vec3 specularColor;
    vec3 normal;
    vec3 fragPos;
    vec3 viewDir;
    float shininess;
};

struct LightColor
{
    vec3 ambient;
    float _LightColor_pad0;

    vec3 diffuse;
    float _LightColor_pad1;

    vec3 specular;
    float _LightColor_pad2;
};

struct DirLight
{
    LightColor color;
    vec3 direction;
    float _DirLight_pad0;
};

struct PointLight
{
    LightColor color;

    vec3 position;
    float constant;

    float linear;
    float quadratic;
    float _PointLight_pad0;
    float _PointLight_pad1;
};

struct SpotLight
{
    PointLight pl;

    vec3 direction;
    float cutOff;

    float outerCutOff;
    float _SpotLight_pad0;
    float _SpotLight_pad1;
    float _SpotLight_pad2;
};

layout (std140) uniform LightingData {
    DirLight uDirLights[MAX_NUM_DIR_LIGHTS];
    PointLight uPointLights[MAX_NUM_POINT_LIGHTS];
    SpotLight uSpotLights[MAX_NUM_SPOT_LIGHTS];
    uint uNumLightsMask; // b[0:7] = numDirLights, b[8:15] = numPointLights, b[16:23] = numSpotLights, b[24:31] = free
    uint _LightData_pad0;
    uint _LightData_pad1;
    uint _LightData_pad2;
};

float calculateDiffuse(vec3 normal, vec3 lightDir) {
    return max(dot(normal, lightDir), 0.0);
}

float calculateSpecular(vec3 normal, vec3 lightDir, vec3 viewDir, float shininess) {
    vec3 reflectDir = reflect(-lightDir, normal);
    return pow(max(dot(viewDir, reflectDir), 0.0), shininess);
}

float calculateAttenuation(float constant, float linear, float quadratic, float distance) {
    return 1.0 / (constant + linear * distance + quadratic * distance * distance);
}

float calculateSpotIntensity(vec3 lightDir, vec3 spotDirection, float cutOff, float outerCutOff) {
    float theta = dot(lightDir, normalize(-spotDirection));
    float epsilon = cutOff - outerCutOff;
    return clamp((theta - outerCutOff) / epsilon, 0.0, 1.0);
}

vec3 calculateLight(LightColor lightColor, Surface surface, vec3 lightDir) {
    float diff = calculateDiffuse(surface.normal, lightDir);
    float spec = calculateSpecular(surface.normal, lightDir, surface.viewDir, surface.shininess);
    vec3 ambient = lightColor.ambient * surface.diffuseColor;
    vec3 diffuse = lightColor.diffuse * diff * surface.diffuseColor;
    vec3 specular = lightColor.specular * spec * surface.specularColor;
    return ambient + diffuse + specular;
}

vec3 calculateDirLight(DirLight light, Surface surface) {
    vec3 lightDir = normalize(-light.direction);
    return calculateLight(light.color, surface, lightDir);
}

vec3 calculatePointLight(PointLight light, Surface surface) {
    vec3 lightDir = normalize(light.position - surface.fragPos);
    float distance = length(light.position - surface.fragPos);
    float attenuation = calculateAttenuation(light.constant, light.linear, light.quadratic, distance);
    return calculateLight(light.color, surface, lightDir) * attenuation;
}

vec3 calculateSpotLight(SpotLight light, Surface surface) {
    vec3 lightDir = normalize(light.pl.position - surface.fragPos);
    float distance = length(light.pl.position - surface.fragPos);
    float attenuation = calculateAttenuation(light.pl.constant, light.pl.linear, light.pl.quadratic, distance);
    float intensity = calculateSpotIntensity(lightDir, light.direction, light.cutOff, light.outerCutOff);
    return calculateLight(light.pl.color, surface, lightDir) * attenuation * intensity;
}

int getNumDirLights(uint mask) {
    return int(mask & 0xFFu);
}

int getNumPointLights(uint mask) {
    return int((mask >> 8) & 0xFFu);
}

int getNumSpotLights(uint mask) {
    return int((mask >> 16) & 0xFFu);
}

#endif
