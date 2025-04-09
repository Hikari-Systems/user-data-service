import { Knex } from 'knex';

export interface UserTermsAcceptance {
  id: string;
  userId: string;
  termsVersion: string;
  acceptedAt: Date;
  createdAt?: Date;
  updatedAt?: Date;
}

const insert = (db: Knex) => (acceptance: UserTermsAcceptance) =>
  db
    .insert({ ...acceptance, createdAt: new Date() })
    .into('userTermsAcceptance')
    .returning('*')
    .then((r) => r[0]);

const get =
  (db: Knex) =>
  (id: string): Promise<UserTermsAcceptance> =>
    db
      .select()
      .from('userTermsAcceptance')
      .where('id', id)
      .then((r) => (r.length ? r[0] : null));

const getByUserId =
  (db: Knex) =>
  (userId: string): Promise<UserTermsAcceptance[]> =>
    db
      .select()
      .from('userTermsAcceptance')
      .where('userId', userId)
      .orderBy('acceptedAt', 'desc');

const getLatestByUserId =
  (db: Knex) =>
  (userId: string): Promise<UserTermsAcceptance> =>
    db
      .select()
      .from('userTermsAcceptance')
      .where('userId', userId)
      .orderBy('acceptedAt', 'desc')
      .limit(1)
      .then((r) => (r.length ? r[0] : null));

const getAll = (db: Knex) => () =>
  db.select().from('userTermsAcceptance').orderBy('acceptedAt', 'desc');

const del = (db: Knex) => (id: string) => db.del().where('id', id);

export default (db: Knex) => ({
  insert: insert(db),
  get: get(db),
  getByUserId: getByUserId(db),
  getLatestByUserId: getLatestByUserId(db),
  getAll: getAll(db),
  del: del(db),
});
