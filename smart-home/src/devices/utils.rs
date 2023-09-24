use rand::{distributions::uniform::SampleUniform, thread_rng, Rng};

pub trait RandomValue {
    type Value: SampleUniform + PartialOrd;
    const LOW: Self::Value;
    const MAX: Self::Value;

    fn choose() -> Self::Value {
        thread_rng().gen_range(Self::LOW..Self::MAX)
    }
}
