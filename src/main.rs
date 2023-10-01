extern crate models;
extern crate interfaces;
extern crate controllers;
extern crate dtos;
extern crate services;
extern crate connectors;
extern crate settings;
extern crate data_access;
extern crate migrations;

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