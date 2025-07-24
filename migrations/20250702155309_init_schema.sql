-- Enums
CREATE TYPE engine_type AS ENUM ('Gasoline', 'Diesel', 'Electric');
CREATE TYPE maintenance_interval_type AS ENUM ('Kilometers', 'EngineHours', 'Years');

-- Users
CREATE TABLE users (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    -- role UserRole NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Vehicles
CREATE TABLE vehicles (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    make TEXT NOT NULL,
    model TEXT NOT NULL,
    year SMALLINT NOT NULL,
    vin TEXT NOT NULL UNIQUE,
    license_plate TEXT NOT NULL UNIQUE,
    engine_type engine_type NOT NULL,

    -- Timestamps and user tracking
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(uuid) ON DELETE SET NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_by UUID NOT NULL REFERENCES users(uuid) ON DELETE SET NULL
);

-- Vehicle Statuses (Historical & Current)
CREATE TABLE vehicle_statuses (
    id SERIAL PRIMARY KEY,
    vehicle_id UUID NOT NULL REFERENCES vehicles(uuid) ON DELETE CASCADE,
    performed_by UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    odometer INTEGER,
    engine_hour_meter INTEGER,
    fuel_level INTEGER,
    notes TEXT,
    latest BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enforce at most one 'latest' status per vehicle
CREATE UNIQUE INDEX one_latest_status_per_vehicle
ON vehicle_statuses(vehicle_id)
WHERE latest = true;

-- Maintenance Types (global definitions like "Oil Change", "Brake Check")
CREATE TABLE maintenance_types (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,

    -- Timestamps and user tracking
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(uuid) ON DELETE SET NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_by UUID NOT NULL REFERENCES users(uuid) ON DELETE SET NULL
);

-- Maintenances (custom rules per vehicle & type)
CREATE TABLE maintenances (
    id SERIAL PRIMARY KEY,
    vehicle_id UUID NOT NULL REFERENCES vehicles(uuid) ON DELETE CASCADE,
    maintenance_type_id INTEGER NOT NULL REFERENCES maintenance_types(id) ON DELETE CASCADE,
    interval_type maintenance_interval_type NOT NULL,
    interval_value INTEGER NOT NULL,
    red_threshold INTEGER NOT NULL, -- in percentage
    yellow_threshold INTEGER NOT NULL, -- in percentage
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Ensure only one rule per (vehicle, maintenance type)
    UNIQUE(vehicle_id, maintenance_type_id)
);

-- Maintenance Records (what was performed, when)
CREATE TABLE maintenance_records (
    id SERIAL PRIMARY KEY,
    -- Relationships
    vehicle_id UUID NOT NULL REFERENCES vehicles(uuid) ON DELETE CASCADE,
    maintenance_id INTEGER REFERENCES maintenances(id) ON DELETE CASCADE,
    performed_by UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    vehicle_status_id INTEGER NOT NULL REFERENCES vehicle_statuses(id) ON DELETE CASCADE,
    -- Record details
    performed_at TIMESTAMPTZ NOT NULL,
    details TEXT NOT NULL,
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_by UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE
);
