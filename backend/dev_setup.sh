#!/usr/bin/env bash
sqlite3 db.sqlite 'CREATE TABLE IF NOT EXISTS metadorerna_items(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    location TEXT NOT NULL,
    min REAL,
    max REAL,
    current REAL NOT NULL,
    supplier INTEGER,
    updated INTEGER NOT NULL,
    link TEXT
    );' \
    'CREATE TABLE IF NOT EXISTS metadorerna_suppliers(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    link TEXT,
    notes TEXT,
    username TEXT,
    password TEXT
    );' \
    'CREATE TABLE IF NOT EXISTS metadorerna_log(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    amount REAL NOT NULL,
    time INTEGER NOT NULL
    )'
