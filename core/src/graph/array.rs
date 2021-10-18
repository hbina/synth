use super::prelude::*;

use synth_gen::value::Seq;

use std::cell::RefCell;
use std::rc::Rc;

struct RandomArray(Seq<Repeat<Rc<RefCell<Graph>>>>);

impl RandomArray {
    pub fn with_length(len: u64, content: Rc<RefCell<Graph>>) -> Self {
        let len = len as usize;
        Self(content.repeat(len).into_seq(Some(len)))
    }
}

impl Generator for RandomArray {
    type Yield = Token;

    type Return = Result<Value, Error>;

    fn next<R: Rng>(&mut self, rng: &mut R) -> GeneratorState<Self::Yield, Self::Return> {
        self.0.next(rng).map_complete(|seqr| {
            seqr.into_iter()
                .collect::<Result<Vec<_>, Error>>()
                .map(|seq| seq.into())
        })
    }
}

type ArrayNodeInner = AndThenTry<SizeGenerator, Box<dyn Fn(u64) -> RandomArray>, RandomArray>;

derive_generator! {
    yield Token,
    return Result<Value, Error>,
    pub struct ArrayNode(ArrayNodeInner);
}

impl ArrayNode {
    pub fn new_with(len: SizeGenerator, content: Graph) -> Self {
        let content = Rc::new(RefCell::new(content));
        let closure: Box<dyn Fn(u64) -> RandomArray> =
            Box::new(move |length| RandomArray::with_length(length, content.clone()));
        let inner = len.and_then_try(closure);
        Self(inner)
    }
}
