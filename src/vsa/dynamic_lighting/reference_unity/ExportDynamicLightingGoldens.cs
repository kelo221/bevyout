using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using UnityEditor;
using UnityEngine;

namespace AlpacaIT.DynamicLighting
{
    public partial class DynamicLightManager
    {
        [Serializable]
        private sealed class Sample
        {
            public int frame;
            public float time;
            public float delta;
            public float intensity;
        }

        [Serializable]
        private sealed class EffectTrace
        {
            public string effect;
            public int discriminant;
            public int seed;
            public string schedule;
            public List<Sample> samples = new List<Sample>();
        }

        [Serializable]
        private sealed class EffectsFixture
        {
            public string upstreamCommit;
            public string unityVersion;
            public List<EffectTrace> traces = new List<EffectTrace>();
        }

        [Serializable]
        private sealed class DefaultsFixture
        {
            public string upstreamCommit;
            public string unityVersion;
            public float intensity = 2.0f;
            public float radius = 4.0f;
            public float falloff = 0.0f;
            public float innerCutoffDegrees = 26.0f;
            public float outerCutoffDegrees = 30.0f;
            public float waveSpeed = 1.0f;
            public float waveFrequency = 1.0f;
            public float waveOffset = 0.0f;
            public float rotorCenter = 0.1f;
            public float discoVerticalSpeed = 1.0f;
            public float pulseSpeed = 1.0f;
            public float pulseModifier = 0.25f;
            public float pulseOffset = 0.0f;
            public float timestepSeconds = 1.0f / 30.0f;
            public float bounceModifier = 1.0f;
            public float bounceIntensity = 1.0f;
            public int volumetricType = 0;
            public float volumetricRadius = 4.0f;
            public float volumetricThickness = 1.0f;
            public float volumetricIntensity = 0.75f;
            public float volumetricVisibility = 2.0f;
        }

        [Serializable]
        private sealed class PerlinSample
        {
            public float x;
            public float y;
            public float value;
        }

        [Serializable]
        private sealed class PerlinFixture
        {
            public string upstreamCommit;
            public string unityVersion;
            public List<PerlinSample> samples = new List<PerlinSample>();
        }

        [Serializable]
        private sealed class MultiLightFrame
        {
            public int frame;
            public float time;
            public float[] intensities;
        }

        [Serializable]
        private sealed class MultiLightRandomFixture
        {
            public string upstreamCommit;
            public string unityVersion;
            public int seed;
            public string schedule;
            public int[] effectDiscriminants;
            public List<MultiLightFrame> frames = new List<MultiLightFrame>();
        }

        [Serializable]
        private sealed class SpatialSample
        {
            public string type;
            public int discriminant;
            public float time;
            public float[] world;
            public float value;
        }

        [Serializable]
        private sealed class SpatialFixture
        {
            public string upstreamCommit;
            public string unityVersion;
            public List<SpatialSample> samples = new List<SpatialSample>();
        }

        [Serializable]
        private sealed class VolumetricSample
        {
            public string type;
            public int discriminant;
            public float[] camera;
            public float[] world;
            public float[] center;
            public float[] direction;
            public float[] scale;
            public float radius;
            public float thickness;
            public float intensity;
            public float visibility;
            public float temporalMultiplier;
            public float coneAngle;
            public float opacity;
            public float[] outputRgb;
        }

        [Serializable]
        private sealed class VolumetricFixture
        {
            public string upstreamCommit;
            public string unityVersion;
            public List<VolumetricSample> samples = new List<VolumetricSample>();
        }

        private const string UpstreamCommit = "dd7c195cba2599a20bf1b662fa0f69366e0f74b5";

        private static readonly DynamicLightEffect[] Effects =
        {
            DynamicLightEffect.Steady,
            DynamicLightEffect.Pulse,
            DynamicLightEffect.Random,
            DynamicLightEffect.Strobe,
            DynamicLightEffect.Flicker,
            DynamicLightEffect.FluorescentStarter,
            DynamicLightEffect.FluorescentClicker,
            DynamicLightEffect.FluorescentRandom,
            DynamicLightEffect.Candle,
            DynamicLightEffect.Pulsar,
            DynamicLightEffect.Fire,
            DynamicLightEffect.Generator,
            DynamicLightEffect.Lightning,
            DynamicLightEffect.Cloudy,
            DynamicLightEffect.Overcast,
        };

