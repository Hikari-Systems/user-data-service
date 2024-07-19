import { Knex } from 'knex';

export const up = (knex: Knex) =>
  knex.schema
    .createTable('user', (t) => {
      t.uuid('id').primary().notNullable();
      t.string('email', 400).notNullable();
      t.string('name', 400);
      t.string('picture', 1000);
      t.timestamps();
    })
    .createTable('oauthProfile', (t) => {
      t.string('sub').primary().notNullable();
      t.uuid('userId').notNullable().references('id').inTable('user');
      t.jsonb('profileJson').notNullable();
      t.timestamps();
    });

export const down = (knex: Knex) =>
  knex.schema.dropTable('oauthProfile').dropTable('user');
