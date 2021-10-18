use std::marker::PhantomData;

use rand::Rng;

use crate::{GeneratorState, Never};

use super::Generator;
use std::collections::VecDeque;

/// we use this trait to bound `Result`s because we don't have type equality
/// in `where` clauses yet.
pub trait GeneratorResult {
    type Ok;
    type Err;
    // this should be a no-op
    fn into_result(self) -> Result<Self::Ok, Self::Err>;
    fn from_ok(ok: Self::Ok) -> Self;
    fn from_err(err: Self::Err) -> Self;
}

impl<O, E> GeneratorResult for Result<O, E> {
    type Ok = O;
    type Err = E;

    // inlined to make sure it's optimized out
    #[inline(always)]
    fn into_result(self) -> Result<O, E> {
        self
    }
    fn from_ok(ok: O) -> Self {
        Ok(ok)
    }
    fn from_err(err: E) -> Self {
        Err(err)
    }
}

/// A convenience trait for [`Generator`](crate::Generator)s that
/// return [`Result`](std::result::Result)s.
pub trait TryGenerator {
    type Yield;

    type Ok;

    type Error;

    type Return: GeneratorResult<Ok = Self::Ok, Err = Self::Error>;

    fn try_next_yielded<R: Rng>(&mut self, rng: &mut R) -> Result<Self::Yield, Self::Error> {
        loop {
            match self.try_next(rng) {
                GeneratorState::Yielded(y) => return Ok(y),
                GeneratorState::Complete(t) => match t.into_result() {
                    Err(e) => return Err(e),
                    Ok(_) => continue,
                },
            }
        }
    }

    fn try_next<R: Rng>(&mut self, rng: &mut R) -> GeneratorState<Self::Yield, Self::Return>;

    fn try_complete<R: Rng>(&mut self, rng: &mut R) -> Self::Return {
        loop {
            match self.try_next(rng) {
                GeneratorState::Yielded(_) => continue,
                GeneratorState::Complete(r) => return r,
            }
        }
    }
}

pub trait FallibleGenerator {
    type Ok;

    type Error;

    type Yield: GeneratorResult<Ok = Self::Ok, Err = Self::Error>;

    type Return;

    fn next<R: Rng>(&mut self, rng: &mut R) -> GeneratorState<Self::Yield, Self::Return>;
}

impl<G> FallibleGenerator for G
where
    G: Generator,
    G::Yield: GeneratorResult,
{
    type Ok = <G::Yield as GeneratorResult>::Ok;

    type Error = <G::Yield as GeneratorResult>::Err;

    type Yield = G::Yield;

    type Return = G::Return;

    fn next<R: Rng>(&mut self, rng: &mut R) -> GeneratorState<G::Yield, G::Return> {
        <Self as Generator>::next(self, rng)
    }
}

pub trait FallibleGeneratorExt: Sized
where
    Self: FallibleGenerator,
{
    fn unwrap(self) -> Unwrap<Self> {
        Unwrap { inner: self }
    }
}

impl<G> FallibleGeneratorExt for G where G: FallibleGenerator {}

pub struct Unwrap<G> {
    inner: G,
}

impl<G> Generator for Unwrap<G>
where
    G: Generator,
    G::Yield: GeneratorResult,
{
    type Yield = <G::Yield as GeneratorResult>::Ok;

    type Return = Result<G::Return, <G::Yield as GeneratorResult>::Err>;

    fn next<R: Rng>(&mut self, rng: &mut R) -> GeneratorState<Self::Yield, Self::Return> {
        match self.inner.next(rng) {
            GeneratorState::Yielded(y) => match y.into_result() {
                Ok(y_ok) => GeneratorState::Yielded(y_ok),
                Err(err) => GeneratorState::Complete(Err(err)),
            },
            GeneratorState::Complete(c) => GeneratorState::Complete(Ok(c)),
        }
    }
}

impl<G> TryGenerator for G
where
    G: Generator,
    G::Return: GeneratorResult,
{
    type Yield = G::Yield;

    type Ok = <G::Return as GeneratorResult>::Ok;

    type Error = <G::Return as GeneratorResult>::Err;

    type Return = G::Return;

    fn try_next<Rand: Rng>(&mut self, rng: &mut Rand) -> GeneratorState<G::Yield, G::Return> {
        self.next(rng)
    }
}