        [MenuItem("Tools/bevyout/Export DynamicLighting Goldens")]
        public static void ExportGoldens()
        {
            string outputDirectory = Environment.GetEnvironmentVariable("BEVYOUT_DYNAMIC_LIGHTING_GOLDEN_DIR");
            if (String.IsNullOrWhiteSpace(outputDirectory))
                throw new InvalidOperationException("BEVYOUT_DYNAMIC_LIGHTING_GOLDEN_DIR is required");

            Directory.CreateDirectory(outputDirectory);
            var fixture = new EffectsFixture
            {
                upstreamCommit = UpstreamCommit,
                unityVersion = Application.unityVersion,
            };

            foreach (int seed in new[] { 1, 12345 })
            {
                foreach (DynamicLightEffect effect in Effects)
                {
                    fixture.traces.Add(Capture(effect, seed, "30hz", Repeat(1.0f / 30.0f, 211)));
                    fixture.traces.Add(Capture(effect, seed, "60hz", Repeat(1.0f / 60.0f, 421)));
                    fixture.traces.Add(Capture(effect, seed, "120hz", Repeat(1.0f / 120.0f, 841)));
                    fixture.traces.Add(Capture(effect, seed, "jitter", Jittered(7.0f)));
                }
            }

            WriteJson(Path.Combine(outputDirectory, "unity_effects_v1.json"), fixture);
            WriteJson(Path.Combine(outputDirectory, "unity_defaults_v1.json"), new DefaultsFixture
            {
                upstreamCommit = UpstreamCommit,
                unityVersion = Application.unityVersion,
            });
            WriteJson(Path.Combine(outputDirectory, "unity_perlin_v1.json"), CapturePerlin());
            WriteJson(
                Path.Combine(outputDirectory, "unity_multilight_random_v1.json"),
                CaptureMultiLightRandom());
            WriteJson(Path.Combine(outputDirectory, "unity_spatial_v1.json"), CaptureSpatial());
            WriteJson(
                Path.Combine(outputDirectory, "unity_volumetric_v1.json"),
                CaptureVolumetric());

            Debug.Log(String.Format(
                CultureInfo.InvariantCulture,
                "bevyout DynamicLighting goldens exported: {0} traces to {1}",
                fixture.traces.Count,
                outputDirectory));
        }

        private static PerlinFixture CapturePerlin()
        {
            var fixture = new PerlinFixture
            {
                upstreamCommit = UpstreamCommit,
                unityVersion = Application.unityVersion,
            };
            float[] coordinates =
            {
                -2.75f, -1.0f, -0.5f, 0.0f, 0.01f, 0.05f, 0.1f, 0.25f,
                0.5f, 0.75f, 1.0f, 1.25f, 2.0f, 5.0f, 7.8f, 10.0f,
                20.0f, 121.81f, 281.24f,
            };
            foreach (float x in coordinates)
            foreach (float y in coordinates)
            {
                fixture.samples.Add(new PerlinSample
                {
                    x = x,
                    y = y,
                    value = Mathf.PerlinNoise(x, y),
                });
            }
            return fixture;
        }

        private static MultiLightRandomFixture CaptureMultiLightRandom()
        {
            const int seed = 24680;
            const float delta = 1.0f / 60.0f;
            DynamicLightEffect[] effects =
            {
                DynamicLightEffect.Random,
                DynamicLightEffect.Flicker,
                DynamicLightEffect.FluorescentRandom,
                DynamicLightEffect.Random,
            };
            var fixture = new MultiLightRandomFixture
            {
                upstreamCommit = UpstreamCommit,
                unityVersion = Application.unityVersion,
                seed = seed,
                schedule = "60hz-stable-source-order",
                effectDiscriminants = Array.ConvertAll(effects, effect => (int)effect),
            };
            var manager = new DynamicLightManager();
            var lights = Array.ConvertAll(effects, effect => new DynamicLight { lightEffect = effect });
            UnityEngine.Random.InitState(seed);

            float elapsed = 0.0f;
            for (int frame = 0; frame < 241; frame++)
            {
                manager.deltaTime = delta;
                manager.timeTime = elapsed;
                foreach (DynamicLight light in lights)
                {
                    light.cache.fixedTimestep.timePerStep = light.lightEffectTimestepFrequency;
                    light.cache.fixedTimestep.Update(delta);
                    manager.UpdateReferenceEffect(light);
                }
                if (frame < 20 || frame % 15 == 0 || frame == 240)
                {
                    fixture.frames.Add(new MultiLightFrame
                    {
                        frame = frame,
                        time = elapsed,
                        intensities = Array.ConvertAll(lights, light => light.cache.intensity),
                    });
                }
                elapsed += delta;
            }
            return fixture;
        }

