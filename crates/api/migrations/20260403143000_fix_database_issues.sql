-- Fix DECIMCAL precision to prevent overflow for large datasets
ALTER TABLE upload_summaries
    ALTER COLUMN cost_actual TYPE DECIMAL(15, 2),
    ALTER COLUMN cost_optimized TYPE DECIMAL(15, 2),
    ALTER COLUMN savings TYPE DECIMAL(15, 2);

-- Add constraints to prevent negative financial and trip counts
ALTER TABLE upload_summaries
    ADD CONSTRAINT chk_cost_actual_positive CHECK (cost_actual >= 0),
    ADD CONSTRAINT chk_cost_optimized_positive CHECK (cost_optimized >= 0),
    ADD CONSTRAINT chk_total_trips_positive CHECK (total_trips >= 0);
