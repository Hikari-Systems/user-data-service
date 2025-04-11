import express from 'express';
import { logging } from '@hikari-systems/hs.utils';

import { oauthProfileModel } from '../model';

const log = logging('routes:oauthProfile');

const router = express.Router();
// const jsonParser = express.json();

router.get('/byUserId/:userId', async (req, res, next) => {
  const { userId } = req.params as { userId: string };
  if (!userId) {
    log.debug(`No userId provided`);
    return res.status(400).send(`No userId provided`);
  }
  try {
    const oauthProfile = await oauthProfileModel.getByUserId(userId);
    if (!oauthProfile) {
      log.debug(`No oauthProfile found for userId ${userId}`);
      return res.status(204).send(`No oauthProfile found for userId ${userId}`);
    }
    return res.status(200).json(oauthProfile);
  } catch (e) {
    log.error(`Error fetching oauthProfile for userId ${userId}`, e);
    return next(e);
  }
});

router.get('/bySub', async (req, res, next) => {
  const { sub } = req.query as { sub: string };
  if (!sub) {
    log.debug(`No id provided`);
    return res.status(400).send(`No id provided`);
  }
  try {
    const oauthProfile = await oauthProfileModel.getBySub(sub);
    if (!oauthProfile) {
      log.debug(`No oauthProfile found for sub ${sub}`);
      return res.status(204).send(`No oauthProfile found for sub ${sub}`);
    }
    return res.status(200).json(oauthProfile);
  } catch (e) {
    log.error(`Error fetching oauthProfile for sub ${sub}`, e);
    return next(e);
  }
});

router.put('/', express.json(), async (req, res, next) => {
  const { sub, userId, profileJson } = req.body as {
    sub: string;
    userId: string;
    profileJson: string;
  };
  try {
    const oauthProfile = await oauthProfileModel.upsert({
      sub,
      userId,
      profileJson: JSON.parse(profileJson),
    });
    return res.status(200).json(oauthProfile);
  } catch (e) {
    log.error(
      `Error updating oauthProfile for sub=${sub} ${JSON.stringify(req.body)}`,
      e,
    );
    return next(e);
  }
});

router.delete('/', express.json(), async (req, res, next) => {
  const { sub } = req.query as { sub: string };
  if (!sub) {
    log.debug(`No sub provided`);
    return res.status(400).send(`No sub provided`);
  }
  try {
    const oauthProfile = await oauthProfileModel.getBySub(sub);
    if (!oauthProfile) {
      log.debug(`No oauthProfile found for sub ${sub}`);
      return res.status(204).send(`No oauthProfile found for sub ${sub}`);
    }
    log.debug(`Deleting oauthProfile ${sub} ...`);
    await oauthProfileModel.del(sub);
    return res.status(203).end();
  } catch (e) {
    log.error(`Error deleting oauthProfile for sub ${sub}`, e);
    return next(e);
  }
});

export default router;