        private static SpatialFixture CaptureSpatial()
        {
            string[] typeNames =
            {
                "Point", "Spot", "Discoball", "Wave",
                "Interference", "Rotor", "Shock", "Disco",
            };
            Vector3[] worlds =
            {
                new Vector3(0.0f, 0.0f, -1.0f),
                new Vector3(0.2f, 0.0f, -1.0f),
                new Vector3(1.0f, 0.0f, -1.0f),
                new Vector3(0.0f, 1.0f, -1.0f),
                new Vector3(-1.0f, 0.5f, -2.0f),
                new Vector3(2.0f, 0.25f, -1.0f),
                new Vector3(1.0f, 2.0f, -3.0f),
            };
            float[] times = { 0.0f, 0.25f, 0.75f };
            var fixture = new SpatialFixture
            {
                upstreamCommit = UpstreamCommit,
                unityVersion = Application.unityVersion,
            };

            for (int discriminant = 0; discriminant < typeNames.Length; discriminant++)
            foreach (float time in times)
            foreach (Vector3 world in worlds)
            {
                fixture.samples.Add(new SpatialSample
                {
                    type = typeNames[discriminant],
                    discriminant = discriminant,
                    time = time,
                    world = new[] { world.x, world.y, world.z },
                    value = CalculateSpatial(discriminant, time, world),
                });
            }
            return fixture;
        }

        private static VolumetricFixture CaptureVolumetric()
        {
            var fixture = new VolumetricFixture
            {
                upstreamCommit = UpstreamCommit,
                unityVersion = Application.unityVersion,
            };
            Vector3 center = Vector3.zero;
            Vector3 scale = new Vector3(1.0f, 2.0f, 0.5f);
            Vector3 sourceColor = new Vector3(0.15f, 0.25f, 0.4f);
            Vector3 fogColor = new Vector3(0.8f, 0.3f, 0.1f);
            AddVolumetricSample(fixture, "None", 0, new Vector3(0, 0, -8), new Vector3(0, 0, 8), center, Vector3.forward, scale, 4, 1, 0.75f, 2, 1, 30, sourceColor, fogColor);
            AddVolumetricSample(fixture, "Sphere", 1, new Vector3(0, 0, -8), new Vector3(0, 0, 8), center, Vector3.forward, scale, 4, 1, 0.75f, 2, 1, 30, sourceColor, fogColor);
            AddVolumetricSample(fixture, "Sphere", 1, new Vector3(5, 0, -8), new Vector3(5, 0, 8), center, Vector3.forward, scale, 4, 2, 0.5f, 3, 0.25f, 30, sourceColor, fogColor);
            AddVolumetricSample(fixture, "Box", 2, new Vector3(0, 0, -8), new Vector3(0, 0, 8), center, Vector3.forward, scale, 4, 2, 0.75f, 2, 1, 30, sourceColor, fogColor);
            AddVolumetricSample(fixture, "Box", 2, new Vector3(6, 0, -8), new Vector3(6, 0, 8), center, Vector3.forward, scale, 4, 2, 0.75f, 2, 1, 30, sourceColor, fogColor);
            AddVolumetricSample(fixture, "ConeZ", 3, new Vector3(0, 0, -8), new Vector3(0, 0, 8), center, Vector3.forward, Vector3.one, 4, 2, 0.75f, 2, 1, 30, sourceColor, fogColor);
            AddVolumetricSample(fixture, "ConeZ", 3, new Vector3(3, 0, -8), new Vector3(3, 0, 8), center, Vector3.forward, Vector3.one, 4, 2, 0.75f, 2, 1, 30, sourceColor, fogColor);
            AddVolumetricSample(fixture, "ConeY", 4, new Vector3(0, -8, 0), new Vector3(0, 8, 0), center, Vector3.up, Vector3.one, 4, 2, 0.75f, 2, 1, 30, sourceColor, fogColor);
            AddVolumetricSample(fixture, "ConeY", 4, new Vector3(0, -8, 3), new Vector3(0, 8, 3), center, Vector3.up, Vector3.one, 4, 2, 0.75f, 2, 1, 30, sourceColor, fogColor);
            return fixture;
        }

