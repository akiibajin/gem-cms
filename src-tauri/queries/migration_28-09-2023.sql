CREATE TYPE "weight_units" AS ENUM (
  'ct',
  'gr',
  'kg'
);

CREATE TYPE "dim_units" AS ENUM (
  'mm',
  'cm',
  'mt'
);

CREATE TYPE "colors" AS ENUM (
  'Yellow',
  'Blue',
  'Colourless',
  'Orange',
  'Black',
  'Purple',
  'Red',
  'Pink',
  'Green',
  'Violet'
);

CREATE TYPE "cuts" AS ENUM (
  'Briollet',
  'Rough',
  'Heart',
  'Cushion',
  'Sculpture',
  'Emerald',
  'Emscuadrada',
  'Fancy',
  'Pear',
  'Marquise',
  'Oval',
  'Princess',
  'Rectangular',
  'Round',
  'Sugar Loaf',
  'Trillion'
);

CREATE TYPE "languages" AS ENUM (
  'Español',
  'Chino',
  'Francés',
  'Italiano'
);

CREATE TYPE "services" AS ENUM (
  'A',
  'B',
  'C',
  'AV',
  'BV',
  'CV',
  'T',
  'Copia'
);

CREATE TABLE "client" (
  "doc" INTEGER PRIMARY KEY,
  "name" VARCHAR(100),
  "mobile" VARCHAR(15),
  "landline" VARCHAR(10),
  "address" VARCHAR(50),
  "email" VARCHAR(50),
  "discount" SMALLINT
);

CREATE TABLE "company_info" (
  "mobile" VARCHAR(10),
  "landline" VARCHAR(10),
  "address" VARCHAR(50),
  "email" VARCHAR(50)
);

CREATE TABLE "receipts" (
  "id" SERIAL PRIMARY KEY,
  "client_doc" INTEGER,
  "entry_date" TIMESTAMP,
  "delivery_date" DATE,
  "observations" VARCHAR(200)
);

CREATE TABLE "items" (
  "id" SERIAL PRIMARY KEY,
  "jewel" CHAR(8),
  "receipt_id" INTEGER,
  "weight" REAL,
  "weight_unit" weight_units,
  "length" REAL,
  "width" REAL,
  "height" REAL,
  "dim_unit" dim_units,
  "color" colors,
  "cut" cuts,
  "language" languages,
  "service" services,
  "value" REAL
);

-- Add foreign keys
ALTER TABLE "receipts" ADD FOREIGN KEY ("client_doc") REFERENCES "client" ("doc");
ALTER TABLE "items" ADD FOREIGN KEY ("receipt_id") REFERENCES "receipts" ("id");

--Enter data to the table "company_info"
INSERT INTO "company_info"
VALUES('3106898392','3344289','Emerald Trade Center Of. 1204');
