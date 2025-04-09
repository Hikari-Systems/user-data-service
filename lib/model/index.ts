import { knex, Knex } from 'knex';
import knexFile from '../knexfile';
import user from './user';
import oauthProfile from './oauth_profile';
import accessRequest from './access_request';
import userTermsAcceptance from './user_terms_acceptance';

const db: Knex = knex(knexFile.main);

export const healthcheck = () => db.select().from('knex_migrations').limit(1);

export const shutdown = () => db.destroy();

export const userModel = user(db);
export const oauthProfileModel = oauthProfile(db);
export const accessRequestModel = accessRequest(db);
export const userTermsAcceptanceModel = userTermsAcceptance(db);
