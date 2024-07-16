import { knex, Knex } from 'knex';
import knexFile from '../knexfile';
import user from './user';
import oauthProfile from './oauth_profile';

const db: Knex = knex(knexFile.main);

export const healthcheck = () => db.select().from('knex_migrations').limit(1);

export const shutdown = () => db.destroy();

export const userModel = user(db);
export const oauthProfileModel = oauthProfile(db);
