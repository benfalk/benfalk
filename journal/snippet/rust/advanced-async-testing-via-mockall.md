---
CREATED_AT: 2025-02-26T13:40:49.025943090+00:00
TOPIC: snippet
lang: rust
title: advanced async testing via mockall
---

## Advanced Async Testing Via Mockall

```rust
#![allow(dead_code, unused_imports, unused_variables)]

use anyhow::{Ok, Result};
use futures::future::BoxFuture;

struct Pool;
pub struct Config;
type AsyncCallback<E> = Box<dyn Fn(&E) -> BoxFuture<'static, ()>>;
struct Callback<E>(AsyncCallback<E>);

impl<E> Callback<E> {
    pub async fn call(&self, value: &E) {
        (self.0)(value).await
    }
}

impl<E> Default for Callback<E> {
    fn default() -> Self {
        let do_nothing = |_: &E| async {};
        Self(Box::new(move |e: &E| Box::pin(do_nothing(e))))
    }
}

impl<E> Callback<E> {
    fn new<F, Fut>(func: F) -> Self
    where
        F: Fn(&E) -> Fut + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        Self(Box::new(move |e: &E| Box::pin(func(e))))
    }
}

#[cfg_attr(test, mockall::automock)]
trait Db {
    fn pool(&self) -> &Pool;

    async fn exec<Q>(&self, query: &Q) -> Result<Q::Value>
    where
        Q: Query + 'static,
    {
        query.exec(self.pool()).await
    }
}

impl Db for &Pool {
    fn pool(&self) -> &Pool {
        self
    }
}

trait Query {
    type Value;
    async fn exec(&self, pool: &Pool) -> Result<Self::Value>;
}

#[cfg_attr(test, mockall::automock)]
trait Repo {
    async fn report(&self, bar: u32);
}

pub trait Env {
    fn db(&self) -> &impl Db;
    fn repo(&self) -> &impl Repo;
    fn config(&self) -> &Config;
}

struct ImporterEnv<D: Db, R: Repo> {
    db: D,
    repo: R,
}

impl<D: Db, R: Repo> Env for ImporterEnv<D, R> {
    fn db(&self) -> &impl Db {
        &self.db
    }

    fn repo(&self) -> &impl Repo {
        &self.repo
    }

    fn config(&self) -> &Config {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GetData {
    pub value: u8,
}

impl Query for GetData {
    type Value = u32;
    async fn exec(&self, pool: &Pool) -> Result<Self::Value> {
        todo!()
    }
}

async fn download_data<E>(env: &E) -> Result<u32>
where
    E: Env,
{
    let query = GetData { value: 42 };
    let val = env.db().exec(&query).await?;
    Ok(val)
}

#[cfg(test)]
mod test {
    type TestEnv = ImporterEnv<MockDb, MockRepo>;
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn test_env() -> TestEnv {
        TestEnv {
            repo: MockRepo::new(),
            db: MockDb::new(),
        }
    }

    #[rstest]
    #[tokio::test]
    async fn it_works(mut test_env: TestEnv) {
        test_env
            .db
            .expect_exec()
            .with(predicate::eq(GetData { value: 42 }))
            .times(1)
            .returning(|_| Ok(69));
        let val = download_data(&test_env).await.unwrap();
        assert_eq!(val, 69);
    }
}
```
