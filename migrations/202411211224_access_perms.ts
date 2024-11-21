import { Knex } from 'knex';

export const up = (knex: Knex) =>
  knex.schema.createTable('accessRequest', (t) => {
    t.uuid('id').primary().notNullable();
    t.uuid('userId').notNullable().references('id').inTable('user');
    t.string('key', 400).notNullable();
    t.timestamp('decisionTimestamp');
    t.boolean('granted');
    t.timestamps();
  });

export const down = (knex: Knex) => knex.schema.dropTable('accessRequest');
