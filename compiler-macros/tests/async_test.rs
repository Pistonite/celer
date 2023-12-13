use celercmacros::{async_trait, async_recursion};

#[async_trait]
pub trait X {
    async fn foo() {}
}

#[async_trait(?Send)]
pub trait Y {
    async fn foo() {}
}

#[async_trait(auto)]
pub trait Z {
    async fn foo() {}
}

#[async_recursion]
pub async fn x() {}

#[async_recursion(?Send)]
pub async fn y() {}

#[async_recursion(auto)]
pub async fn z() {}
