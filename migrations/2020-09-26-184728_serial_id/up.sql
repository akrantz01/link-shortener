/* Create a sequence for the ID */
CREATE SEQUENCE links_id_seq;
ALTER SEQUENCE links_id_seq OWNED BY links.id;

/* Set the starting value to be 1 more than the greatest ID */
SELECT setval('links_id_seq', coalesce(max(id), 0) + 1, false) FROM links;

/* Use the sequence as the default for the ID column */
ALTER TABLE links ALTER COLUMN id SET DEFAULT nextval('links_id_seq');
