import { Knex } from 'knex';

export interface OauthProfile {
  sub: string;
  userId: string;
  profileJson: string;
}

const insert = (db: Knex) => (oauthProfile: OauthProfile) =>
  db
    .insert({ ...oauthProfile, createdAt: new Date() })
    .into('oauthProfile')
    .returning('*')
    .then((r) => r[0]);

const upsert = (db: Knex) => (oauthProfile: OauthProfile) =>
  db
    .insert({ ...oauthProfile, createdAt: new Date() })
    .into('oauthProfile')
    .onConflict('sub')
    .merge({ ...oauthProfile, updatedAt: new Date() })
    .returning('*')
    .then((r) => r[0]);

const getBySub =
  (db: Knex) =>
  (sub: string): Promise<OauthProfile> =>
    db
      .select()
      .from('oauthProfile')
      .where('sub', sub)
      .then((r) => (r.length ? r[0] : null));

const getByEmail =
  (db: Knex) =>
  (email: string): Promise<OauthProfile> =>
    db
      .select()
      .from('oauthProfile')
      .where('email', email)
      .then((r) => (r.length ? r[0] : null));

const getAll = (db: Knex) => () =>
  db.select().from('oauthProfile').orderBy('id', 'asc');

const del = (db: Knex) => (id: string) => db.del().where('id', id);

export default (db: Knex) => ({
  insert: insert(db),
  upsert: upsert(db),
  getBySub: getBySub(db),
  getByEmail: getByEmail(db),
  getAll: getAll(db),
  del: del(db),
});
