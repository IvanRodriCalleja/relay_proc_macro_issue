use std::marker::PhantomData;

use juniper::FieldResult;
use uuid::Uuid;

pub use relay_derive::*;

pub trait RelayNode where Self: Sized {
    /// get is a method defines by the user to refetch an object of a particular type.
    /// The context can be used to share a database connection or other required context to facilitate the refetch.
    fn get(id: String) -> FieldResult<Self>;
}

#[derive(Clone, PartialEq, Eq)]
pub struct RelayNodeID<T: RelayNode + ?Sized>(Uuid, PhantomData<T>);
