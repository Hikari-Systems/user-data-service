import express from 'express';
import { v4 } from 'uuid';
import { logging } from '@hikari-systems/hs.utils';

import { userModel } from '../model';

const log = logging('routes:user');

const router = express.Router();
// const jsonParser = express.json();

router.get('/byEmail', async (req, res, next) => {
  const { email } = req.query as { email: string };
  if (!email) {
    log.debug(`No email provided`);
    return res.status(400).send(`No email provided`);
  }
  try {
    const user = await userModel.getByEmail(email);
    if (!user) {
      log.debug(`No user found for email ${email}`);
      return res.status(204).send(`No user found for email ${email}`);
    }
    return res.status(200).json(user);
  } catch (e) {
    log.error(`Error fetching user for email ${email}`, e);
    return next(e);
  }
});

router.get('/:id', async (req, res, next) => {
  const id = req.params.id as string;
  if (!id) {
    log.debug(`No id provided`);
    return res.status(400).send(`No id provided`);
  }
  try {
    const user = await userModel.get(id);
    if (!user) {
      log.debug(`No user found for id ${id}`);
      return res.status(204).send(`No user found for id ${id}`);
    }
    return res.status(200).json(user);
  } catch (e) {
    log.error(`Error fetching user for id ${id}`, e);
    return next(e);
  }
});

router.post('/', express.json(), async (req, res, next) => {
  const { email } = req.body as { email: string };
  try {
    const user = await userModel.insert({
      id: v4(),
      email,
    });
    return res.status(201).json(user);
  } catch (e) {
    log.error(`Error adding user for ${JSON.stringify(req.body)}`, e);
    return next(e);
  }
});

router.put('/:id', express.json(), async (req, res, next) => {
  const { id } = req.params;
  const { email } = req.body as { email: string };
  try {
    const user = await userModel.upsert({ id, email });
    return res.status(200).json(user);
  } catch (e) {
    log.error(
      `Error updating user for id=${id} ${JSON.stringify(req.body)}`,
      e,
    );
    return next(e);
  }
});

router.delete('/:id', express.json(), async (req, res, next) => {
  const id = req.params.id as string;
  if (!id) {
    log.debug(`No id provided`);
    return res.status(400).send(`No id provided`);
  }
  try {
    const user = await userModel.get(id);
    if (!user) {
      log.debug(`No user found for id ${id}`);
      return res.status(204).send(`No user found for id ${id}`);
    }
    log.debug(`Deleting user ${id} ...`);
    await userModel.del(id);
    return res.status(203).end();
  } catch (e) {
    log.error(`Error deleting user for id ${id}`, e);
    return next(e);
  }
});

export default router;