        private static void AddVolumetricSample(
            VolumetricFixture fixture,
            string type,
            int discriminant,
            Vector3 camera,
            Vector3 world,
            Vector3 center,
            Vector3 direction,
            Vector3 scale,
            float radius,
            float thickness,
            float intensity,
            float visibility,
            float temporalMultiplier,
            float outerCutoff,
            Vector3 sourceColor,
            Vector3 fogColor)
        {
            float coneAngle = Mathf.Cos((Mathf.PI * 0.5f) + outerCutoff * Mathf.Deg2Rad)
                * Mathf.Lerp(1.0f, 1.3f, outerCutoff / 90.0f);
            float opacity = CalculateVolumetricOpacity(
                discriminant,
                camera,
                world,
                center,
                direction,
                scale,
                radius,
                thickness,
                intensity * temporalMultiplier,
                visibility,
                coneAngle);
            Vector3 fog = fogColor * opacity;
            Vector3 screened = Vector3.one - Vector3.Scale(Vector3.one - fog, Vector3.one - sourceColor);
            Vector3 output = Vector3.Lerp(screened, fog, Mathf.Clamp01(opacity));
            fixture.samples.Add(new VolumetricSample
            {
                type = type,
                discriminant = discriminant,
                camera = ToArray(camera),
                world = ToArray(world),
                center = ToArray(center),
                direction = ToArray(direction),
                scale = ToArray(scale),
                radius = radius,
                thickness = thickness,
                intensity = intensity,
                visibility = visibility,
                temporalMultiplier = temporalMultiplier,
                coneAngle = coneAngle,
                opacity = opacity,
                outputRgb = ToArray(output),
            });
        }

        private static float CalculateVolumetricOpacity(
            int type,
            Vector3 camera,
            Vector3 world,
            Vector3 center,
            Vector3 direction,
            Vector3 scale,
            float radius,
            float thickness,
            float intensity,
            float visibility,
            float coneAngle)
        {
            if (type == 0 || radius <= 0.0f || intensity == 0.0f)
                return 0.0f;
            float t = 0.0f;
            if (type == 1)
            {
                Vector3 line = world - camera;
                float lineLengthSqr = Vector3.Dot(line, line);
                float lineT = lineLengthSqr <= 0.000001f
                    ? 0.0f
                    : Mathf.Clamp01(Vector3.Dot(center - camera, line) / lineLengthSqr);
                Vector3 closest = camera + line * lineT;
                float distanceToCenter = Vector3.Distance(closest, center);
                if (distanceToCenter < radius + 0.00001f)
                    t = (radius - distanceToCenter) / radius;
            }
            else
            {
                Vector3 ray = world - camera;
                float maxDepth = ray.magnitude;
                if (maxDepth > 0.000001f)
                {
                    float tMin;
                    float tMax;
                    bool hit = type == 2
                        ? RayBoxIntersection(camera, ray / maxDepth, center - radius * scale, center + radius * scale, maxDepth, out tMin, out tMax)
                        : RayConeIntersection(camera, ray / maxDepth, center, direction.normalized, coneAngle, radius, maxDepth, out tMin, out tMax);
                    if (hit)
                    {
                        float denominator = type == 2 ? (2.0f * radius * scale).magnitude : radius;
                        t = (tMax - tMin) / denominator;
                    }
                }
            }
            t = Mathf.SmoothStep(0.0f, 1.0f, t);
            t = Mathf.Clamp01(t * thickness);
            float inverseVisibility = 1.0f / Mathf.Max(visibility, 0.00001f);
            t = Mathf.Min(t, Vector3.Distance(camera, world) * inverseVisibility);
            return t * intensity;
        }

        private static bool RayBoxIntersection(
            Vector3 origin,
            Vector3 direction,
            Vector3 minimum,
            Vector3 maximum,
            float maxDepth,
            out float tMin,
            out float tMax)
        {
            Vector3 inverse = new Vector3(1.0f / direction.x, 1.0f / direction.y, 1.0f / direction.z);
            Vector3 first = Vector3.Scale(minimum - origin, inverse);
            Vector3 second = Vector3.Scale(maximum - origin, inverse);
            Vector3 near = Vector3.Min(first, second);
            Vector3 far = Vector3.Max(first, second);
            tMin = Mathf.Max(0.0f, Mathf.Max(near.x, Mathf.Max(near.y, near.z)));
            tMax = Mathf.Min(maxDepth, Mathf.Min(far.x, Mathf.Min(far.y, far.z)));
            return tMax >= tMin;
        }

