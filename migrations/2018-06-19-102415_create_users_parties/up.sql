CREATE TABLE users_parties (
  id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
  user_id INTEGER NOT NULL,
  party_id INTEGER NOT NULL
);

ALTER TABLE users_parties
  ADD CONSTRAINT user_id FOREIGN KEY (user_id) REFERENCES users (id),
  ADD CONSTRAINT party_id FOREIGN KEY (party_id) REFERENCES parties (id);
COMMIT;