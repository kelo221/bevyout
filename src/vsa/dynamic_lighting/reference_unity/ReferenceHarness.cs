#if BEVYOUT_REFERENCE_HARNESS
namespace AlpacaIT.DynamicLighting
{
    public enum DynamicLightShadowMode
    {
        RaytracedShadows = 0,
        RealtimeShadows = 1,
    }

    public enum DynamicLightIlluminationMode
    {
        DirectIllumination = 0,
        SingleBounce = 1,
    }

    public enum DynamicLightVolumetricType
    {
        None = 0,
        Sphere = 1,
        Box = 2,
        ConeZ = 3,
        ConeY = 4,
    }

    // Minimal authoring/runtime surface used only by the batchmode reference
    // project. The effect implementations themselves are copied byte-for-byte
    // from the frozen upstream checkout.
    public sealed class DynamicLight
    {
        public float lightIntensity = 2.0f;
        public float lightRadius = 4.0f;
        public float lightFalloff = 0.0f;
        public float lightCutoff = 26.0f;
        public float lightOuterCutoff = 30.0f;
        public float lightWaveSpeed = 1.0f;
        public float lightWaveFrequency = 1.0f;
        public float lightWaveOffset = 0.0f;
        public float lightRotorCenter = 0.1f;
        public float lightDiscoVerticalSpeed = 1.0f;
        public float lightBounceModifier = 1.0f;
        public float lightBounceIntensity = 1.0f;
        public DynamicLightShadowMode lightShadows = DynamicLightShadowMode.RaytracedShadows;
        public DynamicLightIlluminationMode lightIllumination = DynamicLightIlluminationMode.DirectIllumination;
        public DynamicLightEffect lightEffect = DynamicLightEffect.Steady;
        public float lightEffectPulseSpeed = 1.0f;
        public float lightEffectPulseModifier = 0.25f;
        public float lightEffectPulseOffset = 0.0f;
        public float lightEffectTimestepFrequency = 1.0f / 30.0f;
        public DynamicLightVolumetricType lightVolumetricType = DynamicLightVolumetricType.None;
        public float lightVolumetricRadius = 4.0f;
        public float lightVolumetricThickness = 1.0f;
        public float lightVolumetricIntensity = 0.75f;
        public float lightVolumetricVisibility = 2.0f;
        internal DynamicLightCache cache = new DynamicLightCache();
    }

    internal sealed class DynamicLightCache
    {
        public bool initialized;
        public FixedTimestep fixedTimestep = new FixedTimestep(1.0f / 30.0f);
        public float intensity;
        public bool strobeActive;
        public int fluorescentRandomState;
        public float fluorescentRandomTime;
    }

    internal sealed class FixedTimestep
    {
        public float timePerStep;
        private float timeAccumulator;
        public int pendingSteps;

        public FixedTimestep(float timePerStep) => this.timePerStep = timePerStep;

        public void Update(float deltaTime)
        {
            pendingSteps = 0;
            timeAccumulator += deltaTime;
            if (timeAccumulator >= timePerStep)
            {
                pendingSteps = (int)(timeAccumulator / timePerStep);
                timeAccumulator -= pendingSteps * timePerStep;
            }
        }
    }

    public partial class DynamicLightManager
    {
        private float deltaTime;
        private float timeTime;
    }
}
#endif