        private static bool RayConeIntersection(
            Vector3 origin,
            Vector3 direction,
            Vector3 tip,
            Vector3 coneDirection,
            float coneAngle,
            float coneDistance,
            float maxDepth,
            out float tMin,
            out float tMax)
        {
            Vector3 w = origin - tip;
            float cosine = Mathf.Cos(coneAngle);
            float cosineSqr = cosine * cosine;
            float sineSqr = 1.0f - cosineSqr;
            float directionDotAxis = Vector3.Dot(direction, coneDirection);
            float wDotAxis = Vector3.Dot(w, coneDirection);
            float directionDotW = Vector3.Dot(direction, w);
            float wDotW = Vector3.Dot(w, w);
            float a = directionDotAxis * directionDotAxis - cosineSqr;
            float b = 2.0f * (directionDotAxis * wDotAxis - cosineSqr * directionDotW);
            float c = wDotAxis * wDotAxis - cosineSqr * wDotW;
            float discriminant = b * b - 4.0f * a * c;
            bool inside = wDotAxis >= 0.0f && wDotAxis <= coneDistance
                && wDotW - wDotAxis * wDotAxis <= wDotAxis * wDotAxis * sineSqr / Mathf.Max(cosineSqr, 0.000001f);
            var hits = new List<float>();
            if (discriminant >= 0.0f && Mathf.Abs(a) > 0.000001f)
            {
                float root = Mathf.Sqrt(discriminant);
                float first = (-b - root) / (2.0f * a);
                float second = (-b + root) / (2.0f * a);
                float firstHeight = wDotAxis + first * directionDotAxis;
                float secondHeight = wDotAxis + second * directionDotAxis;
                if (first >= 0.0f && firstHeight >= 0.0f && firstHeight <= coneDistance) hits.Add(first);
                if (second >= 0.0f && secondHeight >= 0.0f && secondHeight <= coneDistance) hits.Add(second);
            }
            float capDenominator = Vector3.Dot(direction, coneDirection);
            if (Mathf.Abs(capDenominator) > 0.0001f)
            {
                float cap = (coneDistance - wDotAxis) / capDenominator;
                if (cap >= 0.0f)
                {
                    Vector3 point = w + cap * direction;
                    float height = wDotAxis + cap * directionDotAxis;
                    float radialSqr = Vector3.Dot(point, point) - height * height;
                    float capRadiusSqr = coneDistance * coneDistance * sineSqr / Mathf.Max(cosineSqr, 0.000001f);
                    if (radialSqr <= capRadiusSqr) hits.Add(cap);
                }
            }
            if (!inside && hits.Count == 0)
            {
                tMin = tMax = 0.0f;
                return false;
            }
            hits.Sort();
            tMin = inside ? 0.0f : hits[0];
            tMax = hits.Count == 0 ? 0.0f : Mathf.Min(hits[hits.Count - 1], maxDepth);
            return tMax >= tMin;
        }

        private static float[] ToArray(Vector3 value)
        {
            return new[] { value.x, value.y, value.z };
        }

