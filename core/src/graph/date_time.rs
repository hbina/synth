use super::prelude::*;

use std::{ops::Range as StdRange, sync::Arc};

derive_generator! {
    yield Token,
    return Result<Value, Error>,
    pub struct DateTimeNode(Valuize<Tokenizer<RandomDateTime>, ChronoValueAndFormat>);
}

impl From<RandomDateTime> for DateTimeNode {
    fn from(value: RandomDateTime) -> Self {
        Self(
            value
                .into_token()
                .map_complete(value_from_ok::<ChronoValueAndFormat>),
        )
    }
}

pub struct RandomDateTime {
    inner: OnceInfallible<Random<ChronoValue, Uniform<ChronoValue>>>,
    format: Arc<str>,
}

impl RandomDateTime {
    pub fn new(range: StdRange<ChronoValue>, format: &str) -> Self {
        Self {
            inner: Random::new_with(Uniform::new_inclusive(range.start, range.end))
                .infallible()
                .try_once(),
            format: Arc::from(format.to_owned()),
        }
    }
}

pub struct UniformChronoValue(ChronoValue, UniformDuration);

impl SampleUniform for ChronoValue {
    type Sampler = UniformChronoValue;
}

impl UniformSampler for UniformChronoValue {
    type X = ChronoValue;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        // safe because it has been asserted by rand API contract that
        // high >= low, which implies same variant of ChronoValue
        let delta = low.borrow().delta_to(high.borrow()).unwrap();
        let inner = UniformDuration::new(StdDuration::default(), delta);
        UniformChronoValue(low.borrow().clone(), inner)
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let delta = low.borrow().delta_to(high.borrow()).unwrap();
        let inner = UniformDuration::new_inclusive(StdDuration::default(), delta);
        UniformChronoValue(low.borrow().clone(), inner)
    }

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        let delta = self.1.sample(rng);
        self.0.clone() + delta
    }
}

impl Generator for RandomDateTime {
    type Yield = String;

    type Return = Result<ChronoValueAndFormat, Error>;

    fn next<R: Rng>(&mut self, rng: &mut R) -> GeneratorState<Self::Yield, Self::Return> {
        match self.inner.next(rng) {
            GeneratorState::Yielded(y) => {
                match ChronoValueFormatter::new(&self.format).format(&y) {
                    Ok(formatted) => GeneratorState::Yielded(formatted),
                    Err(err) => GeneratorState::Complete(Err(err)),
                }
            }
            GeneratorState::Complete(r) => {
                GeneratorState::Complete(r.map(|value| ChronoValueAndFormat {
                    value,
                    format: Arc::clone(&self.format),
                }))
            }
        }
    }
}
