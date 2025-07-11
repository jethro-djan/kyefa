-- migrations/002_add_payout_entities.sql

-- ===============================
-- Core Tables
-- ===============================

CREATE TABLE support_staff (
    id UUID PRIMARY KEY,
    first_name TEXT NOT NULL,
    surname TEXT NOT NULL,
    other_names TEXT,
    role TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true
);

CREATE TABLE payments (
    id UUID PRIMARY KEY,
    student_id UUID NOT NULL REFERENCES students(id),
    term_id UUID NOT NULL REFERENCES terms(id),
    amount_paid DOUBLE PRECISION NOT NULL,
    date_paid DATE NOT NULL,
    recorded_by UUID NOT NULL REFERENCES users(id)
);

CREATE TABLE constraint_configs (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    base_percentage DOUBLE PRECISION NOT NULL,
    admin_percentage DOUBLE PRECISION NOT NULL,
    support_staff_percentage DOUBLE PRECISION NOT NULL,
    max_periods_paid INTEGER,
    max_ratio DOUBLE PRECISION,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE disbursements (
    id UUID PRIMARY KEY,
    term_id UUID NOT NULL REFERENCES terms(id),
    total_revenue DOUBLE PRECISION NOT NULL,
    admin_share DOUBLE PRECISION NOT NULL,
    support_staff_share DOUBLE PRECISION NOT NULL,
    teacher_base_share DOUBLE PRECISION NOT NULL,
    teacher_period_share DOUBLE PRECISION NOT NULL,
    constraint_id UUID NOT NULL REFERENCES constraint_configs(id),
    calculated_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE teacher_payouts (
    id UUID PRIMARY KEY,
    disbursement_id UUID NOT NULL REFERENCES disbursements(id),
    teacher_id UUID NOT NULL REFERENCES teachers(id),
    base_share DOUBLE PRECISION NOT NULL,
    period_share DOUBLE PRECISION NOT NULL,
    capped BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE teacher_payout_items (
    id UUID PRIMARY KEY,
    payout_id UUID NOT NULL REFERENCES teacher_payouts(id),
    conducted_period_id UUID NOT NULL REFERENCES conducted_periods(id),
    paid_amount DOUBLE PRECISION NOT NULL
);
