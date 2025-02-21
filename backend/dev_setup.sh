#!/usr/bin/env bash
psql -h localhost -p 5432 -U postgres -c "CREATE DATABASE db ENCODING 'UTF8' OWNER postgres;"
psql -h localhost -p 5432 -U postgres -d db -c 'CREATE TABLE IF NOT EXISTS items(
    id SERIAL PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    location TEXT NOT NULL,
    min REAL,
    max REAL,
    current REAL NOT NULL,
    supplier INTEGER,
    updated INTEGER NOT NULL,
    link TEXT,
    club TEXT NOT NULL
    );' -c 'CREATE TABLE IF NOT EXISTS suppliers(
    id SERIAL PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    link TEXT,
    notes TEXT,
    username TEXT,
    password TEXT,
    updated INTEGER NOT NULL,
    club TEXT NOT NULL
    );' -c 'CREATE TABLE IF NOT EXISTS log(
    id SERIAL PRIMARY KEY NOT NULL,
    item_id INTEGER NOT NULL,
    amount REAL NOT NULL,
    time INTEGER NOT NULL,
    club TEXT NOT NULL
    );'