/// An extension trait for [`TryGenerator`](crate::TryGenerator)s.
pub trait TryGeneratorExt: Sized
where
    Self: TryGenerator,
{
    fn try_once(self) -> TryOnce<Self> {
        TryOnce {
            inner: self,
            output: None,
        }
    }

    fn try_yield(self) -> TryYield<Self> {
        TryYield {
            inner: self,
            yielded: None,
        }
    }

    fn map_ok<F, O>(self, f: F) -> MapOk<Self, F, O>
    where
        F: Fn(Self::Ok) -> O,
    {
        MapOk {
            inner: self,
            closure: f,
            _output: PhantomData,
        }
    }

    fn or_else_try<F, O>(self, f: F) -> OrElseTry<Self, F, O>
    where
        F: Fn(Self::Error) -> O,
        O: TryGenerator<Ok = Self::Ok>,
    {
        OrElseTry {
            inner: self,
            closure: f,
            output: None,
        }
    }

    fn and_then_try<F, O>(self, f: F) -> AndThenTry<Self, F, O>
    where
        F: Fn(Self::Ok) -> O,
        O: TryGenerator<Error = Self::Error>,
    {
        AndThenTry {
            inner: self,
            closure: f,
            output: None,
        }
    }

    fn try_filter_map<F, O>(self, f: F) -> TryFilterMap<Self, F, O>
    where
        F: FnMut(Self::Ok) -> Result<Option<O>, Self::Error>,
    {
        TryFilterMap {
            inner: self,
            yielded: VecDeque::new(),
            output: None,
            closure: f,
        }
    }

    fn try_aggregate(self) -> TryAggregate<Self> {
        TryAggregate {
            inner: self,
            output: None,
        }
    }
}

impl<TG> TryGeneratorExt for TG where TG: TryGenerator {}

/// This struct is created by the
/// [`try_once`](crate::TryGeneratorExt::try_once) method on
/// [`TryGenerator`](crate::TryGenerator).
pub struct TryOnce<G: TryGenerator> {
    inner: G,
    output: Option<G::Yield>,
}

impl<G> Generator for TryOnce<G>
where
    G: Generator,
    G::Yield: Clone,
    G::Return: GeneratorResult<Ok = Never>,
{
    type Yield = G::Yield;

    type Return = Result<G::Yield, <G::Return as GeneratorResult>::Err>;

    fn next<R: Rng>(&mut self, rng: &mut R) -> GeneratorState<Self::Yield, Self::Return> {
        if let Some(y) = std::mem::replace(&mut self.output, None) {
            GeneratorState::Complete(Ok(y))
        } else {
            match self.inner.next(rng) {
                GeneratorState::Yielded(y) => {
                    self.output = Some(y.clone());
                    GeneratorState::Yielded(y)
                }
                GeneratorState::Complete(c) => match c.into_result() {
                    Err(err) => GeneratorState::Complete(Err(err)),
                    Ok(_) => unreachable!(),
                },
            }
        }
    }
}

pub struct TryYield<G: TryGenerator> {
    inner: G,
    yielded: Option<G::Ok>,
}

impl<G> Generator for TryYield<G>
where
    G: TryGenerator,
    G::Ok: Clone,
{
    type Yield = G::Ok;
    type Return = G::Return;

    fn next<R: Rng>(&mut self, rng: &mut R) -> GeneratorState<Self::Yield, Self::Return> {
        if let Some(yielded) = std::mem::replace(&mut self.yielded, None) {
            return GeneratorState::Complete(Self::Return::from_ok(yielded));
        }

        match self.inner.try_complete(rng).into_result() {
            Ok(ret) => {
                self.yielded = Some(ret.clone());
                GeneratorState::Yielded(ret)
            }
            Err(err) => GeneratorState::Complete(Self::Return::from_err(err)),
        }
    }
}

/// This struct is created by the
/// [`and_then_try`](crate::TryGeneratorExt::and_then_try) method on
/// [`TryGenerator`](crate::TryGenerator).
pub struct AndThenTry<G, F, O> {
    inner: G,
    closure: F,
    output: Option<O>,
}

impl<TG, F, O> Generator for AndThenTry<TG, F, O>
where
    TG: TryGenerator,
    F: Fn(TG::Ok) -> O,
    O: TryGenerator<Yield = TG::Yield, Error = TG::Error>,
{
    type Yield = TG::Yield;

    type Return = O::Return;

    fn next<R: Rng>(&mut self, rng: &mut R) -> GeneratorState<Self::Yield, Self::Return> {
        if let Some(output) = self.output.as_mut() {
            let next = output.try_next(rng);
            if next.is_complete() {
                self.output = None;
            }
            next
        } else {
            match self.inner.try_next(rng) {
                GeneratorState::Yielded(y) => GeneratorState::Yielded(y),
                GeneratorState::Complete(c) => match c.into_result() {
                    Err(err) => GeneratorState::Complete(O::Return::from_err(err)),
                    Ok(r) => {
                        self.output = Some((self.closure)(r));
                        self.next(rng)
                    }
                },
            }
        }
    }
}