        // CPU transcription of the frozen DynamicLighting.cginc spatial
        // functions. Unity's Mathf/Vector3 implementations produce the golden
        // float values consumed by the Rust parity tests.
        private static float CalculateSpatial(int type, float time, Vector3 world)
        {
            const float radiusSqr = 16.0f;
            Vector3 forward = Vector3.forward;
            Vector3 up = Vector3.up;
            Vector3 lightMinusWorld = -world;
            Vector3 lightDirection = lightMinusWorld.normalized;
            float parameterA;
            float parameterB;
            float parameterC = 0.0f;

            switch (type)
            {
                case 0:
                    return 1.0f;
                case 1:
                case 2:
                    parameterA = Mathf.Cos(26.0f * Mathf.Deg2Rad);
                    parameterB = Mathf.Cos(30.0f * Mathf.Deg2Rad);
                    break;
                case 3:
                    parameterA = time;
                    parameterB = Mathf.PI * 2.0f;
                    break;
                case 4:
                    parameterA = time * Mathf.PI * 2.0f;
                    parameterB = Mathf.PI;
                    break;
                case 5:
                    parameterA = time * Mathf.PI * 2.0f;
                    parameterB = Mathf.Round(1.0f);
                    parameterC = 0.1f;
                    break;
                case 6:
                    parameterA = time;
                    parameterB = 1.0f;
                    break;
                case 7:
                    parameterA = time * Mathf.PI * 2.0f;
                    parameterB = Mathf.Round(1.0f);
                    parameterC = time * Mathf.PI * 2.0f;
                    break;
                default:
                    throw new ArgumentOutOfRangeException(nameof(type));
            }

            if (type == 1)
            {
                float theta = Vector3.Dot(lightDirection, forward);
                return Mathf.Clamp01((theta - parameterB) / (parameterA - parameterB));
            }
            if (type == 2)
            {
                Vector3 rotated = ToLightSpace(lightDirection, forward, up);
                float theta = Vector3.Dot(SnapDirection(rotated), rotated);
                return Mathf.Clamp01((theta - parameterB) / (parameterA - parameterB));
            }
            if (type == 3)
                return 0.7f + 0.3f * Mathf.Sin((world.magnitude - parameterA) * parameterB);

            Vector3 local = ToLightSpace(lightMinusWorld, forward, up);
            if (type == 4)
            {
                float angle = Mathf.Atan2(
                    Mathf.Sqrt(local.x * local.x + local.z * local.z),
                    local.y) * parameterB;
                return 0.5f + 0.5f * Mathf.Cos(angle - parameterA);
            }
            if (type == 5)
            {
                float angle = parameterB * Mathf.Atan2(local.x, local.z);
                float scale = 0.5f + 0.5f * Mathf.Cos(angle + parameterA);
                float absoluteCenter = radiusSqr * Mathf.Abs(parameterC);
                float distanceSqr = local.x * local.x + local.z * local.z;
                if (parameterC < 0.0f)
                {
                    if (distanceSqr < absoluteCenter)
                        scale *= Mathf.Pow(distanceSqr / absoluteCenter, Mathf.PI);
                }
                else
                {
                    distanceSqr *= 1.0f / absoluteCenter;
                    if (distanceSqr < 1.0f)
                        scale = 1.0f - distanceSqr + scale * distanceSqr;
                }
                return Mathf.Pow(scale, Mathf.PI * 0.5f);
            }
            if (type == 6)
            {
                float distance = parameterB * world.magnitude;
                float brightness = 0.9f + 0.1f * Mathf.Sin((distance * 2.0f - parameterA) * Mathf.PI * 2.0f);
                brightness *= 0.9f + 0.1f * Mathf.Cos((distance + parameterA) * Mathf.PI * 2.0f);
                brightness *= 0.9f + 0.1f * Mathf.Sin((distance * 0.5f - parameterA) * Mathf.PI * 2.0f);
                return brightness;
            }

            float horizontal = parameterB * Mathf.Atan2(local.x, local.z);
            float vertical = parameterB * Mathf.Atan2(
                Mathf.Sqrt(local.x * local.x + local.z * local.z),
                local.y);
            float scale1 = 0.5f + 0.5f * Mathf.Cos(horizontal + parameterA);
            float scale2 = 0.5f + 0.5f * Mathf.Cos(vertical - parameterC);
            float discoScale = scale1 + scale2 - scale1 * scale2;
            float radial = 0.5f * (local.x * local.x + local.z * local.z);
            if (radial < 1.0f)
                discoScale *= radial;
            return 1.0f - discoScale;
        }

        private static Vector3 ToLightSpace(Vector3 value, Vector3 forward, Vector3 up)
        {
            Vector3 right = Vector3.Cross(forward, up);
            return new Vector3(
                Vector3.Dot(value, right),
                Vector3.Dot(value, up),
                Vector3.Dot(value, forward));
        }

        private static Vector3 SnapDirection(Vector3 input)
        {
            float divisor = Mathf.Max(Mathf.Max(Mathf.Abs(input.x), Mathf.Abs(input.y)), Mathf.Abs(input.z));
            input /= divisor;
            return new Vector3(
                SnapDirectionRound(input.x),
                SnapDirectionRound(input.y),
                SnapDirectionRound(input.z)).normalized;
        }

        private static float SnapDirectionRound(float value)
        {
            return Mathf.Abs(value) < Mathf.Tan(Mathf.PI / 8.0f) ? 0.0f : Mathf.Sign(value);
        }

