extern crate rocket;
extern crate validator;

use std::error::Error;
use std::boxed::Box;
use std::ops::{Deref, DerefMut};

use rocket::outcome::Outcome;
use rocket::request::Request;
use rocket::data::{self, Data, FromData};
use rocket::http::Status;
use validator::Validate;

#[derive(Debug, Clone)]
pub struct Validation<T>(pub T);

impl<T> Validation<T> {
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl <T> FromData for Validation<T>
    where T: FromData + Validate,
          <T as rocket::data::FromData>::Error: Error + 'static
{
    type Error = Box<Error>;

    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        use Outcome::*;

        match T::from_data(request, data) {
            Success(val) => {
                if let Err(err) = val.validate() {
                    Failure((Status::BadRequest, Box::new(err)))
                } else {
                    Success(Validation(val))
                }
            },
            Failure((status, err)) => Failure((status, Box::new(err))),
            Forward(data) => Forward(data),
        }
    }
}

impl<T> Deref for Validation<T> {
    type Target = T;

    #[inline(always)]
    fn deref<'a>(&'a self) -> &'a T {
        &self.0
    }
}

impl<T> DerefMut for Validation<T> {
    #[inline(always)]
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        &mut self.0
    }
}
