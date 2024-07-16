import { Knex } from 'knex';

export interface User {
  id: string;
  email: string;
}

const insert = (db: Knex) => (user: User) =>
  db
    .insert({ ...user, createdAt: new Date() })
    .into('user')
    .returning('*')
    .then((r) => r[0]);

const upsert = (db: Knex) => (user: User) =>
  db
    .insert({ ...user, createdAt: new Date() })
    .into('user')
    .onConflict('id')
    .merge({ ...user, updatedAt: new Date() })
    .returning('*')
    .then((r) => r[0]);

const get =
  (db: Knex) =>
  (id: string): Promise<User> =>
    db
      .select()
      .from('user')
      .where('id', id)
      .then((r) => (r.length ? r[0] : null));

const getByEmail =
  (db: Knex) =>
  (email: string): Promise<User> =>
    db
      .select()
      .from('user')
      .where('email', email)
      .then((r) => (r.length ? r[0] : null));

const getAll = (db: Knex) => () =>
  db.select().from('user').orderBy('id', 'asc');

const del = (db: Knex) => (id: string) => db.del().where('id', id);

export default (db: Knex) => ({
  insert: insert(db),
  upsert: upsert(db),
  get: get(db),
  getByEmail: getByEmail(db),
  getAll: getAll(db),
  del: del(db),
});
