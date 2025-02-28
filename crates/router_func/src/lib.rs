#![allow(dead_code, unused_imports, unused_variables)]

mod structures {
    pub struct Config(pub String);
    pub struct User(pub u32);
    pub struct Context {
        pub(super) user: User,
        pub(super) config: Config,
    }
}

mod context {
    use super::structures::*;

    pub trait Ctx {
        fn user(&self) -> &User;
        fn config(&self) -> &Config;
    }

    impl Ctx for Context {
        fn user(&self) -> &User {
            &self.user
        }

        fn config(&self) -> &Config {
            &self.config
        }
    }

    pub trait FromContext {
        fn from_context(ctx: &impl Ctx) -> &Self;
    }

    impl FromContext for User {
        fn from_context(ctx: &impl Ctx) -> &Self {
            ctx.user()
        }
    }

    impl FromContext for Config {
        fn from_context(ctx: &impl Ctx) -> &Self {
            ctx.config()
        }
    }
}

mod handler {
    use super::context::{Ctx, FromContext};

    pub trait Handle<T> {
        type Value;
        fn call(self, ctx: &impl Ctx) -> Self::Value;
    }

    impl<F, T, R> Handle<T> for F
    where
        F: Fn(&T) -> R,
        T: FromContext,
    {
        type Value = R;
        fn call(self, ctx: &impl Ctx) -> Self::Value {
            (self)(T::from_context(ctx))
        }
    }

    impl<F, T1, T2, R> Handle<(T1, T2)> for F
    where
        F: Fn(&T1, &T2) -> R,
        T1: FromContext,
        T2: FromContext,
    {
        type Value = R;
        fn call(self, ctx: &impl Ctx) -> Self::Value {
            (self)(T1::from_context(ctx), T2::from_context(ctx))
        }
    }

    pub trait CtxHandle: Ctx + Sized {
        fn call<T, H>(&self, handle: H) -> H::Value
        where
            H: Handle<T>,
        {
            handle.call(self)
        }
    }

    impl<C: Ctx + Sized> CtxHandle for C {}
}

#[cfg(test)]
mod test {
    use super::context::*;
    use super::handler::*;
    use super::structures::*;

    fn next_user(user: &User) -> u32 {
        user.0 + 1
    }

    fn hello(config: &Config) -> String {
        format!("hello {}", config.0.as_str())
    }

    #[test]
    fn it_works() {
        let ctx = Context {
            user: User(42),
            config: Config("world".to_owned()),
        };

        assert_eq!(next_user.call(&ctx), 43);
        assert_eq!(ctx.call(next_user), 43);

        assert_eq!(ctx.call(hello), "hello world");
        assert_eq!(hello.call(&ctx), "hello world");
    }
}
