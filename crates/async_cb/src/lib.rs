pub use structure::Callback;

mod structure {
    use ::futures::future::BoxFuture;

    type Cb<E> = Box<dyn Fn(E) -> BoxFuture<'static, ()>>;

    pub struct Callback<E> {
        pub(super) func: Cb<E>,
    }
}

mod impls {
    use super::Callback;

    impl<E> Default for Callback<E> {
        fn default() -> Self {
            let do_nothing = |_| async {};
            Self {
                func: Box::new(move |e| Box::pin(do_nothing(e))),
            }
        }
    }

    impl<E> Callback<E> {
        pub fn new<F, Ret>(func: F) -> Self
        where
            Ret: Future<Output = ()> + Send + 'static,
            F: Fn(E) -> Ret + 'static,
        {
            Self {
                func: Box::new(move |e| Box::pin(func(e))),
            }
        }

        pub fn call(&self, e: E) -> impl Future<Output = ()> {
            (self.func)(e)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Callback;
    use mockall::predicate::*;
    use mockall::*;

    #[cfg_attr(test, mockall::automock)]
    trait Foo {
        async fn say(&self, string: &str);
    }

    #[tokio::test]
    async fn example() {
        let mut foo = MockFoo::new();

        foo.expect_say()
            .with(predicate::eq("hello"))
            .returning(|_| ());

        foo.expect_say()
            .with(predicate::eq("goodbye"))
            .returning(|_| ());

        let cb = Callback::new(|foo: MockFoo| async move {
            foo.say("hello").await;
            foo.say("goodbye").await;
        });

        cb.call(foo).await;
    }
}
