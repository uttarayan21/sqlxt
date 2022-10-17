/// Bind multiple or single values to a sql query
///
/// the bind values should be able to be bounded by the bind() function with the following
/// signature
/// ```rust
/// pub fn bind<T>(self, value: T) -> Query<'q, Db, <Db as HasArguments<'q>>::Arguments>
/// where
///     T: 'q + Send + Encode<'q, Db> + Type<Db>,
/// ```
/// Since I'm only planning on sqlite3 support all ints are i64 and all floats are f64
/// But other types can be added in the future
pub trait Bindable<'q, Db: sqlx::Database>:
    'q + Send + sqlx::Encode<'q, Db> + sqlx::Type<Db>
{
}

/// Make the bind() function accessible by traits
pub trait BindingTo<'q, Db: sqlx::Database, T: 'q + Send + sqlx::Encode<'q, Db> + sqlx::Type<Db>> {
    fn bind(self, value: T) -> Self;
}

impl<'q, Db: sqlx::Database, T> BindingTo<'q, Db, T>
    for sqlx::query::Query<'q, Db, <Db as sqlx::database::HasArguments<'q>>::Arguments>
where
    T: 'q + Send + sqlx::Encode<'q, Db> + sqlx::Type<Db>,
{
    fn bind(self, value: T) -> Self {
        sqlx::query::Query::bind(self, value)
    }
}

impl<'q, Db: sqlx::Database, T, O> BindingTo<'q, Db, T>
    for sqlx::query::QueryAs<'q, Db, O, <Db as sqlx::database::HasArguments<'q>>::Arguments>
where
    T: 'q + Send + sqlx::Encode<'q, Db> + sqlx::Type<Db>,
{
    fn bind(self, value: T) -> Self {
        sqlx::query::QueryAs::bind(self, value)
    }
}

impl<'q, Db: sqlx::Database, O, T> BindingTo<'q, Db, T>
    for sqlx::query::QueryScalar<'q, Db, O, <Db as sqlx::database::HasArguments<'q>>::Arguments>
where
    T: 'q + Send + sqlx::Encode<'q, Db> + sqlx::Type<Db>,
{
    fn bind(self, value: T) -> Self {
        sqlx::query::QueryScalar::bind(self, value)
    }
}

/// Bind multiple values to a sql query
///
/// the bind values should be able to be bounded by the bind() function with the following signature
pub trait Bind<BindingTo> {
    type Error: std::error::Error;
    fn query(&self) -> Result<(), Self::Error>;
    fn map_values(&self) -> Result<(), Self::Error>;
}
