use models::model::Model;

extern crate connectors;
extern crate parsers;
extern crate controllers;
extern crate data_access;
extern crate dtos;
extern crate models;
extern crate services;

fn main() {
    let container_meta = models::container_meta::ContainerMeta::new();
    // controllers::hello();
    // dtos::hello();
    // services::hello();
    // connectors::hello();
    // data_access::hello();
}
