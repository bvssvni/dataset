extern crate dataset;

use std::any::{ Any, TypeId };
use dataset::*;

pub struct Family {
    pub persons: Vec<Person>,
}

pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

unsafe impl DataSet for Family {
    fn get_table<T: Any>(&self) -> Option<&[T]> {
        use std::mem::transmute;

        let id = TypeId::of::<T>();
        match id {
            x if x == TypeId::of::<Person>() => {
                unsafe { Some(transmute(&self.persons[0..])) }
            }
            _ => None
        }
    }

    fn raw_table<T: Any>(&mut self) -> Option<*mut Vec<T>> {
        use std::mem::transmute;

        let id = TypeId::of::<T>();
        match id {
            x if x == TypeId::of::<Person>() => {
                unsafe { Some(transmute(&mut self.persons)) }
            }
            _ => None
        }
    }
}

fn main() {
    let simpson = "Simpson";
    let mut family = Family {
        persons: vec![]
    };

    let homer = Person { first_name: "Homer".into(), last_name: simpson.into() };
    let marge = Person { first_name: "Marge".into(), last_name: simpson.into() };
    let bart = Person { first_name: "Bart".into(), last_name: simpson.into() };
    let lisa = Person { first_name: "Lisa".into(), last_name: simpson.into() };
    let maggie = Person { first_name: "Maggie".into(), last_name: simpson.into() };

    {
        let persons = unsafe { &mut *family.raw_table().unwrap() };
        persons.push(homer);
        persons.push(marge);
        persons.push(bart);
        persons.push(lisa);
        persons.push(maggie);
    }

    for p in family.get_table::<Person>().unwrap() {
        println!("{} {}", p.first_name, p.last_name);
    }
}