/// This struct is created by the
/// [`and_then_try`](crate::TryGeneratorExt::and_then_try) method on
/// [`TryGenerator`](crate::TryGenerator).
pub struct OrElseTry<G, F, O> {
    inner: G,
    closure: F,
    output: Option<O>,
}

impl<TG, F, O> Generator for OrElseTry<TG, F, O>
where
    TG: TryGenerator,
    F: Fn(TG::Error) -> O,
    O: TryGenerator<Yield = TG::Yield, Ok = TG::Ok>,
{
    type Yield = TG::Yield;

    type Return = O::Return;

    fn next<R: Rng>(&mut self, rng: &mut R) -> GeneratorState<Self::Yield, Self::Return> {
        if let Some(output) = self.output.as_mut() {
            let next = output.try_next(rng);
            if next.is_complete() {
                self.output = None;
            }
            next
        } else {
            match self.inner.try_next(rng) {
                GeneratorState::Yielded(y) => GeneratorState::Yielded(y),
                GeneratorState::Complete(c) => match c.into_result() {
                    Err(err) => {
                        self.output = Some((self.closure)(err));
                        self.next(rng)
                    }
                    Ok(r) => GeneratorState::Complete(O::Return::from_ok(r)),
                },
            }
        }
    }
}

pub struct TryFilterMap<TG, F, O>
where
    TG: TryGenerator,
    F: FnMut(TG::Ok) -> Result<Option<O>, TG::Error>,
{
    inner: TG,
    yielded: VecDeque<TG::Yield>,
    output: Option<O>,
    closure: F,
}

impl<TG, F, O> Generator for TryFilterMap<TG, F, O>
where
    TG: TryGenerator,
    F: FnMut(TG::Ok) -> Result<Option<O>, TG::Error>,
{
    type Yield = TG::Yield;

    type Return = Result<O, TG::Error>;

    fn next<RR: Rng>(&mut self, rng: &mut RR) -> GeneratorState<Self::Yield, Self::Return> {
        match self.yielded.pop_front() {
            Some(fst) => GeneratorState::Yielded(fst),
            None => match std::mem::replace(&mut self.output, None) {
                Some(output) => GeneratorState::Complete(Ok(output)),
                None => {
                    loop {
                        match self.inner.try_next(rng) {
                            GeneratorState::Yielded(yielded) => self.yielded.push_back(yielded),
                            GeneratorState::Complete(complete) => match complete.into_result() {
                                Ok(output) => match (self.closure)(output) {
                                    Ok(Some(value)) => {
                                        self.output = Some(value);
                                        break;
                                    }
                                    Ok(None) => {
                                        self.yielded.clear();
                                    }
                                    Err(err) => return GeneratorState::Complete(Err(err)),
                                },
                                Err(err) => return GeneratorState::Complete(Err(err)),
                            },
                        }
                    }
                    self.next(rng)
                }
            },
        }
    }
}

/// This struct is created by the
/// [`try_aggregate`](crate::TryGeneratorExt::try_aggregate) method on
/// [`TryGenerator`](crate::TryGenerator).
pub struct TryAggregate<TG>
where
    TG: TryGenerator,
{
    inner: TG,
    output: Option<TG::Return>,
}

impl<TG> Generator for TryAggregate<TG>
where
    TG: TryGenerator,
{
    type Yield = Vec<TG::Yield>;
    type Return = TG::Return;

    fn next<R: Rng>(&mut self, rng: &mut R) -> GeneratorState<Self::Yield, Self::Return> {
        if let Some(r) = std::mem::replace(&mut self.output, None) {
            GeneratorState::Complete(r)
        } else {
            let mut out = Vec::new();
            loop {
                match self.inner.try_next(rng) {
                    GeneratorState::Yielded(y) => out.push(y),
                    GeneratorState::Complete(r) => {
                        self.output = Some(r);
                        break;
                    }
                }
            }
            match std::mem::replace(&mut self.output, None)
                .unwrap()
                .into_result()
            {
                Ok(v) => {
                    self.output = Some(TG::Return::from_ok(v));
                    GeneratorState::Yielded(out)
                }
                Err(e) => GeneratorState::Complete(TG::Return::from_err(e)),
            }
        }
    }
}

pub struct MapOk<TG, F, O> {
    inner: TG,
    closure: F,
    _output: PhantomData<O>,
}

impl<TG, F, O> Generator for MapOk<TG, F, O>
where
    TG: TryGenerator,
    F: Fn(TG::Ok) -> O,
{
    type Yield = TG::Yield;

    type Return = Result<O, TG::Error>;

    fn next<R: Rng>(&mut self, rng: &mut R) -> GeneratorState<Self::Yield, Self::Return> {
        self.inner
            .try_next(rng)
            .map_complete(|ret| ret.into_result().map(|ok| (self.closure)(ok)))
    }
}
