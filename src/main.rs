extern crate connectors;
extern crate controllers;
extern crate data_access;
extern crate dtos;
extern crate interfaces;
extern crate migrations;
extern crate models;
extern crate services;
extern crate settings;

fn main() {
    models::hello();
    interfaces::hello();
    controllers::hello();
    dtos::hello();
    services::hello();
    connectors::hello();
    settings::hello();
    data_access::hello();
    migrations::hello();
}
