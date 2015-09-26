#[macro_use]
extern crate dataset;

use std::any::{ Any, TypeId };

use dataset::*;

pub struct Family {
    pub persons: Vec<Person>,
    pub parents: Vec<Parent>,
}

pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

pub struct Parent {
    pub parent_id: usize,
    pub child_id: usize,
}

fn foo<T: HasTable<Person> + HasTable<Parent>>(dataset: &mut T) {
    let persons: &mut Vec<Person> = unsafe { &mut *dataset.raw_table() };
    let parents: &mut Vec<Parent> = unsafe { &mut *dataset.raw_table() };
    println!("{}", persons.len());
    println!("{}", parents.len());
}

fn bar<T: DataSet>(dataset: &T) {
    /*
    assert_eq!(unsafe {
            *dataset.read_usize("Parent", "child_id").unwrap().get(0).unwrap()
        }, 2);
    */
    for p in dataset.read::<usize>("Parent", "parent_id").unwrap() {
        println!("{}", unsafe { *p });
    }
    for p in dataset.read::<String>("Person", "last_name").unwrap() {
        println!("{}", unsafe { &*p });
    }
}

dataset_impl! {
    Family {
        persons: Person { first_name: String, last_name: String }
        parents: Parent { parent_id: usize, child_id: usize }
    }
}

has_table_impls! {
    Family {
        persons: Person,
        parents: Parent
    }
}

fn main() {
    let simpson = "Simpson";
    let mut family = Family {
        persons: vec![],
        parents: vec![],
    };

    let homer = Person { first_name: "Homer".into(), last_name: simpson.into() };
    let marge = Person { first_name: "Marge".into(), last_name: simpson.into() };
    let bart = Person { first_name: "Bart".into(), last_name: simpson.into() };
    let lisa = Person { first_name: "Lisa".into(), last_name: simpson.into() };
    let maggie = Person { first_name: "Maggie".into(), last_name: simpson.into() };

    let homer_id = family.add(homer);
    let marge_id = family.add(marge);
    let bart_id = family.add(bart);
    let lisa_id = family.add(lisa);
    let maggie_id = family.add(maggie);
    for &parent in &[homer_id, marge_id] {
        for &child in &[bart_id, lisa_id, maggie_id] {
            family.add(Parent { parent_id: parent, child_id: child });
        }
    }

    {
        let persons: &[Person] = family.get_table();
        let parents: &[Parent] = family.get_table();
        for p in persons {
            println!("{} {}", p.first_name, p.last_name);
        }
        for p in parents {
            let parent = &persons[p.parent_id];
            let child = &persons[p.child_id];
            println!("{} is parent of {}", parent.first_name, child.first_name);
        }
    }

    // foo(&mut family);
    bar(&family);
}
