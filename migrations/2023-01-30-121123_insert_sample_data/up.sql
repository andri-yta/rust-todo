INSERT INTO accounts(email) 
VALUES 
('john@sample.com'),
('smith@sample.com');

INSERT INTO todos(account_id, created_at, title, status)
VALUES (1, '2023-01-30', 'Buy grocery', 'Pending');