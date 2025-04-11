import { Knex } from 'knex';

export const up = (knex: Knex) =>
  knex.schema.table('user', (t: Knex.TableBuilder) => {
    t.uuid('pictureImageServiceId').nullable();
  });

export const down = (knex: Knex) =>
  knex.schema.table('user', (t: Knex.TableBuilder) => {
    t.dropColumn('pictureImageServiceId');
  });
