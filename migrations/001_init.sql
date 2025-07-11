-- migrations/001_init.sql

-- ===============================
-- Core Tables
-- ===============================

CREATE TABLE terms (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    academic_year TEXT NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE students (
    id UUID PRIMARY KEY,
    first_name TEXT NOT NULL,
    surname TEXT NOT NULL,
    other_names TEXT,
    gender TEXT NOT NULL,
    class_level TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE teachers (
    id UUID PRIMARY KEY,
    first_name TEXT NOT NULL,
    surname TEXT NOT NULL,
    other_names TEXT,
    status TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE subjects (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    stream TEXT NOT NULL
);

CREATE TABLE conducted_periods (
    id UUID PRIMARY KEY,
    date DATE NOT NULL,
    class_level TEXT NOT NULL,
    subject_id UUID NOT NULL REFERENCES subjects(id),
    teacher_id UUID NOT NULL REFERENCES teachers(id),
    term_id UUID NOT NULL REFERENCES terms(id),
    week_of_term INTEGER NOT NULL,
    was_conducted BOOLEAN NOT NULL DEFAULT TRUE,
    notes TEXT
);

-- ===============================
-- User Accounts (System-level)
-- ===============================

CREATE TABLE users (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE
);

