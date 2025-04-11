import express from 'express';
import { v4 } from 'uuid';
import { logging } from '@hikari-systems/hs.utils';

import { userTermsAcceptanceModel } from '../model';

const log = logging('routes:userTermsAcceptance');

const router = express.Router();

router.get('/:id', async (req, res, next) => {
  const id = req.params.id as string;
  if (!id) {
    log.debug(`No id provided`);
    return res.status(400).send(`No id provided`);
  }
  try {
    const acceptance = await userTermsAcceptanceModel.get(id);
    if (!acceptance) {
      log.debug(`No terms acceptance found for id ${id}`);
      return res.status(204).send(`No terms acceptance found for id ${id}`);
    }
    return res.status(200).json(acceptance);
  } catch (e) {
    log.error(`Error fetching terms acceptance for id ${id}`, e);
    return next(e);
  }
});

router.get('/byUserId/:userId', async (req, res, next) => {
  const userId = req.params.userId as string;
  if (!userId) {
    log.debug(`No userId provided`);
    return res.status(400).send(`No userId provided`);
  }
  try {
    const acceptances = await userTermsAcceptanceModel.getByUserId(userId);
    return res.status(200).json(acceptances);
  } catch (e) {
    log.error(`Error fetching terms acceptances for userId ${userId}`, e);
    return next(e);
  }
});

router.get('/byUserId/:userId/latest', async (req, res, next) => {
  const userId = req.params.userId as string;
  if (!userId) {
    log.debug(`No userId provided`);
    return res.status(400).send(`No userId provided`);
  }
  try {
    const acceptance = await userTermsAcceptanceModel.getLatestByUserId(userId);
    if (!acceptance) {
      log.debug(`No terms acceptance found for userId ${userId}`);
      return res
        .status(204)
        .send(`No terms acceptance found for userId ${userId}`);
    }
    return res.status(200).json(acceptance);
  } catch (e) {
    log.error(`Error fetching latest terms acceptance for userId ${userId}`, e);
    return next(e);
  }
});

router.post('/', express.json(), async (req, res, next) => {
  const { userId, termsVersion } = req.body as {
    userId: string;
    termsVersion: string;
  };
  if (!userId || !termsVersion) {
    log.debug(`Missing required fields: userId or termsVersion`);
    return res
      .status(400)
      .send(`Missing required fields: userId or termsVersion`);
  }
  try {
    const acceptance = await userTermsAcceptanceModel.insert({
      id: v4(),
      userId,
      termsVersion,
      acceptedAt: new Date(),
    });
    return res.status(201).json(acceptance);
  } catch (e) {
    log.error(
      `Error adding terms acceptance for ${JSON.stringify(req.body)}`,
      e,
    );
    return next(e);
  }
});

router.delete('/:id', async (req, res, next) => {
  const id = req.params.id as string;
  if (!id) {
    log.debug(`No id provided`);
    return res.status(400).send(`No id provided`);
  }
  try {
    const acceptance = await userTermsAcceptanceModel.get(id);
    if (!acceptance) {
      log.debug(`No terms acceptance found for id ${id}`);
      return res.status(204).send(`No terms acceptance found for id ${id}`);
    }
    log.debug(`Deleting terms acceptance ${id} ...`);
    await userTermsAcceptanceModel.del(id);
    return res.status(203).end();
  } catch (e) {
    log.error(`Error deleting terms acceptance for id ${id}`, e);
    return next(e);
  }
});

export default router;