        private static EffectTrace Capture(
            DynamicLightEffect effect,
            int seed,
            string schedule,
            IEnumerable<float> deltas)
        {
            UnityEngine.Random.InitState(seed);
            var manager = new DynamicLightManager();
            var light = new DynamicLight { lightEffect = effect };
            var trace = new EffectTrace
            {
                effect = effect.ToString(),
                discriminant = (int)effect,
                seed = seed,
                schedule = schedule,
            };

            var frameDeltas = new List<float>(deltas);
            float elapsed = 0.0f;
            for (int frame = 0; frame < frameDeltas.Count; frame++)
            {
                float delta = frameDeltas[frame];
                manager.deltaTime = delta;
                manager.timeTime = elapsed;
                light.cache.fixedTimestep.timePerStep = light.lightEffectTimestepFrequency;
                light.cache.fixedTimestep.Update(delta);
                manager.UpdateReferenceEffect(light);
                if (frame < 12 || frame % 30 == 0 || frame == frameDeltas.Count - 1)
                {
                    trace.samples.Add(new Sample
                    {
                        frame = frame,
                        time = elapsed,
                        delta = delta,
                        intensity = light.cache.intensity,
                    });
                }
                elapsed += delta;
            }

            return trace;
        }

        // Same source order as DynamicLightManager.UpdateLightEffects. The
        // continuous branches call the unmodified upstream partial methods.
        private void UpdateReferenceEffect(DynamicLight light)
        {
            DynamicLightCache lightCache = light.cache;
            switch (light.lightEffect)
            {
                case DynamicLightEffect.Steady: lightCache.intensity = 1.0f; break;
                case DynamicLightEffect.Candle: ComputeLightEffectCandle(lightCache, light); break;
                case DynamicLightEffect.Fire: ComputeLightEffectFire(lightCache, light); break;
                case DynamicLightEffect.Generator: ComputeLightEffectGenerator(lightCache, light); break;
                case DynamicLightEffect.Lightning: ComputeLightEffectLightning(lightCache, light); break;
                case DynamicLightEffect.Pulsar: ComputeLightEffectPulsar(lightCache, light); break;
                case DynamicLightEffect.Pulse: ComputeLightEffectPulse(lightCache, light); break;
                case DynamicLightEffect.FluorescentStarter: ComputeLightEffectFluorescentStarter(lightCache, light); break;
                case DynamicLightEffect.FluorescentClicker: ComputeLightEffectFluorescentClicker(lightCache, light); break;
                case DynamicLightEffect.FluorescentRandom: ComputeLightEffectFluorescentRandom(lightCache, light); break;
                case DynamicLightEffect.Overcast: ComputeLightEffectOvercast(lightCache, light); break;
                case DynamicLightEffect.Cloudy: ComputeLightEffectCloudy(lightCache, light); break;
            }

            if (lightCache.fixedTimestep.pendingSteps > 0 || !lightCache.initialized)
            {
                lightCache.initialized = true;
                switch (light.lightEffect)
                {
                    case DynamicLightEffect.Flicker:
                        float random = UnityEngine.Random.value;
                        lightCache.intensity = random < 0.5f
                            ? 0.0f
                            : Mathf.Lerp(light.lightEffectPulseModifier, 1.0f, UnityEngine.Random.value);
                        break;
                    case DynamicLightEffect.Random:
                        lightCache.intensity = Mathf.Lerp(
                            light.lightEffectPulseModifier,
                            1.0f,
                            UnityEngine.Random.value);
                        break;
                    case DynamicLightEffect.Strobe:
                        lightCache.strobeActive = !lightCache.strobeActive;
                        lightCache.intensity = lightCache.strobeActive
                            ? 1.0f
                            : light.lightEffectPulseModifier;
                        break;
                }
            }
        }

        private static IEnumerable<float> Repeat(float delta, int count)
        {
            for (int index = 0; index < count; index++)
                yield return delta;
        }

        private static IEnumerable<float> Jittered(float duration)
        {
            float[] pattern = { 1.0f / 53.0f, 1.0f / 71.0f, 1.0f / 44.0f, 1.0f / 97.0f };
            float elapsed = 0.0f;
            int index = 0;
            while (elapsed < duration)
            {
                float delta = Mathf.Min(pattern[index++ % pattern.Length], duration - elapsed);
                elapsed += delta;
                yield return delta;
            }
        }

        private static void WriteJson(string path, object value)
        {
            File.WriteAllText(path, JsonUtility.ToJson(value, true));
        }
    }
}
