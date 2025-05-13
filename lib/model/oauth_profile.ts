import { Knex } from 'knex';

export interface OauthProfile {
  sub: string;
  userId: string;
  profileJson: Record<string, any>;
}

const insert = (db: Knex) => (oauthProfile: OauthProfile) =>
  db
    .insert({
      ...oauthProfile,
      profileJson: JSON.stringify(oauthProfile.profileJson),
      createdAt: new Date(),
    })
    .into('oauthProfile')
    .returning('*')
    .then((r) => r[0]);

const upsert = (db: Knex) => (oauthProfile: OauthProfile) =>
  db
    .insert({
      ...oauthProfile,
      profileJson: JSON.stringify(oauthProfile.profileJson),
      createdAt: new Date(),
    })
    .into('oauthProfile')
    .onConflict('sub')
    .merge({
      ...oauthProfile,
      profileJson: JSON.stringify(oauthProfile.profileJson),
      updatedAt: new Date(),
    })
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

const getByUserId =
  (db: Knex) =>
  (userId: string): Promise<OauthProfile> =>
    db
      .select()
      .from('oauthProfile')
      .where('userId', userId)
      .then((r) => (r.length ? r[0] : null));

const getAll = (db: Knex) => () =>
  db.select().from('oauthProfile').orderBy('id', 'asc');

const del = (db: Knex) => (id: string) => db.del().where('id', id);

export default (db: Knex) => ({
  insert: insert(db),
  upsert: upsert(db),
  getBySub: getBySub(db),
  getByUserId: getByUserId(db),
  getAll: getAll(db),
  del: del(db),
});
