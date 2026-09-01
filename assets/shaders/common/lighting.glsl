#ifndef LIGHTING_GLSL
#define LIGHTING_GLSL

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
    vec3 diffuse;
    vec3 specular;
};

struct DirLight
{
    LightColor color;
    vec3 direction;
};

struct PointLight
{
    LightColor color;
    vec3 position;
    float constant;
    float linear;
    float quadratic;
};

struct SpotLight
{
    PointLight pl;
    vec3 direction;
    float cutOff;
    float outerCutOff;
};

float calculateDiffuse(vec3 normal, vec3 lightDir)
{
    return max(dot(normal, lightDir), 0.0);
}

float calculateSpecular(vec3 normal, vec3 lightDir, vec3 viewDir, float shininess)
{
    vec3 reflectDir = reflect(-lightDir, normal);
    return pow(max(dot(viewDir, reflectDir), 0.0), shininess);
}

float calculateAttenuation(float constant, float linear, float quadratic, float distance)
{
    return 1.0 / (constant + linear * distance + quadratic * distance * distance);
}

float calculateSpotIntensity(vec3 lightDir, vec3 spotDirection, float cutOff, float outerCutOff)
{
    float theta = dot(lightDir, normalize(-spotDirection));
    float epsilon = cutOff - outerCutOff;
    return clamp((theta - outerCutOff) / epsilon, 0.0, 1.0);
}

vec3 calculateLight(LightColor lightColor, Surface surface, vec3 lightDir)
{
    float diff = calculateDiffuse(surface.normal, lightDir);
    float spec = calculateSpecular(surface.normal, lightDir, surface.viewDir, surface.shininess);
    vec3 ambient = lightColor.ambient * surface.diffuseColor;
    vec3 diffuse = lightColor.diffuse * diff * surface.diffuseColor;
    vec3 specular = lightColor.specular * spec * surface.specularColor;
    return ambient + diffuse + specular;
}

vec3 calculateDirLight(DirLight light, Surface surface)
{
    vec3 lightDir = normalize(-light.direction);
    return calculateLight(light.color, surface, lightDir);
}

vec3 calculatePointLight(PointLight light, Surface surface)
{
    vec3 lightDir = normalize(light.position - surface.fragPos);
    float distance = length(light.position - surface.fragPos);
    float attenuation = calculateAttenuation(light.constant, light.linear, light.quadratic, distance);
    return calculateLight(light.color, surface, lightDir) * attenuation;
}

vec3 calculateSpotLight(SpotLight light, Surface surface)
{
    vec3 lightDir = normalize(light.pl.position - surface.fragPos);
    float distance = length(light.pl.position - surface.fragPos);
    float attenuation = calculateAttenuation(light.pl.constant, light.pl.linear, light.pl.quadratic, distance);
    float intensity = calculateSpotIntensity(lightDir, light.direction, light.cutOff, light.outerCutOff);
    return calculateLight(light.pl.color, surface, lightDir) * attenuation * intensity;
}

#endif
