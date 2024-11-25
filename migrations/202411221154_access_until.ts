import { Knex } from 'knex';

export const up = (knex: Knex) =>
  knex.schema.table('accessRequest', (t) => {
    t.timestamp('grantedFrom');
    t.timestamp('grantedUntil');
    t.renameColumn('decisionTimestamp', 'decidedAt');
  });

export const down = (knex: Knex) =>
  knex.schema.table('accessRequest', (t) => {
    t.dropColumn('grantedFrom');
    t.dropColumn('grantedUntil');
    t.renameColumn('decidedAt', 'decisionTimestamp');
  });
