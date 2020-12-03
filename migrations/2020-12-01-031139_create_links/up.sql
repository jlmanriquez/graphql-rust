-- Your SQL goes here
CREATE TABLE links (
  id serial PRIMARY KEY,
  title VARCHAR(255),
  address VARCHAR(255),
  userid INT,
  CONSTRAINT fk_users
    FOREIGN KEY(userid) 
	    REFERENCES users(id)
);