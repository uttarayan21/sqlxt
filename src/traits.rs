/// Make the bind() function accessible by traits
pub trait BindingTo<'q, Db: sqlx::Database> {
    fn bind<T: 'q + Send + sqlx::Encode<'q, Db> + sqlx::Type<Db>>(self, value: T) -> Self;
}

impl<'q, Db: sqlx::Database> BindingTo<'q, Db>
    for sqlx::query::Query<'q, Db, <Db as sqlx::database::HasArguments<'q>>::Arguments>
{
    fn bind<T: 'q + Send + sqlx::Encode<'q, Db> + sqlx::Type<Db>>(self, value: T) -> Self {
        sqlx::query::Query::bind(self, value)
    }
}

impl<'q, Db: sqlx::Database, O> BindingTo<'q, Db>
    for sqlx::query::QueryAs<'q, Db, O, <Db as sqlx::database::HasArguments<'q>>::Arguments>
{
    fn bind<T: 'q + Send + sqlx::Encode<'q, Db> + sqlx::Type<Db>>(self, value: T) -> Self {
        sqlx::query::QueryAs::bind(self, value)
    }
}

impl<'q, Db: sqlx::Database, O> BindingTo<'q, Db>
    for sqlx::query::QueryScalar<'q, Db, O, <Db as sqlx::database::HasArguments<'q>>::Arguments>
{
    fn bind<T: 'q + Send + sqlx::Encode<'q, Db> + sqlx::Type<Db>>(self, value: T) -> Self {
        sqlx::query::QueryScalar::bind(self, value)
    }
}

/// Bind multiple values to a sql query
///
/// the bind values should be able to be bounded by the bind() function with the following signature
pub trait MapValues<Db: sqlx::Database> {
    type Output: AsRef<str>;
    type Error: std::error::Error + From<sqlx::Error>;
    fn query(&self) -> Result<Self::Output, Self::Error>;
    fn map_values<'q, Q: BindingTo<'q, Db>>(&self, query: Q) -> Result<(), Self::Error>;
}

pub trait QueryGen<'q, Db: sqlx::Database, Q: BindingTo<'q, Db>> {
    type Error: std::error::Error + From<sqlx::Error>;
    fn generate(&self) -> Result<Q, Self::Error>;
}
