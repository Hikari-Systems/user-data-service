import { Knex } from 'knex';

export const up = (knex: Knex) =>
  knex.schema.createTable('userTermsAcceptance', (t: Knex.TableBuilder) => {
    t.uuid('id').primary();
    t.uuid('userId').notNullable();
    t.uuid('termsVersion').notNullable();
    t.timestamp('acceptedAt').notNullable();
    t.timestamps();
  });

export const down = (knex: Knex) =>
  knex.schema.dropTable('userTermsAcceptance');
