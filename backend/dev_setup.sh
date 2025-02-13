#!/usr/bin/env bash
sqlite3 db.sqlite 'CREATE TABLE IF NOT EXISTS items(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    location TEXT NOT NULL,
    min REAL,
    max REAL,
    current REAL NOT NULL,
    supplier INTEGER,
    updated INTEGER NOT NULL,
    link TEXT,
    club TEXT NOT NULL
    );' \
    'CREATE TABLE IF NOT EXISTS suppliers(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    link TEXT,
    notes TEXT,
    username TEXT,
    password TEXT,
    updated INTEGER NOT NULL,
    club TEXT NOT NULL
    );' \
    'CREATE TABLE IF NOT EXISTS log(
    id INTEGER PRIMARY KEY NOT NULL,
    item_id INTEGER NOT NULL,
    amount REAL NOT NULL,
    time INTEGER NOT NULL,
    club TEXT NOT NULL
    )'
