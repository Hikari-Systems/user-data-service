import { Knex } from 'knex';

export const up = (knex: Knex) =>
  knex.schema
    .createTable('user', (t) => {
      t.uuid('id').primary().notNullable();
      t.string('email', 400).notNullable();
      t.timestamps();
    })
    .createTable('oauthProfile', (t) => {
      t.string('sub').primary().notNullable();
      t.uuid('userId').notNullable().references('id').inTable('user');
      t.jsonb('profileJson').notNullable();
      t.timestamps();
    })
    .createTable('site', (t) => {
      t.uuid('id').primary().notNullable();
      t.uuid('userId').notNullable().references('id').inTable('user');
      t.integer('mapN').notNullable();
      t.integer('mapX').notNullable();
      t.integer('mapY').notNullable();
      t.timestamps();
    })
    .createTable('siteImage', (t) => {
      t.uuid('id').primary().notNullable();
      t.uuid('siteId').notNullable().references('id').inTable('site');
      t.timestamps();
    })
    .createTable('siteTreatment', (t) => {
      t.uuid('id').primary().notNullable();
      t.uuid('siteId').notNullable().references('id').inTable('site');
      t.timestamps();
    });

export const down = (knex: Knex) =>
  knex.schema
    .dropTable('siteTreatment')
    .dropTable('siteImage')
    .dropTable('site')
    .dropTable('user');
