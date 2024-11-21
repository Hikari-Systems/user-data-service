import { Knex } from 'knex';

export interface AccessRequest {
  id: string;
  userId: string;
  key: string;
  granted?: boolean;
  decisionTimestamp?: Date;
}

const insert = (db: Knex) => (accessRequest: AccessRequest) =>
  db
    .insert({ ...accessRequest, createdAt: new Date() })
    .into('accessRequest')
    .returning('*')
    .then((r) => r[0]);

const upsert = (db: Knex) => (accessRequest: AccessRequest) =>
  db
    .insert({ ...accessRequest, createdAt: new Date() })
    .into('accessRequest')
    .onConflict('id')
    .merge({ ...accessRequest, updatedAt: new Date() })
    .returning('*')
    .then((r) => r[0]);

const get =
  (db: Knex) =>
  (id: string): Promise<AccessRequest> =>
    db
      .select()
      .from('accessRequest')
      .where('id', id)
      .limit(1)
      .then((r) => (r.length ? r[0] : null));

const getByUserId =
  (db: Knex) =>
  (userId: string): Promise<AccessRequest[]> =>
    db
      .select()
      .from('accessRequest')
      .where('userId', userId)
      .orderBy('createdAt', 'asc');

const getByUserIdAndKey =
  (db: Knex) =>
  (userId: string, key: string): Promise<AccessRequest> =>
    db
      .select()
      .from('accessRequest')
      .where({ userId, key })
      .limit(1)
      .then((r) => (r.length ? r[0] : null));

const getAll = (db: Knex) => () =>
  db.select().from('accessRequest').orderBy('createdAt', 'asc');

const del = (db: Knex) => (id: string) => db.del().where('id', id);

export default (db: Knex) => ({
  insert: insert(db),
  upsert: upsert(db),
  get: get(db),
  getByUserId: getByUserId(db),
  getByUserIdAndKey: getByUserIdAndKey(db),
  getAll: getAll(db),
  del: del(db),
});
