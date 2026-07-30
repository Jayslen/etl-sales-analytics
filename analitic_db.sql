CREATE TABLE fact_sales (
    sales_id SERIAL PRIMARY KEY,

    order_id INT,
    product_id INT,
    customer_id INT,

    date_id INT,
    city_id INT,
    country_id INT,
    category_id INT,
    status_id INT,

    quantity INT,
    unit_price DOUBLE PRECISION,
    total_amount DOUBLE PRECISION
);

CREATE TABLE dim_date (
    date_id INT PRIMARY KEY,
    full_date DATE,
    day INT,
    month INT,
    year INT,
    quarter INT,
    month_name VARCHAR(20),
    day_name VARCHAR(20)
);

CREATE TABLE dim_customer (
    customer_id INT PRIMARY KEY,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    city_id INT,
    country_id INT
);

CREATE TABLE dim_location (
    city_id INT PRIMARY KEY,
    city_name VARCHAR(100),
    country_id INT,
    country_name VARCHAR(100)
);

CREATE TABLE dim_product (
    product_id INT PRIMARY KEY,
    product_name VARCHAR(100),
    category_id INT,
    category_name VARCHAR(100)
);

CREATE TABLE dim_category (
    category_id INT PRIMARY KEY,
    category_name VARCHAR(100)
);

CREATE TABLE dim_status (
    status_id INT PRIMARY KEY,
    status_name VARCHAR(50)
);

INSERT INTO fact_sales (
    order_id,
    product_id,
    customer_id,
    date_id,
    city_id,
    country_id,
    category_id,
    status_id,
    quantity,
    unit_price,
    total_amount
)
SELECT
    o.order_id,
    od.product_id,
    o.customer_id,
    TO_CHAR(o.order_date, 'YYYYMMDD')::INT,
    c.city_id,
    ci.country_id,
    p.category_id,
    o.status_id,
    od.quantity,
    p.price,
    od.total
FROM order_details od
JOIN orders o ON o.order_id = od.order_id
JOIN customers c ON c.customer_id = o.customer_id
JOIN cities ci ON ci.city_id = c.city_id
JOIN products p ON p.product_id = od.product_id;

SELECT * FROM fac