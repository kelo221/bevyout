#if BEVYOUT_REFERENCE_HARNESS
namespace AlpacaIT.DynamicLighting
{
    // Minimal authoring/runtime surface used only by the batchmode reference
    // project. The effect implementations themselves are copied byte-for-byte
    // from the frozen upstream checkout.
    public sealed class DynamicLight
    {
        public float lightIntensity = 2.0f;
        public DynamicLightEffect lightEffect = DynamicLightEffect.Steady;
        public float lightEffectPulseSpeed = 1.0f;
        public float lightEffectPulseModifier = 0.25f;
        public float lightEffectPulseOffset = 0.0f;
        public float lightEffectTimestepFrequency = 1.0f / 30.0f;
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
